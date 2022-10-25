// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use pont_types::{account_address::AccountAddress, transaction::TransactionPayload};

pub use crate::pont_framework_sdk_builder::*;
pub use crate::pont_token_sdk_builder as pont_token_stdlib;

pub fn pont_coin_transfer(to: AccountAddress, amount: u64) -> TransactionPayload {
    coin_transfer(
        pont_types::utility_coin::APTOS_COIN_TYPE.clone(),
        to,
        amount,
    )
}
