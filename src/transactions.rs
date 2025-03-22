use crate::gnosispay::GnosisTransaction;
use crate::gnosisscan::Response;
use crate::monerium::GNOSIS_BANK;
use crate::prelude::*;
use alloy::primitives::{Address, U256, utils::format_units};
use chrono::{DateTime, Utc};
use colored::Colorize;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub date: DateTime<Utc>,
    pub amount: f64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Transactions {
    pub wallet_address: Address,
    pub transactions: HashMap<String, Transaction>,
}

impl Transactions {
    pub fn new(wallet_address: String) -> Result<Self> {
        let wallet_address = Address::from_str(&wallet_address)?;
        let transactions = HashMap::new();

        Ok(Self {
            wallet_address,
            transactions,
        })
    }

    /// Import transactions from Gnosis Scan.
    pub fn import_from_gnosisscan(&mut self, response: Response) -> Result<()> {
        if response.result.is_none() {
            return Err("invalid response".into());
        }

        // This is to see if we are paying with the card, or sending EURe on chain.
        let monerium_address = Address::from_str(GNOSIS_BANK)?;

        // Parse all onchain transactions.
        for token_tx in response.result.unwrap() {
            // Parse the transaction date.
            let Some(date) = DateTime::from_timestamp(token_tx.timestamp.parse::<i64>()?, 0) else {
                return Err("cannot parse date".into());
            };

            // Parse the amount, change amount sign if spending money.
            let amount = U256::from_str_radix(token_tx.value.as_str(), 10)?;
            let amount = format_units(amount, token_tx.token_decimal)?;
            let mut amount = amount.parse::<f64>()?;

            // Parse description.
            let name = if Address::from_str(token_tx.from.as_str())? == Address::ZERO {
                String::from("Minted on chain")
            } else if Address::from_str(token_tx.to.as_str())? == Address::ZERO {
                amount = -amount;
                String::from("Burned on chain")
            } else if Address::from_str(token_tx.to.as_str())? == self.wallet_address {
                String::from("Added funds on chain")
            } else if Address::from_str(token_tx.from.as_str())? == self.wallet_address
                && Address::from_str(token_tx.to.as_str())? != monerium_address
            {
                amount = -amount;
                String::from("Sent funds on chain")
            } else if Address::from_str(token_tx.from.as_str())? == self.wallet_address
                && Address::from_str(token_tx.to.as_str())? == monerium_address
            {
                amount = -amount;
                // This will actually be updated when importing transactions from GnosisPay.
                String::from("From Gnosiscard")
            } else {
                String::from("")
            };
            let description = String::from("");

            let transaction = Transaction {
                hash: token_tx.hash.clone(),
                date,
                amount,
                name,
                description,
            };

            self.transactions.insert(token_tx.hash.clone(), transaction);
        }

        Ok(())
    }

    /// Merge description with information from Gnosis Pay.
    pub fn merge_description_from_gnosispay(
        &mut self,
        gnonis_transactions: Vec<GnosisTransaction>,
    ) -> Result<()> {
        for gnonis_transaction in gnonis_transactions {
            if gnonis_transaction.transactions.len() > 0 {
                let hash = gnonis_transaction.transactions[0].hash.clone();

                if self.transactions.contains_key(&hash) {
                    let name = format!("{}", gnonis_transaction.merchant.name.trim());

                    // Use the merchant's city for the description.
                    let description = format!(
                        "{} - {}",
                        gnonis_transaction.merchant.city.trim(),
                        gnonis_transaction.merchant.country.name.trim(),
                    );

                    let transaction = self.transactions.get_mut(&hash).unwrap();
                    transaction.name = name;
                    transaction.description = description;
                }
            }
        }

        Ok(())
    }

    /// Check that the sum of all transaction match current balance.
    pub fn verify(&self) {
        let mut amount = 0.0;

        for (_, transaction) in &self.transactions {
            amount = amount + transaction.amount;
        }

        println!("Total: {:.2}", amount);
    }

    /// Export all transactions in CSV format.
    pub fn to_csv(&self, filename: String) -> Result<()> {
        let mut file = File::create(filename.clone())?;

        file.write("ID;Date;Amount;Name;Description\n".as_bytes())?;

        // Sorting transactions by date.
        let mut sorted: Vec<_> = self.transactions.iter().collect();
        sorted.sort_by_key(|a| a.1.date);

        for (_, transaction) in sorted {
            file.write(
                format!(
                    "{};{};{:.2};{};{}\n",
                    transaction.hash,
                    transaction.date.format("%d-%m-%Y"),
                    transaction.amount,
                    transaction.name,
                    transaction.description
                )
                .as_bytes(),
            )?;
        }

        println!("{}", format!("[+] CSV export to `{}`.", filename).green());
        Ok(())
    }
}

impl fmt::Display for Transactions {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Sorting transactions by date.
        let mut sorted: Vec<_> = self.transactions.iter().collect();
        sorted.sort_by_key(|a| a.1.date);

        // Display all transactions.
        for (_, transaction) in sorted {
            write!(f, "{}: {}\n", "Hash".blue(), transaction.hash.yellow())?;
            write!(
                f,
                "{}: {}\n",
                "Date".blue(),
                transaction.date.format("%d-%m-%Y").to_string()
            )?;

            if transaction.amount > 0.0 {
                write!(
                    f,
                    "{}: {}\n",
                    "Amount".blue(),
                    format!("{:.2}", transaction.amount).green()
                )?;
            } else {
                write!(
                    f,
                    "{}: {}\n",
                    "Amount".blue(),
                    format!("{:.2}", transaction.amount).red()
                )?;
            }

            write!(f, "{}: {}\n", "Name".blue(), transaction.name)?;
            write!(f, "{}: {}\n", "Description".blue(), transaction.description)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}
