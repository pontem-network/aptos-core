// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::Lazy;
use pont_metrics_core::{register_int_counter, register_int_gauge, IntCounter, IntGauge};

pub static APTOS_JELLYFISH_LEAF_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "pont_jellyfish_leaf_encoded_bytes",
        "Pont jellyfish leaf encoded bytes in total"
    )
    .unwrap()
});

pub static APTOS_JELLYFISH_INTERNAL_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "pont_jellyfish_internal_encoded_bytes",
        "Pont jellyfish total internal nodes encoded in bytes"
    )
    .unwrap()
});

pub static APTOS_JELLYFISH_LEAF_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "pont_jellyfish_leaf_count",
        "Total number of leaves in the latest JMT."
    )
    .unwrap()
});

pub static APTOS_JELLYFISH_LEAF_DELETION_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "pont_jellyfish_leaf_deletion_count",
        "The number of deletions happened in JMT."
    )
    .unwrap()
});
