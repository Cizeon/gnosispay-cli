use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct GnosisTransaction {
    pub created_at: String,
    pub cleared_at: Option<String>,
    pub is_pending: bool,
    pub transaction_amount: String,
    pub transaction_currency: Currency,
    pub billing_amount: String,
    pub billing_currency: Currency,
    pub mcc: String,
    pub merchant: Merchant,
    pub country: Country,
    pub transactions: Vec<Transaction>,
    pub kind: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Currency {
    pub symbol: String,
    pub code: String,
    pub decimals: u32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Merchant {
    pub name: String,
    pub city: String,
    pub country: Country,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Country {
    pub name: String,
    pub numeric: String,
    pub alpha2: String,
    pub alpha3: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Transaction {
    pub status: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub hash: String,
}

pub struct GnosisPay {
    base_url: String,
    session_token: String,
}

impl GnosisPay {
    pub fn new(session_token: String) -> Self {
        Self {
            base_url: String::from("https://app.gnosispay.com"),
            session_token,
        }
    }

    /// Retrieve all transactions from GnosisPay.
    pub async fn retrieve_transactions(&self) -> Result<Vec<GnosisTransaction>> {
        let url = format!("{}/api/v1/transactions", self.base_url);

        let client = reqwest::Client::new();

        let cookie = format!(
            "__Secure-authjs.session-token=\"{}\"",
            self.session_token.clone()
        );

        let response = client
            .get(&url)
            .header("Cookie", cookie.as_str())
            .send()
            .await?;

        let tx_response = match response.status() {
            reqwest::StatusCode::OK => match response.json::<Vec<GnosisTransaction>>().await {
                Ok(tx_response) => tx_response,
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err("Invalid response".into());
                }
            },
            _ => {
                return Err("Request failed".into());
            }
        };

        Ok(tx_response)
    }
}
