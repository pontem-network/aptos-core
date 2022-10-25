// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use pont_sdk::types::{transaction::SignedTransaction, LocalAccount};

pub mod account_generator;
pub mod nft_mint_and_transfer;
pub mod p2p_transaction_generator;
pub mod transaction_mix_generator;

pub trait TransactionGenerator: Sync + Send {
    fn generate_transactions(
        &mut self,
        accounts: Vec<&mut LocalAccount>,
        transactions_per_account: usize,
    ) -> Vec<SignedTransaction>;
}

#[async_trait]
pub trait TransactionGeneratorCreator: Sync + Send {
    async fn create_transaction_generator(&self) -> Box<dyn TransactionGenerator>;
}
