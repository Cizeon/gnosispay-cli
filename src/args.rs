use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    #[clap(subcommand)]
    pub cmd: CmdSubcommand,
}

#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    #[arg(long, env = "GNOSISSCAN_API_KEY")]
    pub gnosisscan_api_key: String,

    #[arg(long, env = "WALLET_ADDRESS")]
    pub wallet_address: String,

    #[arg(long, env = "SESSION_TOKEN")]
    pub session_token: String,
}

#[derive(Debug, Clone, Args)]
pub struct OnchainArgs {
    #[arg(long, env = "WALLET_ADDRESS")]
    pub wallet_address: String,

    #[arg(
        long,
        env = "ETH_RPC_URL",
        default_value_t = String::from("wss://rpc.gnosischain.com/wss")
    )]
    pub rpc_url: String,
    #[arg(long, env = "PUSHOVER_USER")]
    pub pushover_user: String,
    #[arg(long, env = "PUSHOVER_TOKEN")]
    pub pushover_token: String,
}

#[derive(Debug, Clone, Args)]
pub struct ExportArgs {
    #[command(flatten)]
    pub list: ListArgs,
    pub filename: String,
}

#[derive(Debug, Subcommand)]
pub enum CmdSubcommand {
    List(ListArgs),
    Export(ExportArgs),
    Verify(ListArgs),
    Monitor(OnchainArgs),
}
