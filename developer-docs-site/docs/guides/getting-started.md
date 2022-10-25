---
title: "Getting Started"
slug: "getting-started"
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Getting Started

To kickstart your journey as a developer in the Pont ecosystem, set up your development environment as described in this section. See [Installing Pont CLI](../cli-tools/pont-cli-tool/install-pont-cli.md) for the supported operating systems.

## Clone the Pont-core repo

Start by cloning the `pont-core` GitHub repo from [GitHub](https://github.com/aptos-labs/pont-core).

1. Clone the Pont repo.

      ```
      git clone https://github.com/aptos-labs/pont-core.git
      ```

2. `cd` into `pont-core` directory.

    ```
    cd pont-core
    ```

3. Run the `scripts/dev_setup.sh` Bash script as shown below. This will prepare your developer environment by installing most of the dependencies needed to build, test and inspect Pont Core. Note, you may be prompted for your password:

    ```
    ./scripts/dev_setup.sh
    ```
    :::tip
    You can see the available options for the script by running `./scripts/dev_setup.sh --help`
    :::

4. Update your current shell environment to run `cargo build` and other Pont-related commands:

    ```
    source ~/.cargo/env
    ```

5. Optionally, check out a release branch to install an Pont node:

    <Tabs groupId="network">
    <TabItem value="devnet" label="Devnet">

    Check out the `devnet` branch using:

    ```
    git checkout --track origin/devnet
    ```
    </TabItem>
    <TabItem value="testnet" label="Testnet" default>

    Check out the `testnet` branch using:

    ```
    git checkout --track origin/testnet-stable
    ```
    </TabItem>
    </Tabs>

## Install the CLI

Install the Pont CLI following the [Installing Pont CLI](/cli-tools/pont-cli-tool/install-pont-cli.md) guide. 

## Install tools for macOS

Make sure you have the below tools installed on your computer if running macOS. You will need them for running the [Developer Tutorials](/docs/tutorials/index.md), in the order specified:

- **Homebrew**: [https://brew.sh/](https://brew.sh/)
- **Node.js**: Install [Node.js](https://nodejs.org/en/download/), which will install `npm` and `npx`, by executing the below command on your Terminal:
    ```bash
    brew install node
    ```
- **Yarn**: Install the latest [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable) by executing the below command on your Terminal:
    ```bash
    brew install yarn
    ```
- **Poetry**: Install Poetry from [https://python-poetry.org/docs/#installation](https://python-poetry.org/docs/#installation).

Now your basic Pont development environment is ready.

## Pont developer resources

This section contains links to frequently referred Pont developer resources. 

### Pont Explorer

- [Pont Explorer](https://explorer.pontlabs.com/): Use the top-right drop-down menu to select the network.
- [Pont Community](https://pontlabs.com/community): Links to discussion forum, Discord and AIT.

### Pont testnet

- **REST API Open API spec**: [https://fullnode.testnet.pontlabs.com/v1/spec#/](https://fullnode.testnet.pontlabs.com/v1/spec#/)
- **REST service:** [https://fullnode.testnet.pontlabs.com/v1](https://fullnode.testnet.pontlabs.com/v1)
- **Faucet service:** [https://faucet.testnet.pontlabs.com](https://faucet.testnet.pontlabs.com)
- **Genesis**: [https://testnet.pontlabs.com/genesis.blob](https://testnet.pontlabs.com/genesis.blob)
- **Genesis and waypoint**: [https://github.com/aptos-labs/pont-genesis-waypoint/tree/main/testnet](https://github.com/aptos-labs/pont-genesis-waypoint/tree/main/testnet)
- **ChainID**: [Click here to see it on the Pont Explorer](https://explorer.pontlabs.com/?network=testnet).

### Pont devnet

- **REST API Open API spec**: [https://fullnode.devnet.pontlabs.com/v1/spec#/](https://fullnode.devnet.pontlabs.com/v1/spec#/)
- **REST service:** [https://fullnode.devnet.pontlabs.com/v1](https://fullnode.devnet.pontlabs.com/v1)
- **Faucet service:** [https://faucet.devnet.pontlabs.com](https://faucet.devnet.pontlabs.com)
- **Genesis**: [https://devnet.pontlabs.com/genesis.blob](https://devnet.pontlabs.com/genesis.blob)
- **Waypoint**: [https://devnet.pontlabs.com/waypoint.txt](https://devnet.pontlabs.com/waypoint.txt)
- **ChainID**: [Click here to see it on the Pont Explorer](https://explorer.pontlabs.com/?network=devnet).

### Pont CLI

- [Pont CLI releases](https://github.com/aptos-labs/pont-core/releases?q=cli&expanded=true)
- [Pont CLI Documentation](/cli-tools/pont-cli-tool/pont-cli-index)

### Pont SDK

- [Typescript SDK](https://www.npmjs.com/package/pont)
- [Python SDK](https://pypi.org/project/pont-sdk/)
- [Rust SDK](/sdks/rust-sdk.md)

### IDE plugins for Move language

- [Syntax hightlighting for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=damirka.move-syntax)
- [Move analyzer for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer): Supports advanced code navigation and syntax highlighting.
- [Move language plugin for Jetbrains IDEs](https://plugins.jetbrains.com/plugin/14721-move-language): Supports syntax highlighting, code navigation, renames, formatting, type checks and code generation.
