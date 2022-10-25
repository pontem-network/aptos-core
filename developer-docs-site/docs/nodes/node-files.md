---
title: "Node Files"
slug: "node-files"
---

# Node Files

When you are deploying an Pont node, you will need the following files. These can be downloaded from separate `aptos-labs` repos on GitHub. The `wget` commands provided below will work on macOS and Linux. Open a terminal and paste the `wget` command to download the file. 

:::tip Unless specified, all these files are required for validator node.
:::

## docker-compose.yaml

- **Git repo:** `pont-core`
- **Git branch:** `mainnet` on https://github.com/aptos-labs/pont-core
- **Command to download:**
    ```bash
    wget -O docker-compose.yaml https://raw.githubusercontent.com/aptos-labs/pont-core/mainnet/docker/compose/pont-node/docker-compose.yaml
    ```

## validator.yaml

- **Git repo:** `pont-core`
- **Git branch:** `mainnet` on https://github.com/aptos-labs/pont-core
- **Command to download:**
  ```bash
  wget -O validator.yaml https://raw.githubusercontent.com/aptos-labs/pont-core/mainnet/docker/compose/pont-node/validator.yaml
  ```

## genesis.blob 

- **Git repo:** `pont-genesis-waypoint`
- **Git branch:** `main` on https://github.com/aptos-labs/pont-genesis-waypoint
- **Command to download:**
  ```bash
  wget -O genesis.blob https://raw.githubusercontent.com/aptos-labs/pont-genesis-waypoint/main/premainnet/genesis.blob
  ```

## waypoint.txt

- **Git repo:** `pont-genesis-waypoint`
- **Git branch:** `main` on https://github.com/aptos-labs/pont-genesis-waypoint
- **Command to download:**
  ```bash
  wget -O waypoint.txt https://raw.githubusercontent.com/aptos-labs/pont-genesis-waypoint/main/premainnet/waypoint.txt
  ```

## blocked.ips 

- **Git repo:** `pont-core`
- **Git branch:** `mainnet` on https://github.com/aptos-labs/pont-core
- **Command to download:**
  ```bash
  wget -O blocked.ips https://raw.githubusercontent.com/aptos-labs/pont-core/mainnet/docker/compose/pont-node/blocked.ips
  ```

## docker-compose-fullnode.yaml (fullnode only)

:::tip Fullnode 
Fullnode means either a validator fullnode or a public fullnode.
:::

- **Git repo:** `pont-core`
- **Git branch:** `mainnet` on https://github.com/aptos-labs/pont-core
- **Command to download:**
  ```bash
  wget -O docker-compose.yaml https://raw.githubusercontent.com/aptos-labs/pont-core/mainnet/docker/compose/pont-node/docker-compose-fullnode.yaml
  ```

## fullnode.yaml (fullnode only)

:::tip Fullnode 
Fullnode means either a validator fullnode or a public fullnode.
:::

- **Git repo:** `pont-core`
- **Git branch:** `mainnet` on https://github.com/aptos-labs/pont-core
- **Command to download:**
  ```bash
  wget -O fullnode.yaml https://raw.githubusercontent.com/aptos-labs/pont-core/mainnet/docker/compose/pont-node/fullnode.yaml
  ```
