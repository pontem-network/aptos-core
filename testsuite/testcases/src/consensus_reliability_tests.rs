// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{LoadDestination, NetworkLoadTest};
use anyhow::{anyhow, bail, Context};
use forge::test_utils::consensus_utils::{
    test_consensus_fault_tolerance, FailPointFailureInjection, NodeState,
};
use forge::{NetworkContext, NetworkTest, Result, Swarm, SwarmExt, Test};
use pont_logger::{info, warn};
use rand::Rng;
use std::collections::HashSet;
use std::time::Duration;
use tokio::runtime::Runtime;

pub struct ChangingWorkingQuorumTest {
    pub min_tps: usize,
    pub always_healthy_nodes: usize,
    pub max_down_nodes: usize,
    pub num_large_validators: usize,
    pub add_execution_delay: bool,
    pub check_period_s: usize,
}

impl Test for ChangingWorkingQuorumTest {
    fn name(&self) -> &'static str {
        "changing working quorum test"
    }
}

impl NetworkLoadTest for ChangingWorkingQuorumTest {
    fn setup(&self, ctx: &mut NetworkContext) -> Result<LoadDestination> {
        // because we are doing failure testing, we should be sending
        // traffic to nodes that are alive.
        if ctx.swarm().full_nodes().count() > 0 {
            Ok(LoadDestination::AllFullnodes)
        } else if self.always_healthy_nodes > 0 {
            Ok(LoadDestination::Peers(
                ctx.swarm()
                    .validators()
                    .take(self.always_healthy_nodes)
                    .map(|v| v.peer_id())
                    .collect(),
            ))
        } else {
            Ok(LoadDestination::AllValidators)
        }
    }

