use crate::config::StrategyConfig;
use crate::market::MarketBar;

#[derive(Debug, Clone)]
pub struct EntrySignal {
    pub momentum_pct: f64,
    pub volume_surge: f64,
    pub flow_imbalance: f64,
}

pub fn entry_signal(bars: &[MarketBar], index: usize, cfg: &StrategyConfig) -> Option<EntrySignal> {
    if index < cfg.warmup_bars || index < cfg.momentum_lookback || index < cfg.volume_lookback {
        return None;
    }

    let current = &bars[index];
    let lookback = &bars[index - cfg.momentum_lookback];
    let momentum_pct = current.price_sol / lookback.price_sol - 1.0;
    let volume_window = &bars[index - cfg.volume_lookback..index];
    let avg_volume =
        volume_window.iter().map(|bar| bar.volume_sol).sum::<f64>() / volume_window.len() as f64;
    let volume_surge = if avg_volume <= f64::EPSILON {
        0.0
    } else {
        current.volume_sol / avg_volume
    };
    let flow_imbalance = current.flow_imbalance();

    Some(EntrySignal {
        momentum_pct,
        volume_surge,
        flow_imbalance,
    })
}

pub fn should_enter(signal: &EntrySignal, cfg: &StrategyConfig) -> bool {
    signal.momentum_pct >= cfg.min_momentum_pct
        && signal.volume_surge >= cfg.min_volume_surge
        && signal.flow_imbalance >= cfg.min_flow_imbalance
}
