#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub trade_size_sol: f64,
    pub warmup_bars: usize,
    pub momentum_lookback: usize,
    pub volume_lookback: usize,
    pub min_momentum_pct: f64,
    pub min_volume_surge: f64,
    pub min_flow_imbalance: f64,
    pub take_profit_pct: f64,
    pub stop_loss_pct: f64,
    pub trailing_retrace_pct: f64,
    pub max_hold_bars: usize,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            trade_size_sol: 1.0,
            warmup_bars: 8,
            momentum_lookback: 3,
            volume_lookback: 5,
            min_momentum_pct: 0.035,
            min_volume_surge: 1.25,
            min_flow_imbalance: 0.18,
            take_profit_pct: 0.08,
            stop_loss_pct: 0.04,
            trailing_retrace_pct: 0.03,
            max_hold_bars: 6,
        }
    }
}

pub const LIVE_DEV_MESSAGE: &str =
    "Full feature access and live dev mode are not part of the community edition. Message @sheawinkler on GitHub to discuss access.";
