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

## Support

If this public build is useful to you, support development here:

`D2R9kNNPLo7w552uX5FakEh2vgDajpCC4LAaBfhmxiMS`

For full feature access or live dev mode, message [@sheawinkler](https://github.com/sheawinkler).
See `SUPPORT.md` for details.

## Data universe

This repo exposes a small, safe slice of a much larger trading universe.

The community edition shows the shape of the stack:
- SOL-denominated price bars
- SOL-denominated volume and trade-flow
- momentum / surge / imbalance features
- entry, exit, hold-time, and PnL outcomes

The broader system is built around additional source families and layers such as market discovery, onchain event streams, venue-aware market context, execution telemetry, and outcome feedback loops. The public/private boundary is documented in `DATA_UNIVERSE.md`.

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

The exact public/private split is documented in `COMMUNITY_EDITION.md`, and the public-facing data map is documented in `DATA_UNIVERSE.md`.
