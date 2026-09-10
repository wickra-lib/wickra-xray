//! Order-book reconstruction (§6.7 / §1.6).
//!
//! Folding book snapshots and diffs into a live book is inherently sequential
//! (order matters), so the state is kept in two `BTreeMap`s keyed by
//! [`OrderedF64`] for deterministic iteration.

use std::collections::BTreeMap;

use crate::types::{BookEvent, BookKind, OrderedF64};

/// A reconstructed order book: price -> resting quantity, per side.
#[derive(Clone, Debug, Default)]
pub struct BookState {
    bids: BTreeMap<OrderedF64, f64>,
    asks: BTreeMap<OrderedF64, f64>,
}

impl BookState {
    /// A fresh, empty book.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a book event: a snapshot replaces the whole book; a diff merges the
    /// given levels, and a level with `qty == 0.0` is removed. Cost is O(levels
    /// in the event).
    pub fn apply(&mut self, ev: &BookEvent) {
        if ev.kind == BookKind::Snapshot {
            self.bids.clear();
            self.asks.clear();
        }
        apply_side(&mut self.bids, &ev.bids);
        apply_side(&mut self.asks, &ev.asks);
    }

    /// The mid price, `(best_bid + best_ask) / 2`, or `None` if either side is
    /// empty.
    #[must_use]
    pub fn mid(&self) -> Option<f64> {
        let best_bid = self.bids.keys().next_back()?.0;
        let best_ask = self.asks.keys().next()?.0;
        Some(f64::midpoint(best_bid, best_ask))
    }

    /// Total resting quantity (bids + asks) whose price falls in the half-open
    /// bin `[bin_lo, bin_hi)`.
    #[must_use]
    pub fn resting_in_bin(&self, bin_lo: f64, bin_hi: f64) -> f64 {
        let sum = |side: &BTreeMap<OrderedF64, f64>| {
            side.iter()
                .filter(|(price, _)| price.0 >= bin_lo && price.0 < bin_hi)
                .map(|(_, qty)| *qty)
                .sum::<f64>()
        };
        sum(&self.bids) + sum(&self.asks)
    }
}

/// Apply level changes to one side: `qty == 0.0` removes, otherwise set.
fn apply_side(side: &mut BTreeMap<OrderedF64, f64>, levels: &[[f64; 2]]) {
    for &[price, qty] in levels {
        if qty == 0.0 {
            side.remove(&OrderedF64(price));
        } else {
            side.insert(OrderedF64(price), qty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> BookEvent {
        BookEvent {
            ts: 1,
            kind: BookKind::Snapshot,
            bids: vec![[100.4, 3.1], [100.3, 5.0]],
            asks: vec![[100.5, 2.2], [100.6, 4.7]],
        }
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn snapshot_sets_the_book_and_mid() {
        let mut book = BookState::new();
        book.apply(&snapshot());
        assert!(close(book.mid().unwrap(), f64::midpoint(100.4, 100.5)));
    }

    #[test]
    fn diff_merges_updates_and_removes() {
        let mut book = BookState::new();
        book.apply(&snapshot());
        book.apply(&BookEvent {
            ts: 2,
            kind: BookKind::Diff,
            bids: vec![[100.4, 0.0], [100.35, 1.0]], // remove 100.4, add 100.35
            asks: vec![[100.5, 1.1]],                // update 100.5
        });
        // Best bid is now 100.35, best ask still 100.5.
        assert!(close(book.mid().unwrap(), f64::midpoint(100.35, 100.5)));
        // Resting in [100.3, 100.5): bids 100.3 (5.0) + 100.35 (1.0) = 6.0.
        assert!(close(book.resting_in_bin(100.3, 100.5), 6.0));
    }

    #[test]
    fn mid_is_none_when_a_side_is_empty() {
        let mut book = BookState::new();
        book.apply(&BookEvent {
            ts: 1,
            kind: BookKind::Snapshot,
            bids: vec![[100.0, 1.0]],
            asks: vec![],
        });
        assert_eq!(book.mid(), None);
    }
}
