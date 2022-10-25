// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::types::{CliCommand, CliTypedResult, TransactionOptions, TransactionSummary};
use async_trait::async_trait;
use cached_packages::pont_stdlib;
use clap::Parser;
use pont_types::account_address::AccountAddress;

// TODO(Gas): double check if this is correct
// TODO(greg): revisit after fixing gas estimation
pub const DEFAULT_FUNDED_COINS: u64 = 500_000;

/// Command to create a new account on-chain
///
#[derive(Debug, Parser)]
pub struct CreateAccount {
    /// Address of the new account
    #[clap(long, parse(try_from_str=crate::common::types::load_account_arg))]
    pub(crate) account: AccountAddress,

    #[clap(flatten)]
    pub(crate) txn_options: TransactionOptions,
}

#[async_trait]
impl CliCommand<TransactionSummary> for CreateAccount {
    fn command_name(&self) -> &'static str {
        "CreateAccount"
    }

    async fn execute(self) -> CliTypedResult<TransactionSummary> {
        let address = self.account;
        self.txn_options
            .submit_transaction(pont_stdlib::pont_account_create_account(address))
            .await
            .map(TransactionSummary::from)
    }
}
