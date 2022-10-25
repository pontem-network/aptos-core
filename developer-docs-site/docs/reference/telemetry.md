---
title: "Telemetry"
slug: "telemetry"
---

At Pont Labs, we develop software and services for the greater Pont community and ecosystem. On top of community feedback, we use telemetry to help improve the decentralization of the network by understanding how our software is being deployed and run.

The Pont node binary collects telemetry such as software version, operating system information, and IP address. See [Types of information collected](#types-of-information-collected).

The Pont node binary does **not** collect personal information such as usernames or email addresses.

Users can disable telemetry at any point. If telemetry remains enabled, Pont node binary will send telemetry data in the background.

# Disabling telemetry

On macOS and Linux, you can disable telemetry by setting the `APTOS_DISABLE_TELEMETRY` environment variable to any value.

```
export APTOS_DISABLE_TELEMETRY=true
```

The above example only disables telemetry for a single session or terminal. To disable it everywhere, you must do so at shell startup.

```
echo "export APTOS_DISABLE_TELEMETRY=true" >> ~/.profile
source ~/.profile
```

# Types of information collected

* **Pont node information**, e.g., public IP address and core metrics (including node type, synced version and number of network connections).
* **Pont tooling usage**, e.g., the commands and subcommands run by the Pont CLI tool.
* **Rust build information**, e.g., the rust version, cargo version, build target architecture and build tag.
* **System information**, e.g., operating system information (including versions), hardware information and resource utilization (including CPU, memory and disk).
