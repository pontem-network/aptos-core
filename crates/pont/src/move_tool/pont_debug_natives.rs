// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use move_deps::move_vm_runtime::native_functions::NativeFunctionTable;
use pont_gas::{AbstractValueSizeGasParameters, NativeGasParameters, LATEST_GAS_FEATURE_VERSION};
use pont_vm::natives;

// move_stdlib has the testing feature enabled to include debug native functions
pub fn pont_debug_natives(
    gas_parameters: NativeGasParameters,
    abs_val_size_gas_params: AbstractValueSizeGasParameters,
) -> NativeFunctionTable {
    // As a side effect, also configure for unit testing
    natives::configure_for_unit_test();
    // Return all natives -- build with the 'testing' feature, therefore containing
    // debug related functions.
    natives::pont_natives(
        gas_parameters,
        abs_val_size_gas_params,
        LATEST_GAS_FEATURE_VERSION,
    )
}
