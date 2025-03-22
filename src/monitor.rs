use std::str::FromStr;

use crate::monerium;
use crate::notify::PushOverNotify;
use crate::prelude::*;
use alloy::primitives::{Address, utils::format_units};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::{BlockNumberOrTag, Filter};
use alloy::sol;
use alloy::sol_types::SolEvent;
use colored::Colorize;
use futures_util::StreamExt;

sol!(
    #[sol(rpc)]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
);

/// Monitor transaction to Gnosis Pay from wallet_address.
pub async fn monitor(
    wallet_address: String,
    rpc_url: String,
    pushover_user: String,
    pushover_token: String,
) -> Result<()> {
    let pushover = PushOverNotify::new(pushover_user, pushover_token);
    let wallet_address = Address::from_str(&wallet_address)?;
    let monerium_address = Address::from_str(monerium::GNOSIS_BANK)?;

    // Connect to the endpoint.
    let ws = WsConnect::new(rpc_url.clone());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to logs.
    let filter = Filter::new()
        .address(Address::from_str(monerium::EURE_V2_ADDRESS)?)
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    println!(
        "{}",
        format!("[+] Listening for blocks on `{}`", rpc_url).green()
    );

    // Check each blocks.
    while let Some(log) = stream.next().await {
        match log.topic0() {
            Some(&IERC20::Transfer::SIGNATURE_HASH) => {
                let IERC20::Transfer { from, to, value } = log.log_decode()?.inner.data;

                let amount = format_units(value, monerium::EURE_V2_DECIMALS)?.parse::<f64>()?;

                if from == wallet_address && to == monerium_address {
                    let now = chrono::Local::now();

                    println!(
                        "[+] {}: Transfer from `{}` to `{}` of value {:.2}",
                        now.format("%Y-%m-%d %H:%M:%S"),
                        from,
                        to,
                        amount
                    );

                    pushover
                        .notify(String::from("GnosisPay"), format!("Amount: {:.2}", amount))
                        .await?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
