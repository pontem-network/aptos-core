---
title: "Local testnet development flow"
slug: "local-testnet-dev-flow"
---

This guide describes the end-to-end flow for developing with a local testnet.

:::caution CLI from source, not from GitHub
This guide is not correct if you're using the `pont` CLI from a GitHub release or from `cargo install`, only if you build it yourself from `pont-core` as described below.
:::

Please read this guide carefully. This guide specifically addresses the local testnet development flow. This flow will not work if you're building against devnet.

## Run a local testnet from `pont-core`

Pull and go into `pont-core`:
```
git clone git@github.com:aptos-labs/pont-core.git ~/pont-core && cd ~/pont-core
```

Run a local testnet:
```
cargo run -p pont -- node run-local-testnet --with-faucet --faucet-port 8081 --force-restart --assume-yes
```
You may add the `--release` flag after `cargo run` if you want to build a release version of the CLI for running the local testnet.

You are now running a local testnet built from `pont-core` main.

## Typescript: Use the SDK from `pont-core`
**Important**: With this development flow, it is essential that you do not use the SDK from npmjs. Instead, you must use the same SDK as the `pont` CLI is built from, which we describe below.

This guide assumes you have done the previous local testnet step. We also assume you have `yarn` installed.

First, go into `pont-core` and build the SDK:
```
cd ~/pont-core/ecosystem/typescript/sdk
yarn install
yarn build
```

Make a new project if you don't have one already:
```
mkdir ~/project && cd ~/project
yarn init
```

Make your project target the SDK from your local `pont-core`:
```
yarn add ../pont-core/ecosystem/typescript/sdk
```
You could also use the absolute path, e.g. `/home/daniel/pont-core/ecosystem/typescript/sdk`.

Install everything:
```
yarn install
```

Now you're set up! You should see in `package.json` that your project targets your local `pont-core`:
```json
{
  "name": "project",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT",
  "dependencies": {
    "pont": "../a/core/ecosystem/typescript/sdk/"
  }
}
```

This way your local testnet and the SDK you're using match, meaning you won't see any compatibility issues.

Now you can use the Pont module like this in your code:
```ts
import { PontClient, PontAccount, FaucetClient } from "pont";

const NODE_URL = "http://127.0.0.1:8080/v1";
const FAUCET_URL = "http://127.0.0.1:8081";

(async () => {
  const client = new PontClient(NODE_URL);
  const faucetClient = new FaucetClient(NODE_URL, FAUCET_URL);
})();
```

**Note**: See that this code builds clients that talk to your local testnet, not devnet.
