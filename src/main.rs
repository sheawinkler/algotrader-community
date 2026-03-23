use std::path::PathBuf;

use algotrader_community::config::{StrategyConfig, LIVE_DEV_MESSAGE};
use algotrader_community::engine::run_backtest;
use algotrader_community::market::load_market_csv;
use algotrader_community::report::{format_trade, summarize};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "algotrader-community",
    about = "Community edition of AlgoTrader: paper/backtest only"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Demo,
    Backtest {
        #[arg(long, short, default_value = "examples/sample_market.csv")]
        input: PathBuf,
    },
    LiveDev,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Demo => run(PathBuf::from("examples/sample_market.csv")),
        Command::Backtest { input } => run(input),
        Command::LiveDev => {
            println!("{LIVE_DEV_MESSAGE}");
            Ok(())
        }
    }
}

fn run(input: PathBuf) -> Result<()> {
    let bars = load_market_csv(&input)
        .with_context(|| format!("load backtest data from {}", input.display()))?;
    let trades = run_backtest(&bars, &StrategyConfig::default());
    let summary = summarize(&trades);

    println!("community edition backtest");
    println!("input={}", input.display());
    println!("bars={}", bars.len());
    println!("trades={}", summary.trade_count);
    println!("wins={} losses={}", summary.wins, summary.losses);
    println!(
        "total_pnl_sol={:+.4} avg_pnl_sol={:+.4} avg_win_sol={:+.4} avg_loss_sol={:+.4}",
        summary.total_pnl_sol, summary.avg_pnl_sol, summary.avg_win_sol, summary.avg_loss_sol
    );

    if trades.is_empty() {
        println!("No trades fired with the default community thresholds.");
    } else {
        println!();
        println!("recent trades:");
        for trade in trades.iter().rev().take(5).rev() {
            println!("{}", format_trade(trade));
        }
    }

    Ok(())
}
