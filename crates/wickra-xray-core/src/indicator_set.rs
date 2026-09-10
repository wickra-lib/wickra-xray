//! Indicator resolution — the reserved seam for indicator-backed panels (§1.7).
//!
//! # Decision: the MVP panels need no indicators
//!
//! All four microstructure panels are **pure aggregations** of the dataset event
//! streams, so none resolves a `wickra-core` indicator:
//!
//! - **Footprint** — buy/sell volume per price bin, summed from the trades.
//! - **Book heatmap** — resting quantity per (time, price) cell, folded from the
//!   book snapshots and diffs.
//! - **Liquidation map** — liquidation size per price bin, from the liquidations.
//! - **Funding/OI divergence** — the last funding, open-interest and price value
//!   per time bucket (carry-forward); the divergence itself is derived by the
//!   consumer, not the core.
//!
//! Because no panel needs a `name + params` indicator lookup, this module ships
//! **no resolver** rather than dead machinery. Should a future panel need a
//! derived series (e.g. an indicator-smoothed funding curve), this is where the
//! `wickra-backtest-core` registry factory would be wired in — the same resolver
//! the screener and backtester use — and the registry dependencies would return
//! to the crate manifest at that point.
