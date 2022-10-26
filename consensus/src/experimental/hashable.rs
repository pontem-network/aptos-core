// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use pont_crypto::HashValue;

pub trait Hashable {
    fn hash(&self) -> HashValue;
}
