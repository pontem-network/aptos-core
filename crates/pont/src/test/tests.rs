// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["pont"]).await;

    assert_cmd_not_panic(&["pont", "account"]).await;
    assert_cmd_not_panic(&["pont", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["pont", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["pont", "config"]).await;
    assert_cmd_not_panic(&["pont", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["pont", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["pont", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["pont", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["pont", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["pont", "genesis"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["pont", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["pont", "governance"]).await;
    assert_cmd_not_panic(&["pont", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["pont", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["pont", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["pont", "governance", "vote", "--help"]).await;

    assert_cmd_not_panic(&["pont", "info"]).await;

    assert_cmd_not_panic(&["pont", "init", "--help"]).await;

    assert_cmd_not_panic(&["pont", "key"]).await;
    assert_cmd_not_panic(&["pont", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["pont", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["pont", "move"]).await;
    assert_cmd_not_panic(&["pont", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["pont", "move", "transactional-test", "--help"]).await;

    assert_cmd_not_panic(&["pont", "node"]).await;
    assert_cmd_not_panic(&["pont", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["pont", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "pont",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["pont", "stake"]).await;
    assert_cmd_not_panic(&["pont", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["pont", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://pontlabs.com").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://pontlabs.com".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
