// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::Lazy;
use pont_metrics_core::{register_int_counter, IntCounter};

pub static TRANSACTIONS_SENT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "pont_fh_stream_transactions_sent_count",
        "Transactions converted and printed out to stdout, picked up by the StreamingFast Firehose indexer",
    )
    .unwrap()
});

pub static BLOCKS_SENT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "pont_fh_stream_blocks_sent_count",
        "Blocks converted and printed out to stdout, picked up by the StreamingFast Firehose indexer",
    )
    .unwrap()
});
