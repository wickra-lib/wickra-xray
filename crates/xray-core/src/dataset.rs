//! The `Dataset` — the loaded event window the panels fold over (§6.7).

use serde::{Deserialize, Serialize};

use crate::types::{BookEvent, FundingEvent, LiquidationEvent, OiEvent, Trade};
use crate::Result;

/// An OHLCV candle (§6.1). Defined here rather than re-exported so the JSON
/// contract uses `ts` and stays byte-identical across every binding.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Candle {
    /// Bar timestamp in milliseconds.
    pub ts: i64,
    /// Open price.
    pub open: f64,
    /// High price.
    pub high: f64,
    /// Low price.
    pub low: f64,
    /// Close price.
    pub close: f64,
    /// Bar volume in base units.
    pub volume: f64,
}

/// The event streams forming a loaded dataset window. Every stream is optional
/// in the JSON (`#[serde(default)]`), so a caller supplies only the panels' inputs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Dataset {
    /// OHLCV candles.
    #[serde(default)]
    pub candles: Vec<Candle>,
    /// Aggressor-tagged trades.
    #[serde(default)]
    pub trades: Vec<Trade>,
    /// Order-book snapshots and diffs.
    #[serde(default)]
    pub book: Vec<BookEvent>,
    /// Funding-rate events.
    #[serde(default)]
    pub funding: Vec<FundingEvent>,
    /// Open-interest events.
    #[serde(default)]
    pub oi: Vec<OiEvent>,
    /// Liquidation events.
    #[serde(default)]
    pub liquidations: Vec<LiquidationEvent>,
}

impl Dataset {
    /// Parse a dataset from JSON. The CLI resolves a `dataset_ref` to a
    /// directory and assembles this from per-stream files before handing it in.
    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    /// Stable-sort every stream by timestamp (equal `ts` keep their input order),
    /// so the downstream fold is deterministic.
    pub fn sort(&mut self) {
        self.candles.sort_by_key(|c| c.ts);
        self.trades.sort_by_key(|t| t.ts);
        self.book.sort_by_key(|b| b.ts);
        self.funding.sort_by_key(|f| f.ts);
        self.oi.sort_by_key(|o| o.ts);
        self.liquidations.sort_by_key(|l| l.ts);
    }

    /// The total number of events across every stream.
    #[must_use]
    pub fn n_events(&self) -> usize {
        self.candles.len()
            + self.trades.len()
            + self.book.len()
            + self.funding.len()
            + self.oi.len()
            + self.liquidations.len()
    }

    /// The `(min_ts, max_ts, n_events)` bounds across every stream, or `None`
    /// when the dataset is empty.
    #[must_use]
    pub fn bounds(&self) -> Option<(i64, i64, usize)> {
        let n = self.n_events();
        if n == 0 {
            return None;
        }
        let (lo, hi) = self
            .candles
            .iter()
            .map(|c| c.ts)
            .chain(self.trades.iter().map(|t| t.ts))
            .chain(self.book.iter().map(|b| b.ts))
            .chain(self.funding.iter().map(|f| f.ts))
            .chain(self.oi.iter().map(|o| o.ts))
            .chain(self.liquidations.iter().map(|l| l.ts))
            .fold((i64::MAX, i64::MIN), |(lo, hi), ts| {
                (lo.min(ts), hi.max(ts))
            });
        Some((lo, hi, n))
    }

    /// A copy of the dataset clipped to the inclusive `[from, to]` window (a
    /// `None` bound is unbounded on that side). Order within each stream is
    /// preserved.
    #[must_use]
    pub fn window(&self, from: Option<i64>, to: Option<i64>) -> Dataset {
        let keep = |ts: i64| from.is_none_or(|f| ts >= f) && to.is_none_or(|t| ts <= t);
        Dataset {
            candles: self
                .candles
                .iter()
                .copied()
                .filter(|c| keep(c.ts))
                .collect(),
            trades: self.trades.iter().copied().filter(|t| keep(t.ts)).collect(),
            book: self.book.iter().filter(|b| keep(b.ts)).cloned().collect(),
            funding: self
                .funding
                .iter()
                .copied()
                .filter(|f| keep(f.ts))
                .collect(),
            oi: self.oi.iter().copied().filter(|o| keep(o.ts)).collect(),
            liquidations: self
                .liquidations
                .iter()
                .copied()
                .filter(|l| keep(l.ts))
                .collect(),
        }
    }
}
