mod args;
mod error;
mod gnosispay;
mod gnosisscan;
mod monerium;
mod monitor;
mod notify;
mod prelude;
mod transactions;

use crate::args::CmdArgs;
use crate::gnosispay::GnosisPay;
use crate::monitor::monitor;
use crate::prelude::*;
use clap::Parser;
use gnosisscan::Gnosisscan;
use transactions::Transactions;

/// Load transactions from Gnosisscan and merge description with Gnosispay.
async fn load_transactions(
    wallet_address: String,
    session_token: String,
    gnosisscan_api_key: String,
) -> Result<Transactions> {
    // All transactions.
    let mut transactions = Transactions::new(wallet_address.clone().into())?;

    // Importing transactions from Gnosis Scan.
    let gnosisscan = Gnosisscan::new(gnosisscan_api_key.into());
    let response = gnosisscan
        .retrieve_eure_transactions(wallet_address.clone().into())
        .await?;

    transactions.import_from_gnosisscan(response)?;

    // Importing transaction's description from Gnosis Pay.
    let gnosispay = GnosisPay::new(session_token.into());
    let gnosis_transactions = gnosispay.retrieve_transactions().await?;
    transactions.merge_description_from_gnosispay(gnosis_transactions)?;

    Ok(transactions)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("-=[ gnosispay-cli v0.1.0 ]=-\n");

    // Parsing command line.
    let args = CmdArgs::parse();

    // Do we want colors?
    if args.no_color {
        colored::control::set_override(false);
    }

    // Execute the proper command.
    match args.cmd {
        args::CmdSubcommand::List(args) => {
            let transactions = load_transactions(
                args.wallet_address,
                args.session_token,
                args.gnosisscan_api_key,
            )
            .await?;
            println!("{}", transactions);
        }

        args::CmdSubcommand::Export(args) => {
            let transactions = load_transactions(
                args.list.wallet_address,
                args.list.session_token,
                args.list.gnosisscan_api_key,
            )
            .await?;

            transactions.to_csv(args.filename)?;
        }

        args::CmdSubcommand::Verify(args) => {
            let transactions = load_transactions(
                args.wallet_address,
                args.session_token,
                args.gnosisscan_api_key,
            )
            .await?;
            transactions.verify();
        }

        args::CmdSubcommand::Monitor(args) => {
            monitor(
                args.wallet_address,
                args.rpc_url,
                args.pushover_user,
                args.pushover_token,
            )
            .await?
        }
    }

    Ok(())
}
