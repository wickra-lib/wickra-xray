//! Liquidation-map builder — liquidations snapped to price bins (§6.4.3).
//!
//! Each liquidation in the window becomes one [`LiqEvent`] whose `price_bin` is
//! `floor(price/price_bin)*price_bin`. The events stay event-granular (the
//! renderer clusters them visually), sorted by `(ts, price_bin, side)` for a
//! deterministic, byte-reproducible order.

use crate::dataset::Dataset;
use crate::frame::{LiqEvent, LiqMapData};
use crate::types::{bin_of, round8, LiqSide};

/// A stable rank for the side, so it can join the sort key.
fn side_rank(side: LiqSide) -> u8 {
    match side {
        LiqSide::Long => 0,
        LiqSide::Short => 1,
    }
}

/// Build the liquidation map over the (already windowed) dataset.
#[must_use]
pub fn build(win: &Dataset, price_bin: f64) -> LiqMapData {
    let mut events: Vec<LiqEvent> = win
        .liquidations
        .iter()
        .map(|liq| LiqEvent {
            ts: liq.ts,
            price_bin: bin_of(liq.price, price_bin),
            qty: round8(liq.qty),
            side: liq.side,
        })
        .collect();
    events.sort_by(|a, b| {
        a.ts.cmp(&b.ts)
            .then_with(|| a.price_bin.total_cmp(&b.price_bin))
            .then_with(|| side_rank(a.side).cmp(&side_rank(b.side)))
    });
    LiqMapData { events }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LiquidationEvent;

    fn liq(ts: i64, price: f64, qty: f64, side: LiqSide) -> LiquidationEvent {
        LiquidationEvent {
            ts,
            price,
            qty,
            side,
        }
    }

    #[test]
    fn snaps_price_to_bin() {
        let win = Dataset {
            liquidations: vec![liq(10, 100.7, 2.0, LiqSide::Long)],
            ..Dataset::default()
        };
        let map = build(&win, 1.0);
        assert_eq!(map.events.len(), 1);
        let e = &map.events[0];
        // Tuple compare avoids the scalar float_cmp lint.
        assert_eq!((e.price_bin, e.qty, e.side), (100.0, 2.0, LiqSide::Long));
    }

    #[test]
    fn sorts_by_ts_then_bin_then_side() {
        let win = Dataset {
            liquidations: vec![
                liq(20, 100.0, 1.0, LiqSide::Short),
                liq(10, 102.0, 1.0, LiqSide::Long),
                liq(10, 101.0, 1.0, LiqSide::Short),
                liq(10, 101.0, 1.0, LiqSide::Long),
            ],
            ..Dataset::default()
        };
        let map = build(&win, 1.0);
        let key: Vec<(i64, f64, LiqSide)> = map
            .events
            .iter()
            .map(|e| (e.ts, e.price_bin, e.side))
            .collect();
        assert_eq!(
            key,
            vec![
                (10, 101.0, LiqSide::Long),
                (10, 101.0, LiqSide::Short),
                (10, 102.0, LiqSide::Long),
                (20, 100.0, LiqSide::Short),
            ]
        );
    }

    #[test]
    fn empty_window_yields_no_events() {
        let map = build(&Dataset::default(), 1.0);
        assert!(map.events.is_empty());
    }
}
