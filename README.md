# Txboost Relay Client Wrapper

A Rust-based library for interacting with the [txboost.io](https://txboost.io) relay.

## What is txboost.io?

[txboost.io](https://txboost.io) is a transaction relay service designed to optimize and enhance transaction execution on blockchain networks. It provides a decentralized infrastructure to reduce MEV risks and introduces transaction ordering.

## Why Use txboost.io on the BNB Chain?

The BNB Chain lacks a centralized sequencer, a feature present in some other blockchains to enforce fair ordering of transactions. 
Without a centralized sequencer, BNB Chain is susceptible to MEV sandwiches.

Using txboost.io on the BNB provides the following advantages:
- **Decentralized Protection**: Mitigates MEV risks without relying on centralized intermediaries.
- **Fair Transaction Ordering**: Simulate your transactions and leverage transaction atomicity. You get all or you get nothing.

## Features of This Library

- **Simple API Integration**: Easily interact with txboost.io's relay service using intuitive Rust methods.
- **Lightweight and Efficient**: Built with Rust's performance and safety guarantees.
- **Configurable Options**: Adjust parameters to suit your application's specific needs.

## Getting Started

### Prerequisites
- API key for txboost.io (sign up [here](https://txboost.io))

### Installation
Add this library to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
txboost-rs = "0.1.0"
```

## License
MIT
