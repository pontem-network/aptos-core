// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::natives::util::make_native_from_func;

use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte, NumBytes};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};

use ff::{PrimeField, PrimeFieldRepr};
use pont_types::vm_status::StatusCode;
use poseidon_rs::{Fr, Poseidon};
use smallvec::smallvec;
use std::io::Cursor;
use std::{collections::VecDeque, hash::Hasher};
use tiny_keccak::{Hasher as KeccakHasher, Keccak};

/***************************************************************************************************
 * native fun sip_hash
 *
 *   gas cost: base_cost + unit_cost * data_length
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct SipHashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

/// Feed these bytes into SipHasher. This is not cryptographically secure.
fn native_sip_hash(
    gas_params: &SipHashGasParameters,
    _context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vec<u8>);

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    // SipHash of the serialized bytes
    let mut hasher = siphasher::sip::SipHasher::new();
    hasher.write(&bytes);
    let hash = hasher.finish();

    Ok(NativeResult::ok(cost, smallvec![Value::u64(hash)]))
}

#[derive(Debug, Clone)]
pub struct Keccak256HashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

fn native_keccak256(
    gas_params: &Keccak256HashGasParameters,
    _context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vec<u8>);

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    let mut hasher = Keccak::v256();
    hasher.update(&bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

#[derive(Debug, Clone)]
pub struct PoseidonHashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

fn native_poseidon(
    gas_params: &PoseidonHashGasParameters,
    _context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vec<u8>);
    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    let ff: Fr = from_bytes(bytes)
        .map_err(|msg| PartialVMError::new(StatusCode::DATA_FORMAT_ERROR).with_message(msg))?;
    let hasher = Poseidon::new();
    // println!("bytes: {:?}", hex::encode(&bytes));
    //
    // let bytes: Fr = Fr::from_str(hex::encode(&bytes).as_str()).ok_or_else(|| {
    //     PartialVMError::new(StatusCode::DATA_FORMAT_ERROR)
    //         .with_message("Poseidon hash failed".to_string())
    // })?;
    let hash = hasher
        .hash(vec![ff])
        .map_err(|msg| PartialVMError::new(StatusCode::DATA_FORMAT_ERROR).with_message(msg))?;

    let repr = hash.into_repr();

    let mut cursor = Cursor::new([0u8; 32]);
    repr.write_be(&mut cursor).map_err(|msg| {
        PartialVMError::new(StatusCode::DATA_FORMAT_ERROR).with_message(msg.to_string())
    })?;

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(cursor.into_inner())],
    ))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct GasParameters {
    pub sip_hash: SipHashGasParameters,
    pub keccak256: Keccak256HashGasParameters,
    pub poseidon: PoseidonHashGasParameters,
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "sip_hash",
            make_native_from_func(gas_params.sip_hash, native_sip_hash),
        ),
        (
            "keccak256",
            make_native_from_func(gas_params.keccak256, native_keccak256),
        ),
        (
            "poseidon",
            make_native_from_func(gas_params.poseidon, native_poseidon),
        ),
    ];

    crate::natives::helpers::make_module_natives(natives)
}

pub fn from_bytes<F: PrimeField>(mut buf: Vec<u8>) -> Result<F, String> {
    let mut repr = F::Repr::default();
    let required_length = repr.as_ref().len() * 8;
    buf.reverse();
    buf.resize(required_length, 0);

    repr.read_le(&buf[..])
        .map_err(|e| format!("could not buffer {}", &e))?;

    F::from_repr(repr).map_err(|e| format!("could not convert into prime field {}", &e))
}
