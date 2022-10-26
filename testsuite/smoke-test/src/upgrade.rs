// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    pont::move_test_helpers, smoke_test_environment::SwarmBuilder,
    test_utils::check_create_mint_transfer, workspace_builder, workspace_builder::workspace_root,
};
use forge::Swarm;
use pont_crypto::ValidCryptoMaterialStringExt;
use pont_gas::{GasQuantity, InitialGasSchedule, PontGasParameters, ToOnChainGasSchedule};
use pont_temppath::TempPath;
use std::process::Command;
use std::{fmt::Write, fs};

fn generate_blob(data: &[u8]) -> String {
    let mut buf = String::new();

    write!(buf, "vector[").unwrap();
    for (i, b) in data.iter().enumerate() {
        if i % 20 == 0 {
            if i > 0 {
                writeln!(buf).unwrap();
            }
        } else {
            write!(buf, " ").unwrap();
        }
        write!(buf, "{}u8,", b).unwrap();
    }
    write!(buf, "]").unwrap();
    buf
}

#[tokio::test]
/// This test verifies the flow of pont framework upgrade process.
/// i.e: The network will be alive after applying the new pont framework release.
async fn test_upgrade_flow() {
    // prebuild tools.
    let pont_cli = workspace_builder::get_bin("pont");

    let num_nodes = 5;
    let (mut env, _cli, _) = SwarmBuilder::new_local(num_nodes)
        .with_pont_testnet()
        .build_with_cli(0)
        .await;

    let url = env.pont_public_info().url().to_string();
    let private_key = env
        .pont_public_info()
        .root_account()
        .private_key()
        .to_encoded_string()
        .unwrap();

    // Bump the limit in gas schedule
    // TODO: Replace this logic with pont-gas
    let mut gas_parameters = PontGasParameters::initial();
    gas_parameters.txn.max_transaction_size_in_bytes = GasQuantity::new(100_000_000);

    let gas_schedule = pont_types::on_chain_config::GasScheduleV2 {
        feature_version: pont_gas::LATEST_GAS_FEATURE_VERSION,
        entries: gas_parameters.to_on_chain_gas_schedule(),
    };

    let update_gas_script = format!(
        r#"
    script {{
        use pont_framework::pont_governance;
        use pont_framework::gas_schedule;

        fun main(core_resources: &signer) {{
            let framework_signer = pont_governance::get_signer_testnet_only(core_resources, @0000000000000000000000000000000000000000000000000000000000000001);

            let gas_bytes = {};

            gas_schedule::set_gas_schedule(&framework_signer, gas_bytes);
        }}
    }}
    "#,
        generate_blob(&bcs::to_bytes(&gas_schedule).unwrap())
    );

    let gas_script_path = TempPath::new();
    let mut gas_script_path = gas_script_path.path().to_path_buf();
    gas_script_path.set_extension("move");
    fs::write(gas_script_path.as_path(), update_gas_script).unwrap();

    assert!(Command::new(pont_cli.as_path())
        .current_dir(workspace_root())
        .args(&vec![
            "move",
            "run-script",
            "--script-path",
            gas_script_path.to_str().unwrap(),
            "--sender-account",
            "0xA550C18",
            "--url",
            url.as_str(),
            "--private-key",
            private_key.as_str(),
            "--assume-yes",
        ])
        .output()
        .unwrap()
        .status
        .success());

    // Generate the governance proposal script.
    let package_path_list = vec![
        ("0x1", "pont-move/framework/move-stdlib"),
        ("0x1", "pont-move/framework/pont-stdlib"),
        ("0x1", "pont-move/framework/pont-framework"),
        ("0x3", "pont-move/framework/pont-token"),
    ];

    for (publish_addr, relative_package_path) in package_path_list.iter() {
        let temp_script_path = TempPath::new();
        let mut move_script_path = temp_script_path.path().to_path_buf();
        move_script_path.set_extension("move");

        let mut package_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        package_path.pop();
        package_path.pop();
        package_path.push(relative_package_path);

        assert!(Command::new(pont_cli.as_path())
            .current_dir(workspace_root())
            .args(&vec![
                "governance",
                "generate-upgrade-proposal",
                "--testnet",
                "--account",
                publish_addr,
                "--package-dir",
                package_path.to_str().unwrap(),
                "--output",
                move_script_path.to_str().unwrap(),
            ])
            .output()
            .unwrap()
            .status
            .success());

        assert!(Command::new(pont_cli.as_path())
            .current_dir(workspace_root())
            .args(&vec![
                "move",
                "run-script",
                "--script-path",
                move_script_path.to_str().unwrap(),
                "--sender-account",
                "0xA550C18",
                "--url",
                url.as_str(),
                "--private-key",
                private_key.as_str(),
                "--assume-yes",
            ])
            .output()
            .unwrap()
            .status
            .success());
    }

    // Two transactions has been emitted to root account.
    *env.pont_public_info().root_account().sequence_number_mut() +=
        1 + package_path_list.len() as u64;

    // Test the module publishing workflow
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let base_path_v1 = base_dir.join("src/pont/package_publish_modules_v1/");

    move_test_helpers::publish_package(&mut env.pont_public_info(), base_path_v1)
        .await
        .unwrap();

    check_create_mint_transfer(&mut env).await;
}
