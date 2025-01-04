use crate::{
    bundle::BundleHash,
    jsonrpc::{JsonRpcError, Request, Response},
};
use ethers::core::{
    types::{H256, U64},
    utils::keccak256,
};
use ethers::signers::Signer;
use reqwest::{header::HeaderValue, Client, ClientBuilder, Error as ReqwestError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;
use url::Url;

/// A Flashbots relay client.
///
/// The client automatically signs every request and sets the Flashbots
/// authorization header appropriately with the given signer.
///
/// **Note**: You probably do not want to use this directly, unless
/// you want to interact directly with the Relay. Most users should use
/// [`FlashbotsMiddleware`](crate::FlashbotsMiddleware) instead.
#[derive(Debug)]
pub struct Relay<S> {
    id: AtomicU64,
    client: Client,
    url: Url,
    signer: Option<S>,
}

/// Errors for relay requests.
#[derive(Error, Debug)]
pub enum RelayError {
    #[error("Client error: {text}")]
    ClientError { text: String },
    #[error("Failed to send request")]
    RequestError(#[from] ReqwestError),
    #[error("Error response: {text}")]
    ResponseSerdeJson {
        text: String,
    },
}

impl<S: Signer> Relay<S> {
    /// Initializes a new relay client.
    pub fn new(url: impl Into<Url>, signer: Option<S>, authorization_key: String) -> Self {
        let client_builder = ClientBuilder::new();
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&authorization_key).unwrap(),
        );
        
        let client_builder = client_builder.default_headers(headers);
        
        Self {
            id: AtomicU64::new(0),
            client: client_builder.build().unwrap(),
            url: url.into(),
            signer,
        }
    }

    /// Sends a request with the provided method to the relay, with the
    /// parameters serialized as JSON.
    pub async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<Option<R>, RelayError> {
        let next_id = self.id.load(Ordering::SeqCst) + 1;
        self.id.store(next_id, Ordering::SeqCst);

        let payload = Request::new(next_id, method, params);

        let mut req = self.client.post(self.url.as_ref());

        let res = req.json(&payload).send().await?;
        let status = res.error_for_status_ref();

        match status {
            Err(err) => {
                let text = res.text().await?;
                let status_code = err.status().unwrap();
                if status_code.is_client_error() {
                    // Client error (400-499)
                    Err(RelayError::ClientError { text })
                } else {
                    // Internal server error (500-599)
                    Err(RelayError::RequestError(err))
                }
            }
            Ok(_) => {
                let text = res.text().await?;
                let res: Response<R> = serde_json::from_str(&text).unwrap();
                let result: Result<Option<R>, JsonRpcError> = res.data.into_result();
                
                match result {
                    Ok(result) => Ok(result),
                    Err(err) => Err(RelayError::ResponseSerdeJson {
                        text: format!("{:?}", err),
                    }),
                }
            }
        }
    }
}

impl<S: Signer + Clone> Clone for Relay<S> {
    fn clone(&self) -> Self {
        Self {
            id: AtomicU64::new(0),
            client: self.client.clone(),
            url: self.url.clone(),
            signer: self.signer.clone(),
        }
    }
}