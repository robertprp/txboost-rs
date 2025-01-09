# Txboost Relay Client Wrapper

A Rust-based library for interacting with the [txboost.com](https://txboost.com) relay.

## What is txboost.com?

[txboost.com](https://txboost.com) is a transaction relay service designed to optimize and enhance transaction execution on Mainnet (free) or BNB. It provides infrastructure to reduce MEV risks and introduces transaction ordering.

## Why Use txboost.com on the BNB Chain?

The BNB Chain lacks a centralized sequencer, a feature present in some other blockchains to enforce fair ordering of transactions. 
Without a centralized sequencer, BNB Chain is susceptible to MEV sandwiches.

The diff between BNB and other roll-ups like optimism or base is that they relay on a single master centralized sequencer, disallowing the access to mempool and thus, pending transactions.

Using txboost.com on the BNB provides the following advantages:
- **Decentralized Protection**: Mitigates MEV risks without relying on centralized intermediaries.
- **Fair Transaction Ordering**: Simulate your transactions and leverage transaction atomicity. You get all or you get nothing.

## Features of This Library

- **Simple API Integration**: Easily interact with txboost.com's relay service using intuitive Rust methods.
- **Lightweight and Efficient**: Built with Rust's performance and safety guarantees.
- **Configurable Options**: Adjust parameters to suit your application's specific needs.

## Getting Started

### Prerequisites
- API key for txboost.com (sign up [here](https://txboost.com))

### Installation
Add this library to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
txboost-rs = "0.1.0"
```

## License
MIT
