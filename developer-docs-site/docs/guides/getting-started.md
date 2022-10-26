---
title: "Getting Started"
slug: "getting-started"
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Getting Started

To kickstart your journey in the Pont ecosystem, set up your environment as needed by your role. To interact with Pont, you may simply [install the Pont command line interface (CLI)](#install-the-cli). To develop Pont itself, you will need to [clone the Pont-core repository](#clone-the-Pont-core-repo).

See the [Workflows](#workflows) for use cases associated with each path. See the [Pont developer resources](#pont-developer-resources) for quick links to Pont networks, SDKs, and other tools.

## Workflows

Most Pont users will want to have the Pont CLI installed. [Install](../cli-tools/pont-cli-tool/install-pont-cli.md) and [use](../cli-tools/pont-cli-tool/use-pont-cli.md) the Pont CLI if you will:

* [Run a local testnet](../nodes/local-testnet/using-cli-to-run-a-local-testnet.md).
* [Manage accounts](../cli-tools/pont-cli-tool/use-pont-cli.md#account-examples).
* [Generate keys](../cli-tools/pont-cli-tool/use-pont-cli.md#key-examples).
* [Compile Move packages](../cli-tools/pont-cli-tool/use-pont-cli.md#move-examples).

In addition to installing the CLI, [clone](#clone-the-pont-core-repo) and [review](https://github.com/aptos-labs/pont-core) the Pont repository if you will:

* [Run a fullnode](../nodes/full-node/index.md).
* [Run a validator node](../nodes/validator-node/index.md).
* [Take the starter tutorials](../tutorials/index.md), many of which rely upon Pont source code.
* [Extend and contribute](https://github.com/aptos-labs/pont-core) to the Pont blockchain itself; [pull requests](https://github.com/aptos-labs/pont-core/pulls) welcome!

Although Docker options exist for many of these configurations, you should download the Pont source to become familiar with the innerworkings of the blockchain once you are conducting this more advanced work.

:::tip Find information in the source
The [Pont source files](https://github.com/aptos-labs/pont-core) themselves also contain a wealth of information in docs comments worth reviewing.
:::

## Install the CLI

[Install Pont CLI](../cli-tools/pont-cli-tool/install-pont-cli.md) to interact with the Pont network. As a developer in the Pont ecosystem, set up your development environment as described in the link. See [Installing Pont CLI](../cli-tools/pont-cli-tool/install-pont-cli.md) for the supported operating systems.

## Clone the Pont-core repo

As described in [Workflows](#workflows), you may interact with Pont using only the CLI. If you need the source, clone the `pont-core` GitHub repo from [GitHub](https://github.com/aptos-labs/pont-core).

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
    git checkout --track origin/testnet
    ```
    </TabItem>
    </Tabs>

Now your basic Pont development environment is ready.

## Pont developer resources

This section contains links to frequently referred Pont developer resources. 

### Pont Explorer

- [Pont Explorer](https://explorer.pontlabs.com/): Use the top-right drop-down menu to select the network.
- [Pont Community](https://pontlabs.com/community): Links to discussion forum, Discord and AIT.


### Pont mainnet

- **REST API Open API spec**: [https://fullnode.mainnet.pontlabs.com/v1/spec#/](https://fullnode.mainnet.pontlabs.com/v1/spec#/)
- **REST service:** [https://fullnode.mainnet.pontlabs.com/v1](https://fullnode.mainnet.pontlabs.com/v1)
- **Genesis and waypoint**: [https://github.com/aptos-labs/pont-networks/tree/main/mainnet](https://github.com/aptos-labs/pont-networks/tree/main/mainnet)
- **ChainID**: [Click here to see it on the Pont Explorer](https://explorer.pontlabs.com/?network=mainnet).

### Pont testnet

- **REST API Open API spec**: [https://fullnode.testnet.pontlabs.com/v1/spec#/](https://fullnode.testnet.pontlabs.com/v1/spec#/)
- **REST service:** [https://fullnode.testnet.pontlabs.com/v1](https://fullnode.testnet.pontlabs.com/v1)
- **Faucet dApp:** [https://pontlabs.com/testnet-faucet](https://pontlabs.com/testnet-faucet)
- **Genesis and waypoint**: [https://github.com/aptos-labs/pont-genesis-waypoint/tree/main/testnet](https://github.com/aptos-labs/pont-genesis-waypoint/tree/main/testnet)
- **ChainID**: [Click here to see it on the Pont Explorer](https://explorer.pontlabs.com/?network=testnet).

### Pont devnet

- **REST API Open API spec**: [https://fullnode.devnet.pontlabs.com/v1/spec#/](https://fullnode.devnet.pontlabs.com/v1/spec#/)
- **REST service:** [https://fullnode.devnet.pontlabs.com/v1](https://fullnode.devnet.pontlabs.com/v1)
- **Faucet service:** [https://faucet.devnet.pontlabs.com](https://faucet.devnet.pontlabs.com)
- **Genesis and waypoint**: [https://github.com/aptos-labs/pont-networks/tree/main/devnet](https://github.com/aptos-labs/pont-networks/tree/main/devnet)
- **ChainID**: [Click here to see it on the Pont Explorer](https://explorer.pontlabs.com/?network=devnet).

### Pont CLI

- [Pont CLI releases](https://github.com/aptos-labs/pont-core/releases?q=cli&expanded=true)
- [Pont CLI documentation](/cli-tools/pont-cli-tool/pont-cli-index)

### Pont SDK

- [Typescript SDK](../sdks/ts-sdk/index.md)
- [Python SDK](../sdks/python-sdk.md)
- [Rust SDK](../sdks/rust-sdk.md)

### IDE plugins for Move language

- [Syntax hightlighting for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=damirka.move-syntax)
- [Move analyzer for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=move.move-analyzer): Supports advanced code navigation and syntax highlighting.
- [Move language plugin for Jetbrains IDEs](https://plugins.jetbrains.com/plugin/14721-move-language): Supports syntax highlighting, code navigation, renames, formatting, type checks and code generation.
