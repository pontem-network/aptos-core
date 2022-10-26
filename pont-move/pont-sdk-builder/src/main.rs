// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! # Code generator for Move script builders
//!
//! '''bash
//! cargo run -p pont-sdk-builder -- --help
//! '''

use serde_generate as serdegen;
use serde_reflection::Registry;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
#[derive(Debug, StructOpt)]
enum Language {
    Rust,
    Go,
}
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Pont SDK Builder", about = "Generate boilerplate Pont SDKs")]
struct Options {
    /// Path to the directory containing ABI files in BCS encoding.
    abi_directories: Vec<PathBuf>,

    /// Language for code generation.
    #[structopt(long, possible_values = &Language::variants(), case_insensitive = true, default_value = "Rust")]
    language: Language,

    /// Directory where to write generated modules (otherwise print code on stdout).
    #[structopt(long)]
    target_source_dir: Option<PathBuf>,

    /// Also install the pont types described by the given YAML file, along with the BCS runtime.
    #[structopt(long)]
    with_pont_types: Option<PathBuf>,

    /// Module name for the transaction builders installed in the `target_source_dir`.
    /// * Rust crates may contain a version number, e.g. "test:1.2.0".
    /// * In Java, this is expected to be a package name, e.g. "com.test" to create Java files in `com/test`.
    /// * In Go, this is expected to be of the format "go_module/path/go_package_name",
    /// and `pont_types` is assumed to be in "go_module/path/pont_types".
    #[structopt(long)]
    module_name: Option<String>,

    /// Optional package name (Python) or module path (Go) of the Serde and BCS runtime dependencies.
    #[structopt(long)]
    serde_package_name: Option<String>,

    /// Optional version number for the `pont_types` module (useful in Rust).
    /// If `--with-pont-types` is passed, this will be the version of the generated `pont_types` module.
    #[structopt(long, default_value = "0.1.0")]
    pont_version_number: String,

    /// Optional package name (Python) or module path (Go) of the `pont_types` dependency.
    #[structopt(long)]
    package_name: Option<String>,
}

fn main() {
    let options = Options::from_args();
    let abis = pont_sdk_builder::read_abis(&options.abi_directories)
        .expect("Failed to read ABI in directory");

    let install_dir = match options.target_source_dir {
        None => {
            // Nothing to install. Just print to stdout.
            let stdout = std::io::stdout();
            let mut out = stdout.lock();
            match options.language {
                Language::Rust => {
                    pont_sdk_builder::rust::output(&mut out, &abis, /* local types */ false)
                        .unwrap()
                }
                Language::Go => {
                    pont_sdk_builder::golang::output(
                        &mut out,
                        options.serde_package_name.clone(),
                        options.package_name.clone(),
                        options.module_name.as_deref().unwrap_or("main").to_string(),
                        &abis,
                    )
                    .unwrap();
                }
            }
            return;
        }
        Some(dir) => dir,
    };

    // Pont types
    if let Some(registry_file) = options.with_pont_types {
        let installer: Box<dyn serdegen::SourceInstaller<Error = Box<dyn std::error::Error>>> =
            match options.language {
                Language::Rust => Box::new(serdegen::rust::Installer::new(install_dir.clone())),
                Language::Go => Box::new(serdegen::golang::Installer::new(
                    install_dir.clone(),
                    options.serde_package_name.clone(),
                )),
            };

        let content =
            std::fs::read_to_string(registry_file).expect("registry file must be readable");
        let mut registry = serde_yaml::from_str::<Registry>(content.as_str()).unwrap();
        // update the registry to prevent language keyword being used
        if let Language::Rust = options.language {
            pont_sdk_builder::rust::replace_keywords(&mut registry)
        }

        let (package_name, _package_path) = match options.language {
            Language::Rust => (
                if options.pont_version_number == "0.1.0" {
                    "pont-types".to_string()
                } else {
                    format!("pont-types:{}", options.pont_version_number)
                },
                vec!["pont-types"],
            ),
            Language::Go => ("ponttypes".to_string(), vec!["ponttypes"]),
        };

        let config = serdegen::CodeGeneratorConfig::new(package_name)
            .with_encodings(vec![serdegen::Encoding::Bcs]);

        installer.install_module(&config, &registry).unwrap();
    }

    // Transaction builders
    let installer: Box<dyn pont_sdk_builder::SourceInstaller<Error = Box<dyn std::error::Error>>> =
        match options.language {
            Language::Rust => Box::new(pont_sdk_builder::rust::Installer::new(
                install_dir,
                options.pont_version_number,
            )),
            Language::Go => Box::new(pont_sdk_builder::golang::Installer::new(
                install_dir,
                options.serde_package_name,
                options.package_name,
            )),
        };

    if let Some(name) = options.module_name {
        installer
            .install_transaction_builders(&name, abis.as_slice())
            .unwrap();
    }
}
