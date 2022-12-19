use anyhow::{Context, Result};
use pont_types::account_address::AccountAddress;

use once_cell::sync::Lazy;
use pont_crypto::ed25519::Ed25519PrivateKey;
use pont_rest_client::{pont_api_types, Client};
use pont_sdk::coin_client::CoinClient;
use pont_sdk::types::LocalAccount;
use pont_types::transaction::TransactionPayload;
use std::str::FromStr;
use url::Url;

use log::{error, info, log_enabled, Level};
use pont_types::transaction::Script;

use pont_global_constants::{GAS_UNIT_PRICE, MAX_GAS_AMOUNT};
use pont_sdk::transaction_builder::{TransactionBuilder, TransactionFactory};
use pont_types::chain_id::ChainId;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use framework::BuiltPackage;
use move_compiler::{
    compiled_unit::CompiledUnitEnum,
    shared::{NumberFormat, NumericalAddress},
};
use pont_api_types::Transaction as APITransaction;

pub const PONT_CHAIN_ID: u8 = 4;
pub const MAX_TIMEOUT: Duration = Duration::from_secs(30000000);

pub static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("PONT_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("http://44.212.113.202:8080"),
    )
    .unwrap()
});

pub async fn make_transaction_script(
    account: &mut LocalAccount,
    path: &PathBuf,
) -> Result<APITransaction> {
    let rest_client = Client::new_with_timeout(NODE_URL.clone(), MAX_TIMEOUT);
    let script = Script::new(fs::read(path)?, vec![], vec![]);

    let max_gas_amount = 2 * MAX_GAS_AMOUNT;
    let gas_unit_price: u64 = std::cmp::max(GAS_UNIT_PRICE, 1);
    let expiration_timestamp_secs: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        + 100;

    let transaction_builder = TransactionBuilder::new(
        TransactionPayload::Script(script),
        expiration_timestamp_secs,
        ChainId::new(PONT_CHAIN_ID),
    )
    .sender(account.address())
    .sequence_number(account.sequence_number())
    .max_gas_amount(max_gas_amount)
    .gas_unit_price(gas_unit_price);

    let signed_txn = account.sign_with_transaction_builder(transaction_builder);

    let txn_hash = rest_client
        .submit(&signed_txn)
        .await
        .context("failed build transaction")?
        .inner()
        .clone();
    Ok(rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("failed run transaction")?
        .inner()
        .clone())
}

pub async fn create_local_account(address: &str, private_key: &str) -> Result<LocalAccount> {
    let rest_client = Client::new_with_timeout(NODE_URL.clone(), MAX_TIMEOUT);
    let key_bytes = hex::decode(private_key)?;
    let private_key: Ed25519PrivateKey = (&key_bytes[..]).try_into()?;
    let address: AccountAddress = address.parse()?;
    let sequence_number = rest_client
        .get_account(address)
        .await?
        .inner()
        .sequence_number;
    Ok(LocalAccount::new(address, private_key, sequence_number))
}

pub async fn create_account_on_chain(
    sender: &mut LocalAccount,
    account: &mut LocalAccount,
) -> Result<()> {
    let rest_client = Client::new_with_timeout(NODE_URL.clone(), MAX_TIMEOUT);

    let max_gas_amount = 2 * MAX_GAS_AMOUNT;
    let gas_unit_price: u64 = std::cmp::max(GAS_UNIT_PRICE, 1);
    let txn_factory = TransactionFactory::new(ChainId::new(PONT_CHAIN_ID));
    let txn_builder = txn_factory
        .create_user_account(account.public_key())
        .sender(sender.address())
        .sequence_number(sender.sequence_number())
        .max_gas_amount(max_gas_amount)
        .gas_unit_price(gas_unit_price);

    let signed_txn = sender.sign_with_transaction_builder(txn_builder);

    let txn_hash = rest_client
        .submit(&signed_txn)
        .await
        .context("failed build transaction")?
        .inner()
        .clone();

    let res = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("failed run transaction")?
        .inner()
        .clone();

    if log_enabled!(Level::Info) && res.success() {
        info!("Created account: {}", account.address().to_hex_literal());
    }

    Ok(())
}

