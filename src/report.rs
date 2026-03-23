use crate::engine::{ExitReason, Trade};

#[derive(Debug, Clone)]
pub struct BacktestSummary {
    pub trade_count: usize,
    pub wins: usize,
    pub losses: usize,
    pub total_pnl_sol: f64,
    pub avg_pnl_sol: f64,
    pub avg_win_sol: f64,
    pub avg_loss_sol: f64,
}

pub fn summarize(trades: &[Trade]) -> BacktestSummary {
    let trade_count = trades.len();
    let wins = trades.iter().filter(|trade| trade.pnl_sol > 0.0).count();
    let losses = trades.iter().filter(|trade| trade.pnl_sol <= 0.0).count();
    let total_pnl_sol = trades.iter().map(|trade| trade.pnl_sol).sum::<f64>();
    let avg_pnl_sol = if trade_count == 0 {
        0.0
    } else {
        total_pnl_sol / trade_count as f64
    };

    let win_values: Vec<f64> = trades
        .iter()
        .filter(|trade| trade.pnl_sol > 0.0)
        .map(|trade| trade.pnl_sol)
        .collect();
    let loss_values: Vec<f64> = trades
        .iter()
        .filter(|trade| trade.pnl_sol <= 0.0)
        .map(|trade| trade.pnl_sol)
        .collect();

    let avg_win_sol = if win_values.is_empty() {
        0.0
    } else {
        win_values.iter().sum::<f64>() / win_values.len() as f64
    };
    let avg_loss_sol = if loss_values.is_empty() {
        0.0
    } else {
        loss_values.iter().sum::<f64>() / loss_values.len() as f64
    };

    BacktestSummary {
        trade_count,
        wins,
        losses,
        total_pnl_sol,
        avg_pnl_sol,
        avg_win_sol,
        avg_loss_sol,
    }
}

pub fn format_trade(trade: &Trade) -> String {
    let exit_reason = match trade.exit_reason {
        ExitReason::StopLoss => "stop_loss",
        ExitReason::TrailingStop => "trailing_stop",
        ExitReason::TimeStop => "time_stop",
    };
    format!(
        "{} -> {} | entry={:.4} exit={:.4} pnl_sol={:+.4} pnl_pct={:+.2}% hold_bars={} exit_reason={}",
        trade.entry_timestamp,
        trade.exit_timestamp,
        trade.entry_price_sol,
        trade.exit_price_sol,
        trade.pnl_sol,
        trade.pnl_pct * 100.0,
        trade.hold_bars,
        exit_reason
    )
}
