// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by [`PontDB`](crate::PontDB).

use thiserror::Error;

/// This enum defines errors commonly used among [`PontDB`](crate::PontDB) APIs.
#[derive(Debug, Error)]
pub enum PontDbError {
    /// A requested item is not found.
    #[error("{0} not found.")]
    NotFound(String),
    /// Requested too many items.
    #[error("Too many items requested: at least {0} requested, max is {1}")]
    TooManyRequested(u64, u64),
}
