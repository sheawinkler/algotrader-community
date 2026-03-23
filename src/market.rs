use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketBar {
    pub timestamp: String,
    pub price_sol: f64,
    pub volume_sol: f64,
    pub buy_flow_sol: f64,
    pub sell_flow_sol: f64,
}

impl MarketBar {
    pub fn net_flow_sol(&self) -> f64 {
        self.buy_flow_sol - self.sell_flow_sol
    }

    pub fn flow_imbalance(&self) -> f64 {
        let total = self.buy_flow_sol + self.sell_flow_sol;
        if total <= f64::EPSILON {
            0.0
        } else {
            self.net_flow_sol() / total
        }
    }
}

pub fn load_market_csv(path: &Path) -> Result<Vec<MarketBar>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("open market csv at {}", path.display()))?;
    let mut bars = Vec::new();
    for row in reader.deserialize() {
        let bar: MarketBar = row.with_context(|| format!("parse csv row in {}", path.display()))?;
        bars.push(bar);
    }
    anyhow::ensure!(
        bars.len() >= 12,
        "need at least 12 bars for the community backtest"
    );
    Ok(bars)
}
