---
title: "Installing Pont CLI"
id: "install-pont-cli"
---

# Installing Pont CLI

The `pont` tool is a command line interface (CLI) for developing on the Pont blockchain, debugging Move contracts, and conducting node operations. This document describes how to install the `pont` CLI tool. See [Use Pont CLI](use-pont-cli) for how to use the CLI.

To download the Pont source code, related tools, and IDE plugins for the Move programming language, follow [Getting Started](../../guides/getting-started.md).

Install the CLI by downloading the precompiled binary for your platform, as described below. 

:::tip Move Prover Dependencies
If you want to use the [Move Prover](https://github.com/move-language/move/blob/main/language/move-prover/doc/user/prover-guide.md) to validate your Move code, [install the Move Prover dependencies](#optional-install-the-dependencies-of-move-prover) after installing the CLI binary.
:::

## Download precompiled binary
<details>
<summary>macOS</summary>

### macOS
:::tip
These instructions have been tested on macOS Monterey (12.6)
:::


1. Go to the [Pont CLI Release](https://github.com/aptos-labs/pont-core/releases?q=cli&expanded=true) list.
1. Click the **Assets** expandable menu for the latest release. 
1. You will see the zip files with the filename of the format: `pont-cli-<version>-<platform>`. These are the platform-specific pre-compiled binaries of the CLI. Download the zip file for your platform, dismissing any warnings.
1. Unzip the downloaded file. This will extract the `pont` CLI binary file into your default downloads folder. For example, on macOS it is the `~/Downloads` folder.
1. Move this extracted `pont` binary file into your preferred local folder. For example, place it in the `~/bin/pont` folder on macOS to make it accessible from the command line.

   :::tip Upgrading? Remember to look in the default download folder
   When you update the CLI binary with the latest version, note that the newer version binary will be downloaded to your default Downloads folder. Remember to move this newer version binary from the Downloads folder to the `~/bin/pont` folder to update and overwrite the older version.
:::

1. Make this `~/bin/pont` an executable by running this command: 
   - `chmod +x ~/bin/pont`.
   - On macOS when you attempt to run the `pont` tool for the first time, you will be blocked by the macOS that this app is from an "unknown developer". This is normal. Follow the simple steps recommended by the Apple support in [Open a Mac app from an unidentified developer](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unidentified-developer-mh40616/mac) to remove this blocker.
1. Type `~/bin/pont help` to read help instructions.
1. Add `~/bin` to your path in your `.bashrc` or `.zshrc` file for future use.

</details>

<details>
<summary>Linux</summary>

### Linux
:::tip
These instructions have been tested on Ubuntu 20.04.
:::

1. Go to the [Pont CLI release page](https://github.com/aptos-labs/pont-core/releases?q=cli&expanded=true).
1. Click the **Assets** expandable menu for the latest release. 
1. You will see the zip files with the filename of the format: `pont-cli-<version>-<platform>`. These are the platform-specific pre-compiled binaries of the CLI. Download the zip file for your platform, dismissing any warnings.
1. Unzip the downloaded file. This will extract the `pont` CLI binary file into your default downloads folder. 
1. Move this extracted `pont` binary file into your preferred local folder. 

   :::tip Upgrading? Remember to look in the default download folder
   When you update the CLI binary with the latest version, note that the newer version binary will be downloaded to your default Downloads folder. Remember to move this newer version binary from the Downloads folder to `~/bin/pont` folder (overwriting the older version).
   :::

1. Make this `~/bin/pont` an executable by running this command:
    - `chmod +x ~/bin/pont`.
1. Type `~/bin/pont help` to read help instructions.
1. Add `~/bin` to your path in your `.bashrc` or `.zshrc` file for future use.

</details>

<details>
<summary>Windows 10, 11 and Windows Server 2022+</summary>

### Windows 10, 11 and Windows Server 2022+

:::tip
These instructions have been tested on Windows 11 and Windows Server 2022. Windows support is new and some features may be not complete. Open [Github issues](https://github.com/aptos-labs/pont-core/issues) for bugs.
:::

1. Go to the [Pont CLI release page](https://github.com/aptos-labs/pont-core/releases?q=cli&expanded=true).
1. Click the **Assets** expandable menu for the latest release. 
1. You will see the zip files with the filename of the format: `pont-cli-<version>-<platform>`. These are the platform-specific pre-compiled binaries of the CLI. Download the zip file for your platform, dismissing any warnings.
1. Unzip the downloaded file. This will extract the `pont` CLI binary file into your default downloads folder. For example, on Windows it is the `\Users\user\Downloads` folder.
1. Move this extracted `pont` binary file into your preferred local folder.
   :::tip Upgrading? Remember to look in the default download folder
   When you update the CLI binary with the latest version, note that the newer version binary will be downloaded to your default Downloads folder. Remember to move this newer version binary from the Downloads folder to your preferred location.
   :::
1. Open a powershell terminal via the windows start menu
1. In the powershell terminal, you can get help instructions by running the command with help.  For example ` .\Downloads\pont-cli-0.3.5-Windows-x86_64\pont.exe help` to read help instructions.

</details>

## (Optional) Install the dependencies of Move Prover

If you want to use the Move Prover, install the dependencies by following the below steps:

:::tip
Currently, Windows is not supported by the Move Prover.
:::

<details>
<summary>Prover macOS installation</summary>

### macOS

:::tip
These instructions have been tested on macOS Monterey (12.6)
:::

1. Ensure you have `brew` installed https://brew.sh/.
1. Ensure you have `git` installed https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.
1. Clone the Pont core repo:  `git clone https://github.com/aptos-labs/pont-core.git`.
1. Change directory into `pont-core`: `cd pont-core`
1. Run the dev setup script to prepare your environment: `./scripts/dev_setup.sh -yp`
1. Source the profile file: `source ~/.profile`.

   :::info
   Note that you have to include environment variable definitions in `~/.profile` into your shell. Depending on your setup, the  `~/.profile` may be already automatically loaded for each login shell, or it may not. If not, you may
   need to add `. ~/.profile` to your `~/.bash_profile` or other shell configuration manually.
   :::

1. You can now run the Move Prover to prove an example:
    ```bash
    pont move prove --package-dir pont-move/move-examples/hello_prover/
    ```
   
</details>

<details>
<summary>Prover Linux installation</summary>

### Linux

:::tip 
Some Linux distributions are not supported. Currently, OpenSUSE and Amazon Linux do not support the automatic installation via the `dev_setup.sh` script.
:::

1. Ensure you have `git` installed https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.
1. Clone the Pont core repo:  `git clone https://github.com/aptos-labs/pont-core.git`.
1. Change directory into `pont-core`: `cd pont-core`
1. Run the dev setup script to prepare your environment: `./scripts/dev_setup.sh -yp`
1. Source the profile file: `source ~/.profile`.

   :::info
   Note that you have to include environment variable definitions in `~/.profile` into your shell. Depending on your setup, the  `~/.profile` may be already automatically loaded for each login shell, or it may not. If not, you may
   need to add `. ~/.profile` to your `~/.bash_profile` or other shell configuration manually.
   :::

1. You can now run the Move Prover to prove an example:
    ```bash
    pont move prove --package-dir pont-move/move-examples/hello_prover/
    ```

</details>

## (Advanced users only) Build the CLI binary from the source code

If you are an advanced user and would like to build the CLI binary by downloading the source code, follow the below steps. **This is not recommended** unless you are on a platform unsupported by the prebuilt binaries.

<details>
<summary>macOS</summary>

### macOS
#### Setup dependencies

**> Using the automated script**

1. If on Mac, ensure you have `brew` installed https://brew.sh/
1. Ensure you have `git` installed https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.
1. Clone the Pont core repo:  `git clone https://github.com/aptos-labs/pont-core.git`.
1. Change directory into `pont-core`: `cd pont-core`
1. Run the dev setup script to prepare your environment: `./scripts/dev_setup.sh`
1. Update your current shell environment: `source ~/.cargo/env`.

**> Manual installation of dependencies**

If the script above doesn't work for you, you can install these manually, but it's **not recommended**.

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [Git](https://git-scm.com/download)
1. Install [CMake](https://cmake.org/download/)
1. Install [LLVM](https://releases.llvm.org/)

#### Building the Pont CLI

1. Checkout the correct branch `git checkout --track origin/<branch>`, where `<branch>` is:
    - `devnet` for building on the Pont devnet.
    - `testnet` for building on the Pont testnet.
    - `main` for the current development branch.
1. Build the CLI tool: `cargo build --package pont --release`.
1. The binary will be available in at
    - `target/release/pont`
1. (Optional) Move this executable to a place on your path e.g. `~/bin/pont`.
1. You can now get help instructions by running `~/bin/pont help`

</details>

<details>
<summary>Linux</summary>

### Linux
#### Setup dependencies

**> Using the automated script**

1. If on Mac, ensure you have `brew` installed https://brew.sh/
1. Ensure you have `git` installed https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.
1. Clone the Pont core repo:  `git clone https://github.com/aptos-labs/pont-core.git`.
1. Change directory into `pont-core`: `cd pont-core`
1. Run the dev setup script to prepare your environment: `./scripts/dev_setup.sh`
1. Update your current shell environment: `source ~/.cargo/env`

**> Manual installation of dependencies**

If the script above does not work for you, you can install these manually, but it is **not recommended**:

1. [Rust](https://www.rust-lang.org/tools/install).
1. [Git](https://git-scm.com/download).
1. [CMake](https://cmake.org/download/).
1. [LLVM](https://releases.llvm.org/).

#### Building the Pont CLI

1. Checkout the correct branch `git checkout --track origin/<branch>`, where `<branch>` is:
    - `devnet` for building on the Pont devnet.
    - `testnet` for building on the Pont testnet.
    - `main` for the current development branch.
1. Build the CLI tool: `cargo build --package pont --release`.
1. The binary will be available in at
   - `target/release/pont`
1. (Optional) Move this executable to a place on your path e.g. `~/bin/pont`.
1. You can now get help instructions by running `~/bin/pont help`

</details>

<details>
<summary>Windows</summary>

### Windows

#### Setup dependencies

:::tip
The pont-core codebase currently has no script similar to the `dev_setup.sh` script for
Windows.  All dependencies must be manually installed.
:::

**> Manual installation of dependencies**

1. Install [Rust](https://www.rust-lang.org/tools/install).
1. Install [Git](https://git-scm.com/download).
1. Install [CMake](https://cmake.org/download/).
1. If on Windows ARM, install [Visual Studio Preview](https://visualstudio.microsoft.com/vs/preview/).
1. Install [C++ build tools for Windows](https://visualstudio.microsoft.com/downloads/#microsoft-visual-c-redistributable-for-visual-studio-2022).
1. Install [LLVM](https://releases.llvm.org/).

#### Building pont-core

1. Checkout the correct branch `git checkout --track origin/<branch>`, where `<branch>` is:
    - `devnet` for building on the Pont devnet.
    - `testnet` for building on the Pont testnet.
    - `main` for the current development branch.
1. Build the CLI tool: `cargo build --package pont --release`.
1. The binary will be available at - `target\release\pont.exe`
1. You can now get help instructions by running `target\release\pont.exe` in a Powershell window.

</details>
