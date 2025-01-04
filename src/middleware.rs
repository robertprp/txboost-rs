use crate::{
    bundle::{SendBundleRequest, SimulateBundleRequest, StateBlockNumber},
    relay::{Relay, RelayError},
};
use ethers::{
    core::types::U64,
    providers::{Middleware, MiddlewareError},
    signers::Signer,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TxBoostMiddlewareError<M: Middleware> {
    /// The relay responded with an error.
    #[error(transparent)]
    RelayError(#[from] RelayError),
    #[error("{0}")]
    MiddlewareError(M::Error),
    #[error("Failed to simulate bundle")]
    BundleSimError,
}

impl<M: Middleware> MiddlewareError for TxBoostMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> TxBoostMiddlewareError<M> {
        TxBoostMiddlewareError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            TxBoostMiddlewareError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TxBoostMiddleware<M, S> {
    inner: M,
    relay: Relay<S>,
    simulation_relay: Option<Relay<S>>,
}

impl<M: Middleware, S: Signer> TxBoostMiddleware<M, S> {
    pub fn new(inner: M, relay: Relay<S>, simulation_relay: Option<Relay<S>>) -> Self {
        Self {
            inner,
            relay,
            simulation_relay,
        }
    }

    pub fn relay(&self) -> &Relay<S> {
        &self.relay
    }

    pub fn simulation_relay(&self) -> Option<&Relay<S>> {
        self.simulation_relay.as_ref()
    }

    pub fn into_inner(self) -> M {
        self.inner
    }

    pub async fn simulate_bundle(
        &self,
        bundle: &SimulateBundleRequest,
    ) -> Result<SimulatedBundleResponse, TxBoostMiddlewareError<M>> {
        self.simulation_relay
            .as_ref()
            .unwrap_or(&self.relay)
            .request("eth_callBundle", [bundle])
            .await
            .map_err(TxBoostMiddlewareError::RelayError)?
            .ok_or(TxBoostMiddlewareError::BundleSimError)
    }

    pub async fn send_bundle(
        &self,
        bundle: &SendBundleRequest,
    ) -> Result<SendBundleResponse, TxBoostMiddlewareError<M>> {
        self.relay
            .request("eth_sendBundle", [bundle])
            .await
            .map_err(TxBoostMiddlewareError::RelayError)?
            .ok_or(TxBoostMiddlewareError::BundleSimError)
    }
}

fn de_from_str<'de, D>(deserializer: D) -> Result<U64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    U64::from_str_radix(&s, 16).map_err(de::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedBundleResponse {
    txs: Vec<String>,
    #[serde(deserialize_with = "de_from_str")]
    block_number: U64,
    state_block_number: StateBlockNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBundleResponse {
    pub bundle_gas_price: String,
    pub bundle_hash: String,
    pub coinbase_diff: String,
    pub eth_sent_to_coinbase: String,
    pub gas_fees: String,
    pub results: Vec<SendBundleResult>,
    pub state_block_number: u64,
    pub total_gas_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBundleResult {
    pub bundle_hash: String,
    pub coinbase_diff: String,
    pub eth_sent_to_coinbase: String,
    pub gas_fees: String,
    pub state_block_number: u64,
    pub total_gas_used: u64,
}
