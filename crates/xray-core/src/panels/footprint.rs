//! Footprint builder — per-price-bin buy/sell volume (§6.4.1).
//!
//! Every trade in the window is snapped to its price bin
//! (`floor(price/price_bin)*price_bin`) and its quantity added to that bin's
//! buy or sell total by aggressor side. Accumulation goes through a
//! `BTreeMap<OrderedF64, _>` so the emitted bins come out in ascending price
//! order deterministically; only bins that received volume are emitted.

use std::collections::BTreeMap;

use crate::dataset::Dataset;
use crate::frame::FootprintData;
use crate::types::{bin_of, round8, OrderedF64, Side};

/// Build the footprint over the (already windowed) dataset.
///
/// `bucket_ms` is accepted for parity with the spec and reserved for future
/// time-segmented footprints; the MVP aggregates over the whole window, so it
/// is unused here.
#[must_use]
pub fn build(win: &Dataset, price_bin: f64, _bucket_ms: i64) -> FootprintData {
    let mut acc: BTreeMap<OrderedF64, (f64, f64)> = BTreeMap::new();
    for trade in &win.trades {
        let bin = bin_of(trade.price, price_bin);
        let cell = acc.entry(OrderedF64(bin)).or_insert((0.0, 0.0));
        match trade.side {
            Side::Buy => cell.0 += trade.qty,
            Side::Sell => cell.1 += trade.qty,
        }
    }

    let mut price_bins = Vec::with_capacity(acc.len());
    let mut buy_vol = Vec::with_capacity(acc.len());
    let mut sell_vol = Vec::with_capacity(acc.len());
    for (bin, (buy, sell)) in acc {
        price_bins.push(round8(bin.0));
        buy_vol.push(round8(buy));
        sell_vol.push(round8(sell));
    }
    FootprintData {
        price_bins,
        buy_vol,
        sell_vol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Trade;

    fn trade(ts: i64, price: f64, qty: f64, side: Side) -> Trade {
        Trade {
            ts,
            price,
            qty,
            side,
        }
    }

    /// The §6.11 worked example: five trades over bins 100 and 101.
    #[test]
    fn worked_example() {
        let win = Dataset {
            trades: vec![
                trade(1000, 100.4, 2.0, Side::Buy),
                trade(1100, 100.9, 1.0, Side::Sell),
                trade(1200, 101.2, 3.0, Side::Buy),
                trade(1300, 100.1, 1.5, Side::Sell),
                trade(1400, 101.8, 0.5, Side::Buy),
            ],
            ..Dataset::default()
        };
        let fp = build(&win, 1.0, 60_000);
        assert_eq!(fp.price_bins, vec![100.0, 101.0]);
        assert_eq!(fp.buy_vol, vec![2.0, 3.5]);
        assert_eq!(fp.sell_vol, vec![2.5, 0.0]);
    }

    #[test]
    fn empty_window_yields_empty_bins() {
        let fp = build(&Dataset::default(), 1.0, 60_000);
        assert!(fp.price_bins.is_empty());
        assert!(fp.buy_vol.is_empty());
        assert!(fp.sell_vol.is_empty());
    }
}
