# Data Universe

This public repo is intentionally smaller than the private production system, but it should still give you a real sense of the universe the project is built around.

At a high level, the private system combines multiple source families into one decision surface instead of pretending a single feed is enough.

## Public community-edition layers

The community edition currently exposes a simple, local paper/backtest slice with:

- `price_sol`
  - asset price expressed directly in SOL
- `volume_sol`
  - total traded volume expressed in SOL
- `buy_flow_sol`
  - buy-side flow expressed in SOL
- `sell_flow_sol`
  - sell-side flow expressed in SOL
- derived signal features
  - momentum
  - volume surge
  - flow imbalance
- outcome tracking
  - entries
  - exits
  - hold time
  - PnL in SOL

This is enough to show the framework, the unit system, and the basic flow of turning market observations into decisions and outcomes.

## Broader private-system universe

The broader system is designed around a much wider market and execution surface. Without disclosing the private implementation, the core universe includes categories like:

- source families
  - low-latency onchain event streams
  - venue market data and trade activity
  - liquidity and route-quality observations
  - account, wallet, and program state
  - execution and close-out telemetry
- market discovery
  - token discovery and candidate generation across a broad onchain universe
- price / volume / liquidity context
  - multi-horizon market state, liquidity quality, and participation context
- trade-flow context
  - directional flow, imbalance, and surge behavior across time windows
- cross-venue context
  - venue-aware state, routing context, and relative market behavior
- onchain state
  - token/account/program-derived observations that help characterize live conditions
- execution telemetry
  - routeability, fill quality, slippage pressure, and exitability observations
- portfolio / risk state
  - live exposure, concentration, recycle control, and capital discipline
- outcome feedback
  - realized trade outcomes, calibration inputs, and decision feedback loops

## Public/private line

This repo is meant to show the framework and a usable local workflow, not the full private edge.

That means the public repo intentionally does **not** ship:

- live trading infrastructure
- low-latency private ingestion paths
- proprietary provider wiring
- wallet-following or private cohort intelligence
- predictive runtime and calibration internals
- deploy/relaunch automation for the private stack

For full feature access or live dev mode, message [@sheawinkler](https://github.com/sheawinkler).
