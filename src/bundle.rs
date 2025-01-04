use std::default;
use ethers::types::{BlockNumber, H256, U64};
use serde::{de, Deserialize, Deserializer, Serialize};

pub type BundleHash = H256;

#[derive(Serialize, Debug, Clone)]
pub struct SendBundleRequest {
    pub txs: Vec<BundleHash>,
    #[serde(rename = "blockNumber")]
    pub block_number: U64,
    #[serde(rename = "minTimestamp", skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<u64>,
    #[serde(rename = "maxTimestamp", skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<u64>,
    #[serde(rename = "revertingTxHashes", skip_serializing_if = "Option::is_none")]
    pub reverting_tx_hashes: Option<Vec<BundleHash>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SimulateBundleRequest {
    pub txs: Vec<BundleHash>,
    #[serde(rename = "blockNumber")]
    pub block_number: U64,
    #[serde(rename = "stateBlockNumber")]
    pub state_block_number: StateBlockNumber,
    pub timestamp: Option<u64>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub enum StateBlockNumber {
    Number(U64),
    #[default]
    Latest
}

impl <'de> Deserialize<'de> for StateBlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        
        if s == "latest" {
            Ok(StateBlockNumber::Latest)
        } else {
            U64::from_str_radix(&s, 16)
                .map(StateBlockNumber::Number)
                .map_err(de::Error::custom)
        }
    }
}

impl Default for SendBundleRequest {
    fn default() -> Self {
        Self {
            txs: Vec::new(),
            block_number: U64::zero(),
            min_timestamp: None,
            max_timestamp: None,
            reverting_tx_hashes: None,
        }
    }
}

impl SendBundleRequest {
    pub fn new(txs: Vec<BundleHash>, block_number: U64) -> Self {
        Self {
            txs,
            block_number,
            ..Default::default()
        }
    }

    pub fn with_min_timestamp(mut self, min_timestamp: u64) -> Self {
        self.min_timestamp = Some(min_timestamp);
        self
    }

    pub fn with_max_timestamp(mut self, max_timestamp: u64) -> Self {
        self.max_timestamp = Some(max_timestamp);
        self
    }

    pub fn with_reverting_tx_hashes(mut self, reverting_tx_hashes: Vec<BundleHash>) -> Self {
        self.reverting_tx_hashes = Some(reverting_tx_hashes);
        self
    }
}

impl SimulateBundleRequest {
    pub fn new(txs: Vec<BundleHash>, block_number: U64, state_block_number: StateBlockNumber) -> Self {
        Self {
            txs,
            block_number,
            state_block_number,
            timestamp: None,
        }
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    pub fn block(&self) -> U64 {
        self.block_number
    }
    
    pub fn state_block_number(mut self, state_block_number: StateBlockNumber) -> Self {
        self.state_block_number = state_block_number;
        self
    }
}