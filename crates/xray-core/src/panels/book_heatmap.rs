//! Book-heatmap builder — resting liquidity over a time x price grid (§6.4.2).
//!
//! The order book is reconstructed by folding the window's book events in time
//! order (snapshot replaces, diff merges, `qty == 0.0` removes). The result is a
//! dense `[T][P]` matrix: `intensity[i][j]` is the total resting quantity
//! (bids + asks) in price bin `price[j]` as the book stood at the end of time
//! bucket `time[i]`.
//!
//! # Deterministic axes
//!
//! The grid resolution comes from the spec (`price_bin`, `bucket_ms`,
//! `depth_levels`); the extent follows the window's book events:
//!
//! - **Time axis** — one column per `bucket_ms`-wide bucket, from the bucket of
//!   the first book event to the bucket of the last, inclusive. A bucket's
//!   column is the book state after every event whose timestamp falls in that
//!   bucket (carry-forward across empty buckets).
//! - **Price axis** — `depth_levels` bins centred on the mid of the *final*
//!   book state, `bin_of(mid, price_bin)` at index `depth_levels / 2`. A heatmap
//!   is only meaningful around a mid, so if the final book is not two-sided
//!   (no mid) the panel is empty.
//!
//! Both choices are pure functions of the window, so the matrix is reproducible
//! byte-for-byte in every binding.

use crate::book::BookState;
use crate::dataset::Dataset;
use crate::frame::HeatmapData;
use crate::types::{bin_of, round8};

/// The start of the `bucket_ms`-wide bucket containing `ts`.
fn bucket_start(ts: i64, bucket_ms: i64) -> i64 {
    ts.div_euclid(bucket_ms) * bucket_ms
}

/// Build the book heatmap over the (already windowed) dataset.
///
/// Returns an empty heatmap when the window has no book events or the final
/// book is not two-sided.
#[must_use]
pub fn build(win: &Dataset, price_bin: f64, bucket_ms: i64, depth_levels: u32) -> HeatmapData {
    let empty = HeatmapData {
        time: Vec::new(),
        price: Vec::new(),
        intensity: Vec::new(),
    };
    if win.book.is_empty() {
        return empty;
    }

    // Price axis: depth_levels bins centred on the final book's mid.
    let mut final_book = BookState::new();
    for ev in &win.book {
        final_book.apply(ev);
    }
    let Some(ref_mid) = final_book.mid() else {
        return empty;
    };
    let center = bin_of(ref_mid, price_bin);
    let half = i64::from(depth_levels / 2);
    let price: Vec<f64> = (0..i64::from(depth_levels))
        .map(|j| round8(center + (j - half) as f64 * price_bin))
        .collect();

    // Time axis: one column per bucket from first to last book event.
    let first_bucket = bucket_start(win.book[0].ts, bucket_ms);
    let last_bucket = bucket_start(win.book[win.book.len() - 1].ts, bucket_ms);
    let n_buckets = ((last_bucket - first_bucket) / bucket_ms + 1) as usize;

    // Fold the book bucket by bucket, sampling resting liquidity at each column.
    let mut book = BookState::new();
    let mut next = 0; // index of the next unapplied book event
    let mut bucket = first_bucket;
    let mut time = Vec::with_capacity(n_buckets);
    let mut intensity = Vec::with_capacity(n_buckets);
    for _ in 0..n_buckets {
        let bucket_end = bucket + bucket_ms; // exclusive upper bound
        while next < win.book.len() && win.book[next].ts < bucket_end {
            book.apply(&win.book[next]);
            next += 1;
        }
        let row: Vec<f64> = price
            .iter()
            .map(|&lo| round8(book.resting_in_bin(lo, round8(lo + price_bin))))
            .collect();
        time.push(bucket);
        intensity.push(row);
        bucket += bucket_ms;
    }

    HeatmapData {
        time,
        price,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BookEvent, BookKind};

    fn snapshot(ts: i64, bids: Vec<[f64; 2]>, asks: Vec<[f64; 2]>) -> BookEvent {
        BookEvent {
            ts,
            kind: BookKind::Snapshot,
            bids,
            asks,
        }
    }

    #[test]
    fn single_snapshot_centres_on_the_mid() {
        // Mid = 100.5 -> bin 100.0 at index depth/2 = 2 of a 4-bin axis.
        let win = Dataset {
            book: vec![snapshot(1000, vec![[100.0, 2.0]], vec![[101.0, 3.0]])],
            ..Dataset::default()
        };
        let hm = build(&win, 1.0, 1000, 4);
        assert_eq!(hm.time, vec![1000]);
        assert_eq!(hm.price, vec![98.0, 99.0, 100.0, 101.0]);
        // Bin 100 holds the 2.0 bid, bin 101 the 3.0 ask, the rest empty.
        assert_eq!(hm.intensity, vec![vec![0.0, 0.0, 2.0, 3.0]]);
    }

    #[test]
    fn buckets_carry_the_book_forward() {
        // Two events two buckets apart; the empty middle bucket repeats state.
        let win = Dataset {
            book: vec![
                snapshot(1000, vec![[100.0, 1.0]], vec![[101.0, 1.0]]),
                snapshot(3000, vec![[100.0, 5.0]], vec![[101.0, 1.0]]),
            ],
            ..Dataset::default()
        };
        let hm = build(&win, 1.0, 1000, 2);
        assert_eq!(hm.time, vec![1000, 2000, 3000]);
        assert_eq!(hm.price, vec![99.0, 100.0]);
        // Bin 100 across buckets: 1.0 (first), 1.0 (carried), 5.0 (second).
        assert_eq!(hm.intensity[0], vec![0.0, 1.0]);
        assert_eq!(hm.intensity[1], vec![0.0, 1.0]);
        assert_eq!(hm.intensity[2], vec![0.0, 5.0]);
    }

    #[test]
    fn one_sided_book_has_no_mid_and_is_empty() {
        let win = Dataset {
            book: vec![snapshot(1000, vec![[100.0, 2.0]], vec![])],
            ..Dataset::default()
        };
        let hm = build(&win, 1.0, 1000, 4);
        assert!(hm.time.is_empty());
        assert!(hm.price.is_empty());
        assert!(hm.intensity.is_empty());
    }

    #[test]
    fn no_book_events_is_empty() {
        let hm = build(&Dataset::default(), 1.0, 1000, 4);
        assert!(hm.time.is_empty());
        assert!(hm.price.is_empty());
        assert!(hm.intensity.is_empty());
    }
}
