// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;
use pont_gas::gen::{generate_update_proposal, GenArgs};

fn main() -> Result<()> {
    let args = GenArgs::parse();

    generate_update_proposal(&args)
}
