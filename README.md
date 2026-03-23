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
- `price_sol`
- `volume_sol`
- `buy_flow_sol`
- `sell_flow_sol`
- momentum
- volume surge
- flow imbalance
- entries
- exits
- hold time
- PnL in SOL

At a higher level, the broader private system combines multiple source families into one decision surface instead of depending on a single feed. That wider universe includes:
- market discovery
- price / volume / liquidity context
- trade-flow context
- cross-venue context
- onchain state
- execution telemetry
- portfolio / risk state
- outcome feedback

### Data-source mind-map

The public repo does not ship the full private ingestion stack, but this is the shape of the broader data universe behind it:

```text
AlgoTrader data universe
├── Community edition
│   ├── local sample market data
│   ├── price_sol / volume_sol
│   ├── buy_flow_sol / sell_flow_sol
│   ├── momentum / surge / imbalance features
│   └── entries / exits / hold time / PnL
└── Broader private system
    ├── Low-latency onchain streams
    │   ├── Yellowstone gRPC
    │   └── Bitquery / CoreCast
    ├── Chain state and metadata
    │   ├── Helius
    │   └── Solana RPC / account-program state
    ├── Discovery and enrichment
    │   ├── Birdeye
    │   ├── DexScreener
    │   ├── Jupiter discovery / market surfaces
    │   └── Pump.fun launch / migration context
    ├── Market and execution context
    │   ├── Jupiter quotes / routing context
    │   ├── liquidity and route-quality observations
    │   └── execution telemetry
    ├── External reference markets
    │   ├── Kraken
    │   ├── Binance
    │   └── Coinbase
    └── Outcome feedback
        ├── realized trade outcomes
        └── calibration / decision feedback loops
```

Some of the provider names above exist only in the broader private system and are intentionally not shipped in this community edition. The fuller public/private boundary is documented in `DATA_UNIVERSE.md`.

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
