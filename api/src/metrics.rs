// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::Lazy;
use pont_metrics_core::{register_histogram_vec, HistogramVec};

pub static HISTOGRAM: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "pont_api_requests",
        "API requests latency grouped by method, operation_id and status",
        &["method", "operation_id", "status"]
    )
    .unwrap()
});

pub static RESPONSE_STATUS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "pont_api_response_status",
        "API requests latency grouped by status code only",
        &["status"]
    )
    .unwrap()
});
