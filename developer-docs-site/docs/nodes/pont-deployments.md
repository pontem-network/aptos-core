---
title: "Pont Blockchain Deployments"
slug: "pont-deployments"
hide_table_of_contents: true
---

# Pont Blockchain Deployments

You can connect to the Pont blockchain in a few ways. See the below table:

:::tip What not to do

Make sure to see the row describing [**What not to do**](#what-not-to-do).

:::

|Description | Mainnet | Devnet | Long-lived Testnet | Pont Incentivized Testnet (AIT)|
|---|---|---|---|---|
|<div style={{width: 120}}>**Chain ID**</div>| 1 |[On Pont Explorer **select Devnet from top right**](https://explorer.pontlabs.com/?network=Devnet).| 2|  Available during AIT program.|
|**REST API URL**| <div style={{width: 220}}>https://fullnode.mainnet.pontlabs.com/v1</div> |<div style={{width: 220}}>https://fullnode.devnet.pontlabs.com/v1</div> | <div style={{width: 220}}>https://fullnode.testnet.pontlabs.com/v1</div> | <div style={{width: 110}}>Available during AIT program. </div>|
|**Genesis blob and Waypoint**| In the `mainnet` directory on https://github.com/aptos-labs/pont-networks |In the `devnet` directory on https://github.com/aptos-labs/pont-networks  | <div style={{width: 200}}>In the `testnet` directory on https://github.com/aptos-labs/pont-networks </div>| Available during AIT program.  |
|**Faucet**| No Faucet |<div style={{width: 200}}>https://faucet.devnet.pontlabs.com/</div> | <div style={{width: 200}}>(dApp): https://pontlabs.com/testnet-faucet </div>|Available during AIT program.|
|**Epoch**| 7200 seconds (two hours, set by governance) |--- | 7200 seconds (two hours) |Available during AIT program.|
|**Network runs where**| Validators, validator fullnodes and public fullnodes are run by you (i.e., the Pont community) and Pont Labs. |<div style={{width: 200}}>Validators run on Pont Labs servers. Fullnodes are run by both Pont Labs and you (i.e., the Pont community).</div>|<div style={{width: 200}}>Validators run on Pont Labs servers. Fullnodes are run by both Pont Labs and you (i.e., the Pont community).</div> | Some Validators run on Pont servers, others are run by the Pont community. Fullnodes are run by Pont Labs and the community.|
|**Who is responsible for the network**| Fully decentralized. |Managed by Pont Team. | Managed by Pont Team. | Managed by Pont Labs and the community.|
|**Update release cadence**| Every month. |Every week. |Every 2 weeks. | Managed by Pont Labs and the community.|
|**How often wiped**| Not wiped. |Every week.| Not wiped. | Wiped permanently after AIT program concludes.|
|***Description*** | ***Mainnet*** | ***Devnet*** | ***Long-lived Testnet*** |  ***AIT***|
|**Purpose of the network**| The main Pont network. |The devnet is built to experiment with new ideas, improve performance and enhance the user experience.| | For executing the Pont Incentivized Testnet programs for the community.|
|**Network status**| Always live. |Mostly live, with brief interruptions during regular updates. |Mostly live, with brief interruptions during regular updates. | Live only during Incentivized Testnet drives. |
|**Type of nodes** |Validators and validator fullnodes. |Validators and public fullnodes. | Validators and public fullnodes. | Validators and validator fullnodes.|
|**How to run a node**| See [Validators](/nodes/validator-node/validators) and [Public Fullnode](/nodes/full-node/public-fullnode) sections.  |N/A, run by Pont Labs team. |See [Validators](/nodes/validator-node/validators) and [Public Fullnode](/nodes/full-node/public-fullnode) sections. | See the node deployment guides published during AIT program.|
|<span id="what-not-to-do">**What not to do**</span>||Do not attempt to sync your local AIT fullnode or AIT validator node with devnet. | |Make sure you deploy your local AIT fullnode, AIT validator node and AIT validator fullnode in the test mode, and follow the instructions in the node deployment guides published during AIT program.|

