# AlgoTrader Community Edition

This is the public, intentionally reduced edition of AlgoTrader.

It is designed to be useful without exposing the private edge:
- paper/backtest only
- SOL-denominated sample data and reporting
- simple momentum + flow-imbalance entry logic
- deterministic local CLI workflow

It does **not** include:
- live trading
- wallet execution
- Yellowstone gRPC ingestion
- Bitquery/CoreCast ingestion
- sidecar integration
- wallet-following / smart-money logic
- proprietary predictive selector/runtime
- private deployment scripts or live dev tooling

For **full feature access** or **live dev mode** to try new features, message [@sheawinkler](https://github.com/sheawinkler).

## Quick start

```bash
cargo run -- demo
```

Run the sample backtest explicitly:

```bash
cargo run -- backtest --input examples/sample_market.csv
```

If you try the gated mode:

```bash
cargo run -- live-dev
```

the community edition will direct you to contact [@sheawinkler](https://github.com/sheawinkler).

## Why this repo exists

The production/private system contains significant proprietary work around live data ingestion, execution, calibration, and operating discipline. This public repo is a safe community edition that exposes the general framework and a usable local workflow without disclosing the private edge.

The exact public/private split is documented in `COMMUNITY_EDITION.md`.
