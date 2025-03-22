extern crate serde;
extern crate serde_json;
use crate::monerium::EURE_V2_ADDRESS;
use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub result: Option<Vec<TokenTxResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TokenTxResponse {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub hash: String,
    pub nonce: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "tokenDecimal")]
    pub token_decimal: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    pub input: String,
    pub confirmations: String,
}

pub struct Gnosisscan {
    base_url: String,
    api_key: String,
}

impl Gnosisscan {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: String::from("https://api.gnosisscan.io"),
            api_key,
        }
    }

    /// Retrieve all transactions from a wallet address to EURe V2.
    pub async fn retrieve_eure_transactions(&self, wallet_address: String) -> Result<Response> {
        let url = format!(
            "{}/api?module=account&action=tokentx&address={}&contractaddress={}&sort=asc&apikey={}",
            self.base_url, wallet_address, EURE_V2_ADDRESS, self.api_key
        );

        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;

        let tokentx_response = match response.status() {
            reqwest::StatusCode::OK => match response.json::<Response>().await {
                Ok(tokentx_response) => tokentx_response,
                Err(err) => {
                    eprintln!("{}", err);
                    return Err("Invalid response".into());
                }
            },
            _ => {
                return Err("Request failed".into());
            }
        };

        Ok(tokentx_response)
    }
}
