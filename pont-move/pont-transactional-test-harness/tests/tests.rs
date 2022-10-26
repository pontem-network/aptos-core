// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use pont_transactional_test_harness::run_pont_test;

datatest_stable::harness!(run_pont_test, "tests", r".*\.(mvir|move)$");
