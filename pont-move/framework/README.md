---
id: Pont-framework
title: Pont Framework
custom_edit_url: https://github.com/aptos-labs/pont-core/edit/main/Pont-move/Pont-framework/README.md
---

## The Pont Framework

The Pont Framework defines the standard actions that can be performed on-chain
both by the Pont VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the Pont Framework and contributing guidelines are documented separately. Particularly:
* Documentation for the set of allowed transaction scripts in pont-framework can be found in [script_documentation.md](pont-framework/releases/artifacts/current/build/PontFramework/docs/script_documentation.md).
* The overview documentation for the Move modules can be found in [overview.md](pont-framework/releases/artifacts/current/build/PontFramework/docs/overview.md).
* An overview of the approach to formal verification of the framework can be found in [spec_documentation.md](pont-framework/releases/artifacts/current/build/PontFramework/docs/spec_documentation.md).
* Contributing guidelines and basic coding standards for the Pont Framework can be found in [CONTRIBUTING.md](CONTRIBUTING.md).

## Compilation and Generation

Recompilation of the Pont Framework and the regeneration of the documents,
ABIs, and error information can be performed by running `cargo run` from this
directory. There are a number of different options available and these are
explained in the help for this command by running `cargo run -- --help` in this
directory. Compilation and generation will be much faster if run in release
mode (`cargo run --release`).

## Layout
The overall structure of the Pont Framework is as follows:

```
├── compiled                                # Generated files and public rust interface to the Pont Framework
│   ├── error_descriptions/*.errmap         # Generated error descriptions for use by the Move Explain tool
│   ├── src                                 # External Rust interface/library to use the Pont Framework
│   ├── stdlib                              # The compiled Move bytecode of the Pont Framework source modules
│   ├── script_abis                         # Generated ABIs for entry function transactions, and all new transactions
│   └── legacy/transaction_scripts          # Legacy generated ABIs and bytecode for each transaction script in the allowlist
│       ├── abi/*.abi                       # Directory containing generated ABIs for legacy transaction scripts
│       └── *.mv
├── modules                                 # Pont Framework source modules, script modules, and generated documentation
│   ├── *.move
│   └── doc/*.md                            # Generated documentation for the Pont Framework modules
├── nursery/*.move                          # Move modules that are not published on-chain, but are used for testing and debugging locally
├── src                                     # Compilation and generation of information from Move source files in the Pont Framework. Not designed to be used as a Rust library
├── tests
└── script_documentation/*.md               # Generated documentation for allowed transaction scripts
```