    fn test(&self, swarm: &mut dyn Swarm, duration: Duration) -> Result<()> {
        let runtime = Runtime::new().unwrap();

        let validators = swarm.get_validator_clients_with_names();

        let num_validators = validators.len();

        let num_always_healthy = self.always_healthy_nodes;
        // largest number of (small) nodes that can fail simultaneously, while we have enough for quorum
        let can_fail_for_quorum =
            (self.num_large_validators * 10 + (num_validators - self.num_large_validators) - 1) / 3;
        // In our test, maximum number of nodes that we will fail simultaneously.
        let max_fail_in_test = std::cmp::min(
            std::cmp::min(self.max_down_nodes, num_validators - num_always_healthy),
            can_fail_for_quorum,
        );
        // On every cycle, we will fail this many next nodes, and make this many previous nodes healthy again.
        let cycle_offset = max_fail_in_test / 4 + 1;
        // Function that returns set of down nodes in a given cycle.
        let down_indices_f = move |cycle: usize| -> HashSet<usize> {
            (0..max_fail_in_test)
                .map(|i| {
                    num_always_healthy
                        + (cycle * cycle_offset + i) % (num_validators - num_always_healthy)
                })
                .collect()
        };
        info!(
            "Always healthy {} nodes, every cycle having {} nodes out of {} down, rotating {} each cycle, expecting first {} validators to have 10x larger stake",
            num_always_healthy, max_fail_in_test, num_validators, cycle_offset, self.num_large_validators);

        let slow_allowed_lagging = if self.add_execution_delay {
            runtime.block_on(async {
                let mut rng = rand::thread_rng();
                let mut slow_allowed_lagging = HashSet::new();
                for (index, (name, validator)) in
                    validators.iter().enumerate().skip(num_always_healthy)
                {
                    let sleep_time = rng.gen_range(20, 500);
                    if sleep_time > 100 {
                        slow_allowed_lagging.insert(index);
                    }
                    let name = name.clone();

                    validator
                        .set_failpoint(
                            "pont_vm::execution::block_metadata".to_string(),
                            format!("sleep({})", sleep_time),
                        )
                        .await
                        .map_err(|e| {
                            anyhow!(
                                "set_failpoint to remove execution delay on {} failed, {:?}",
                                name,
                                e
                            )
                        })?;
                }
                Ok::<HashSet<usize>, anyhow::Error>(slow_allowed_lagging)
            })?
        } else {
            HashSet::new()
        };

        let min_tps = self.min_tps;
        let check_period_s = self.check_period_s;

        runtime.block_on(test_consensus_fault_tolerance(
            swarm,
            duration.as_secs() as usize / self.check_period_s,
            self.check_period_s as f32,
            1,
            Box::new(FailPointFailureInjection::new(Box::new(move |cycle, part| {
                if part == 0 {
                    let down_indices = down_indices_f(cycle);
                    info!("For cycle {} down nodes: {:?}", cycle, down_indices);
                    // For all down nodes, we are going to drop all messages we receive.
                    (
                        down_indices.iter().flat_map(|i| {
                            [
                                (
                                    *i,
                                    "consensus::process::any".to_string(),
                                    "return".to_string(),
                                ),
                            ]
                        }).collect(),
                        true,
                    )
                } else {
                    (vec![], false)
                }
            }))),
            Box::new(move |cycle, _, _, _, cycle_end, cycle_start| {
                // we group nodes into 3 groups:
                // - active - nodes we expect to be making progress, and doing so together. we check wery strict rule of min(cycle_end) vs max(cycle_start)
                // - allowed_lagging - nodes that are allowed to not be up-to-date to the tip of the chain, but are required to be making individual progress.
                //                     We treat all nodes that were recently down as those (while state-sync is given time to catch-up), or nodes that
                //                     were added slowness into execution via add_execution_delay param.
                // - down - nodes that are cut-off from the rest of the nodes, and so shouldn't be seeing any progress. There should be no progress
                //          on the ordered certificates, but since we are only seeing committed ones, we allow for only minimal progress there, for
                //          what they already have in the buffer.

                let down_indices = down_indices_f(cycle);
                let recently_down_indices = if cycle > 0 { down_indices_f(cycle - 1) } else { HashSet::new() };
                fn split(all: Vec<NodeState>, down_indices: &HashSet<usize>, allowed_lagging_indices: &HashSet<usize>) -> (Vec<(usize, NodeState)>, Vec<(usize, NodeState)>, Vec<NodeState>) {
                    let (down, not_down): (Vec<_>, Vec<_>) = all.into_iter().enumerate().partition(|(idx, _state)| down_indices.contains(idx));
                    let (allowed_lagging, active)  = not_down.into_iter().partition(|(idx, _state)| allowed_lagging_indices.contains(idx));
                    (down, allowed_lagging, active.into_iter().map(|(_idx, state)| state).collect())
                }

                let allowed_lagging = recently_down_indices.union(&slow_allowed_lagging).cloned().collect::<HashSet<_>>();
                let (cycle_end_down, cycle_end_allowed_lagging, cycle_end_active) = split(cycle_end, &down_indices, &allowed_lagging);
                let (cycle_start_down, cycle_start_allowed_lagging, cycle_start_active) = split(cycle_start, &down_indices, &allowed_lagging);

                // Make sure that every active node is making progress, so we compare min(cycle_end) vs max(cycle_start)
                let (cycle_end_min_epoch, cycle_end_min_round) = cycle_end_active.iter().map(|s| (s.epoch, s.round)).min().unwrap();
                let (cycle_start_max_epoch, cycle_start_max_round) = cycle_start_active.iter().map(|s| (s.epoch, s.round)).max().unwrap();

                let epochs_progress = cycle_end_min_epoch as i64 - cycle_start_max_epoch as i64;
                let round_progress = cycle_end_min_round as i64 - cycle_start_max_round as i64;

                let transaction_progress = cycle_end_active.iter().map(|s| s.version).min().unwrap() as i64
                    - cycle_start_active.iter().map(|s| s.version).max().unwrap() as i64;

                if transaction_progress < (min_tps * check_period_s) as i64 {
                    bail!(
                        "not enough progress with active consensus, only {} transactions, expected >= {} ({} TPS). Down indices {:?}, cycle start active: {:?}. cycle end active: {:?}",
                        transaction_progress,
                        min_tps * check_period_s,
                        min_tps,
                        down_indices,
                        cycle_start_active,
                        cycle_end_active,
                    );
                }
                if epochs_progress < 0 || (epochs_progress == 0 && round_progress < (check_period_s / 2) as i64) {
                    bail!(
                        "not enough progress with active consensus, only {} epochs and {} rounds, expectd >= {}",
                        epochs_progress,
                        round_progress,
                        check_period_s / 2,
                    );
                }

                // Make sure that allowed_lagging nodes are making progress
                for ((node_idx, cycle_end_state), (node_idx_p, cycle_start_state)) in cycle_end_allowed_lagging.iter().zip(cycle_start_allowed_lagging.iter()) {
                    assert_eq!(node_idx, node_idx_p, "{:?} {:?}", cycle_end_allowed_lagging, cycle_start_allowed_lagging);
                    let transaction_progress = cycle_end_state.version as i64 - cycle_start_state.version as i64;
                    if transaction_progress < (min_tps * check_period_s) as i64 {
                        bail!(
                            "not enough individual progress on allowed lagging node ({}), only {} transactions, expected >= {} ({} TPS)",
                            node_idx,
                            transaction_progress,
                            min_tps * check_period_s,
                            min_tps,
                        );
                    }

                    let epochs_progress = cycle_end_state.epoch as i64 - cycle_start_state.epoch as i64;
                    let round_progress = cycle_end_state.round as i64 - cycle_start_state.round as i64;
                    if epochs_progress < 0 || (epochs_progress == 0 && round_progress < (check_period_s / 2) as i64) {
                        bail!(
                            "not enough individual progress on allowed lagging node ({}), only {} epochs and {} rounds, expectd >= {}. Transaction progress was {}.",
                            node_idx,
                            epochs_progress,
                            round_progress,
                            check_period_s / 2,
                            transaction_progress,
                        );
                    }
                }

                // Make sure down nodes don't make progress:
                for ((node_idx, cycle_end_state), (node_idx_p, cycle_start_state)) in cycle_end_down.iter().zip(cycle_start_down.iter()) {
                    assert_eq!(node_idx, node_idx_p, "{:?} {:?}", cycle_end_down, cycle_start_down);
                    if cycle_end_state.round > cycle_start_state.round + 3 {
                        // if we just failed the node, some progress can happen due to pipeline in consensus,
                        // or buffer of received messages in state sync
                        if recently_down_indices.contains(node_idx) {
                            bail!("progress on down node {} from ({}, {}) to ({}, {})", node_idx, cycle_start_state.epoch, cycle_start_state.round, cycle_end_state.epoch, cycle_end_state.round);
                        } else {
                            warn!("progress on down node {} immediatelly after turning off from ({}, {}) to ({}, {})", node_idx, cycle_start_state.epoch, cycle_start_state.round, cycle_end_state.epoch, cycle_end_state.round)
                        }
                    }
                }

                Ok(())
            }),
            false,
            true,
        )).context("test_consensus_fault_tolerance failed")?;

        // undo slowing down.
        if self.add_execution_delay {
            runtime.block_on(async {
                for (name, validator) in validators.iter().skip(num_always_healthy) {
                    let name = name.clone();

                    validator
                        .set_failpoint(
                            "pont_vm::execution::block_metadata".to_string(),
                            "off".to_string(),
                        )
                        .await
                        .map_err(|e| {
                            anyhow!(
                                "set_failpoint to remove execution delay on {} failed, {:?}",
                                name,
                                e
                            )
                        })?;
                }
                Ok::<(), anyhow::Error>(())
            })?;
        }
        Ok(())
    }
}

impl NetworkTest for ChangingWorkingQuorumTest {
    fn run<'t>(&self, ctx: &mut NetworkContext<'t>) -> Result<()> {
        <dyn NetworkLoadTest>::run(self, ctx)
    }
}
