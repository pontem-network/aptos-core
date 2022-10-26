// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{tests::common, MoveHarness};
use language_e2e_tests::account::Account;
use move_core_types::account_address::AccountAddress;
use pont_types::transaction::{ExecutionStatus, TransactionStatus};

#[test]
fn can_upgrade_framework_on_testnet() {
    let mut h = MoveHarness::new_testnet();
    h.increase_transaction_size();

    // Upgrade all frameworks in bottom up order as they may have dependencies from each other
    let acc1 = h.pont_framework_account();
    publish(&acc1, &mut h, "move-stdlib");
    publish(&acc1, &mut h, "pont-stdlib");
    publish(&acc1, &mut h, "pont-framework");
    let acc3 = h.new_account_at(AccountAddress::from_hex_literal("0x3").unwrap());
    publish(&acc3, &mut h, "pont-token");
}

fn publish(acc: &Account, h: &mut MoveHarness, path: &str) {
    match h.publish_package(acc, &common::framework_dir_path(path)) {
        TransactionStatus::Keep(ExecutionStatus::Success) => {}
        s => {
            panic!("cannot publish `{}`: {:?}", path, s)
        }
    }
}
