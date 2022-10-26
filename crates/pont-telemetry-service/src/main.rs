#![forbid(unsafe_code)]

// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use pont_telemetry_service::PontTelemetryServiceArgs;

#[tokio::main]
async fn main() {
    pont_logger::Logger::new().init();
    PontTelemetryServiceArgs::parse().run().await;
}
