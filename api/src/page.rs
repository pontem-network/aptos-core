// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::response::BadRequestError;
use pont_api_types::{LedgerInfo, PontErrorCode};
use serde::Deserialize;

const DEFAULT_PAGE_SIZE: u16 = 25;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Page {
    start: Option<u64>,
    limit: Option<u16>,
    max_page_size: u16,
}

impl Page {
    pub fn new(start: Option<u64>, limit: Option<u16>, max_page_size: u16) -> Self {
        Self {
            start,
            limit,
            max_page_size,
        }
    }

    /// Compute the start of the page for transactions
    pub fn compute_start<E: BadRequestError>(
        &self,
        limit: u16,
        max: u64,
        ledger_info: &LedgerInfo,
    ) -> Result<u64, E> {
        let last_page_start = max.saturating_sub((limit.saturating_sub(1)) as u64);
        self.start(last_page_start, max, ledger_info)
    }

    /// Retrieve the start of the page
    fn start<E: BadRequestError>(
        &self,
        default: u64,
        max: u64,
        ledger_info: &LedgerInfo,
    ) -> Result<u64, E> {
        let start = self.start.unwrap_or(default);
        if start > max {
            return Err(E::bad_request_with_code(
                &format!(
                "Given start value ({}) is higher than the current ledger version, it must be < {}",
                start, max
            ),
                PontErrorCode::InvalidInput,
                ledger_info,
            ));
        }
        Ok(start)
    }

    /// Retrieve the start version
    pub fn start_option(&self) -> Option<u64> {
        self.start
    }

    /// Get the page size for the request
    pub fn limit<E: BadRequestError>(&self, ledger_info: &LedgerInfo) -> Result<u16, E> {
        let limit = self.limit.unwrap_or(DEFAULT_PAGE_SIZE);
        if limit == 0 {
            return Err(E::bad_request_with_code(
                &format!("Given limit value ({}) must not be zero", limit),
                PontErrorCode::InvalidInput,
                ledger_info,
            ));
        }
        // If we go over the max page size, we return the max page size
        if limit > self.max_page_size {
            Ok(self.max_page_size)
        } else {
            Ok(limit)
        }
    }
}
