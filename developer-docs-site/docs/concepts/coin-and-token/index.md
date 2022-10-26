---
title: "Coin and Token"
slug: "index"
---

# Coin and Token

For digital assets, Pont provides two Move modules:

### [Pont Coin](pont-coin)

The [`coin.move`](https://github.com/aptos-labs/pont-core/blob/main/pont-move/framework/pont-framework/sources/coin.move) is a lightweight standard meant for simple, typesafe, and fungible assets. The coin standard is separated out into its own Move module to ensure that:
  - The coin standard can be used to create a token with an emphasis on simplicity and performance and with minimal metadata. 
  - The coin module remains a part of the Pont core framework and be used for currencies, for example the gas currency, thereby enhancing the core functionality of the Pont framework.

See [Pont Coin >](pont-coin)

### [Pont Token](pont-token)

The [`token.move`](https://github.com/aptos-labs/pont-core/blob/main/pont-move/framework/pont-token/sources/token.move) Move module, on the other hand:

- Encapsulates rich, flexible assets, fungible and nonfungible, and collectibles. 
- The token standard is deployed as a separate package at the Pont blockchain address `0x3`. 
- The token standard is designed to create an NFT or a semi-fungible or a fungible non-decimal token, with rich metadata and functionalities. A token definition of this type can be iterated rapidly to respond to the platform and user requirements. 

See [Pont Token >](pont-token)