pub async fn fast_transaction(
    from: &mut LocalAccount,
    to: AccountAddress,
    amount: u64,
) -> Result<()> {
    let rest_client = Client::new_with_timeout(NODE_URL.clone(), MAX_TIMEOUT);
    let coin_client = CoinClient::new(&rest_client);

    let txn_hash = coin_client
        .transfer(from, to, amount, None)
        .await
        .context("Failed to submit transaction to transfer coins")?;
    let res = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("Failed when waiting for the transfer transaction")?
        .inner()
        .clone();

    if log_enabled!(Level::Info) && res.success() {
        info!("fast transaction success");
    }

    Ok(())
}

pub async fn long_transaction(account: &mut LocalAccount) -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("mv/build/load-generator-example28/bytecode_scripts/main.mv")
        .canonicalize()?;
    let res = make_transaction_script(account, &path).await?;

    if log_enabled!(Level::Info) {
        if res.success() {
            info!("Script executed successfully");
            info!("gas used : {}", res.transaction_info()?.gas_used);
        } else {
            error!("Script failed to execute");
            error!("VM status {:?}", res.vm_status());
        }
    }
    Ok(())
}

pub async fn publish_module(
    account: &mut LocalAccount,
    shared_package: BuiltPackage,
) -> Result<()> {
    let mut package = shared_package;
    let rest_client = Client::new_with_timeout(NODE_URL.clone(), MAX_TIMEOUT);

    let max_gas_amount = 2 * MAX_GAS_AMOUNT;
    let gas_unit_price: u64 = std::cmp::max(GAS_UNIT_PRICE, 1);
    let expiration_timestamp_secs: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        + 10;

    match package.package.root_compiled_units[0].unit {
        CompiledUnitEnum::Module(ref mut m) => {
            m.address = NumericalAddress::new(account.address().into_bytes(), NumberFormat::Hex);
            m.module.address_identifiers[0] = account.address();
            if let Some(ref mut name) = m.source_map.module_name_opt {
                name.0 = account.address();
            }
        }
        _ => {
            error!("Couldn't find module");
        }
    };
    let compiled_units = package.extract_code();
    let metadata = package.extract_metadata()?;
    let payload = cached_packages::pont_stdlib::code_publish_package_txn(
        bcs::to_bytes(&metadata).expect("PackageMetadata has BCS"),
        compiled_units,
    );

    let transaction_factory = TransactionFactory::new(ChainId::new(PONT_CHAIN_ID))
        .with_gas_unit_price(gas_unit_price)
        .with_max_gas_amount(max_gas_amount)
        .with_transaction_expiration_time(expiration_timestamp_secs);

    let transaction = account.sign_with_transaction_builder(transaction_factory.payload(payload));

    let txn_hash = rest_client
        .submit(&transaction)
        .await
        .context("failed build transaction")?
        .inner()
        .clone();
    let res = rest_client
        .wait_for_transaction(&txn_hash)
        .await
        .context("failed run transaction")?
        .inner()
        .clone();

    if log_enabled!(Level::Info) {
        if res.success() {
            info!("Module published successfully");
            info!("gas used : {}", res.transaction_info()?.gas_used);
        } else {
            error!("Module failed to publish");
            error!("VM status {:?}", res.vm_status());
        }
    }
    Ok(())
}

pub async fn get_shared_resource(account: &mut LocalAccount) -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources/build/res_test7/bytecode_scripts/main.mv")
        .canonicalize()?;
    let res = make_transaction_script(account, &path).await?;

    if log_enabled!(Level::Info) {
        match res.success() {
            true => {
                info!("resource got successfully");
            }
            false => {
                error!("error while getting resource");
            }
        }
    }

    Ok(())
}
