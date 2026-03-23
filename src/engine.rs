use crate::config::StrategyConfig;
use crate::market::MarketBar;
use crate::strategy::{entry_signal, should_enter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitReason {
    StopLoss,
    TrailingStop,
    TimeStop,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub entry_timestamp: String,
    pub exit_timestamp: String,
    pub entry_price_sol: f64,
    pub exit_price_sol: f64,
    pub pnl_sol: f64,
    pub pnl_pct: f64,
    pub hold_bars: usize,
    pub exit_reason: ExitReason,
}

#[derive(Debug, Clone)]
struct OpenPosition {
    entry_index: usize,
    entry_timestamp: String,
    entry_price_sol: f64,
    quantity: f64,
    max_return_pct: f64,
}

pub fn run_backtest(bars: &[MarketBar], cfg: &StrategyConfig) -> Vec<Trade> {
    let mut trades = Vec::new();
    let mut open_position: Option<OpenPosition> = None;

    for index in 0..bars.len() {
        let bar = &bars[index];

        if let Some(position) = open_position.as_mut() {
            let return_pct = bar.price_sol / position.entry_price_sol - 1.0;
            if return_pct > position.max_return_pct {
                position.max_return_pct = return_pct;
            }

            let hold_bars = index.saturating_sub(position.entry_index);
            let trailing_exit = position.max_return_pct >= cfg.take_profit_pct
                && return_pct <= position.max_return_pct - cfg.trailing_retrace_pct;
            let stop_loss = return_pct <= -cfg.stop_loss_pct;
            let time_stop = hold_bars >= cfg.max_hold_bars;

            let exit_reason = if stop_loss {
                Some(ExitReason::StopLoss)
            } else if trailing_exit {
                Some(ExitReason::TrailingStop)
            } else if time_stop {
                Some(ExitReason::TimeStop)
            } else {
                None
            };

            if let Some(exit_reason) = exit_reason {
                let exit_value_sol = position.quantity * bar.price_sol;
                let pnl_sol = exit_value_sol - cfg.trade_size_sol;
                trades.push(Trade {
                    entry_timestamp: position.entry_timestamp.clone(),
                    exit_timestamp: bar.timestamp.clone(),
                    entry_price_sol: position.entry_price_sol,
                    exit_price_sol: bar.price_sol,
                    pnl_sol,
                    pnl_pct: return_pct,
                    hold_bars,
                    exit_reason,
                });
                open_position = None;
            }
            continue;
        }

        let Some(signal) = entry_signal(bars, index, cfg) else {
            continue;
        };
        if !should_enter(&signal, cfg) {
            continue;
        }

        open_position = Some(OpenPosition {
            entry_index: index,
            entry_timestamp: bar.timestamp.clone(),
            entry_price_sol: bar.price_sol,
            quantity: cfg.trade_size_sol / bar.price_sol,
            max_return_pct: 0.0,
        });
    }

    trades
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StrategyConfig;
    use crate::market::MarketBar;

    #[test]
    fn backtest_produces_trade_on_clear_breakout() {
        let mut bars = Vec::new();
        for idx in 0..16 {
            let price = match idx {
                0..=7 => 1.0 + (idx as f64 * 0.01),
                8 => 1.12,
                9 => 1.18,
                10 => 1.21,
                11 => 1.19,
                _ => 1.17,
            };
            bars.push(MarketBar {
                timestamp: format!("t{idx}"),
                price_sol: price,
                volume_sol: if idx >= 8 { 30.0 } else { 10.0 },
                buy_flow_sol: if idx >= 8 { 24.0 } else { 6.0 },
                sell_flow_sol: if idx >= 8 { 4.0 } else { 4.0 },
            });
        }

        let trades = run_backtest(&bars, &StrategyConfig::default());
        assert!(!trades.is_empty());
    }
}
