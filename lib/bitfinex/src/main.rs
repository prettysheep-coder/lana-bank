use anyhow::Result;
use clap::{Parser, Subcommand};
use config::Config;
use rust_decimal::Decimal;
use tokio;

use websocket::connect_to_websocket;

mod bfx;
mod config;
mod websocket;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long)]
    // action: ActionBfx,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GetAddresses,
    GetBalances { currency: Option<String> },
    GetTransactions { currency: Option<String> },
    CreateAddress,
    GetUserInfo,
    WithdrawUsdtTrx { amount: Decimal, address: String },
    Connect { url: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::build()?;

    let cli = Cli::parse();

    let command = cli.command;

    match command {
        Commands::CreateAddress => {
            println!("create address action");
            let _ = bfx::create_trx_address(&config).await;
        }
        Commands::GetAddresses => {
            println!("create address action");
            let _ = bfx::get_addresses(&config).await;
        }
        Commands::GetBalances { currency } => {
            println!("get balances action");
            let _balance = bfx::get_balances(&config, currency).await;
        }
        Commands::GetTransactions { currency } => {
            println!("get transactions action");
            let _txs = bfx::get_transactions(&config, currency).await;
        }
        Commands::GetUserInfo => {
            println!("get user info action");
            let _txs = bfx::get_user_info(&config).await;
        }
        Commands::WithdrawUsdtTrx { amount, address } => {
            println!("withdraw usdt over tron");
            let _txs = bfx::withdraw_usdt_trx(&config, amount, address).await;
        }
        Commands::Connect { url } => {
            let url = url::Url::parse(&url).unwrap();
            connect_to_websocket(url).await?;
        }
    }

    Ok(())
}
