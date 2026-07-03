//! Input event types and numeric helpers for the xray core.
//!
//! The dataset is five time-stamped event streams (§6.1). Field names and enum
//! representations are the JSON contract, so they must stay byte-identical
//! across every binding.

use core::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// The aggressor side of a trade (the taker's direction).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// The taker bought, lifting the ask.
    Buy,
    /// The taker sold, hitting the bid.
    Sell,
}

/// The side of a liquidated position.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LiqSide {
    /// A long position was liquidated.
    Long,
    /// A short position was liquidated.
    Short,
}

/// Whether a book event is a full snapshot or an incremental diff.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BookKind {
    /// Replace the whole book with the given levels.
    Snapshot,
    /// Apply the given level changes; a `qty` of `0.0` removes a level.
    Diff,
}

/// An aggressor-tagged trade.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Trade {
    /// Event timestamp in milliseconds.
    pub ts: i64,
    /// Trade price.
    pub price: f64,
    /// Trade quantity in base units.
    pub qty: f64,
    /// The taker's side.
    pub side: Side,
}

/// An order-book snapshot or diff. Each level is `[price, qty]`; in a diff a
/// `qty` of `0.0` removes the level.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookEvent {
    /// Event timestamp in milliseconds.
    pub ts: i64,
    /// Whether this replaces the book (snapshot) or updates it (diff).
    pub kind: BookKind,
    /// Bid levels, each `[price, qty]`.
    pub bids: Vec<[f64; 2]>,
    /// Ask levels, each `[price, qty]`.
    pub asks: Vec<[f64; 2]>,
}

/// A funding-rate event (the 8-hour rate as a fraction).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct FundingEvent {
    /// Event timestamp in milliseconds.
    pub ts: i64,
    /// Funding rate as a fraction (e.g. `0.0001`).
    pub rate: f64,
}

/// An open-interest event (in base units).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct OiEvent {
    /// Event timestamp in milliseconds.
    pub ts: i64,
    /// Open interest in base units.
    pub oi: f64,
}

/// A liquidation event.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct LiquidationEvent {
    /// Event timestamp in milliseconds.
    pub ts: i64,
    /// Liquidation price.
    pub price: f64,
    /// Liquidated quantity in base units.
    pub qty: f64,
    /// The liquidated position side.
    pub side: LiqSide,
}

/// An `f64` with a total order, for use as a `BTreeMap` key (book and bin keys).
/// Ordering uses [`f64::total_cmp`], so it is total and never panics — the basis
/// for the deterministic book reconstruction and bin aggregation.
#[derive(Clone, Copy, Debug)]
pub struct OrderedF64(pub f64);

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for OrderedF64 {}

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for OrderedF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Round to eight decimal places — the JSON number rounding used across every
/// output path (§6.9) so the frame is byte-identical in every language.
pub fn round8(x: f64) -> f64 {
    (x * 1e8).round() / 1e8
}

/// The lower edge of the price bin containing `price` for bin width `bin`:
/// `floor(price / bin) * bin`, rounded to eight places.
pub fn bin_of(price: f64, bin: f64) -> f64 {
    round8((price / bin).floor() * bin)
}
