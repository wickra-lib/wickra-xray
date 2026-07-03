//! Funding/OI-divergence builder — three series on one time axis (§6.4.4).
//!
//! On a shared `bucket_ms` raster, each bucket carries the last observed
//! funding rate, open interest and price (last-observation-carry-forward): a
//! stream missing in a bucket keeps its previous value, and a stream not yet
//! seen at the start reads `0.0`. The three series are index-aligned and equal
//! length; the divergence itself (e.g. price up while OI falls) is derived by
//! the consumer.
//!
//! The price series is the candle `close` (carry-forward). §6.4.4 allows the
//! book mid as an alternative, but the MVP uses a single deterministic source.

use crate::dataset::Dataset;
use crate::frame::DivergenceData;
use crate::types::round8;

/// The start of the `bucket_ms`-wide bucket containing `ts`.
fn bucket_start(ts: i64, bucket_ms: i64) -> i64 {
    ts.div_euclid(bucket_ms) * bucket_ms
}

/// Build the funding/OI/price divergence over the (already windowed) dataset.
///
/// Returns empty series when the window carries none of funding, OI or candles.
#[must_use]
pub fn build(win: &Dataset, bucket_ms: i64) -> DivergenceData {
    // The shared axis spans every stream that feeds a series.
    let first_ts = [
        win.funding.first().map(|f| f.ts),
        win.oi.first().map(|o| o.ts),
        win.candles.first().map(|c| c.ts),
    ]
    .into_iter()
    .flatten()
    .min();
    let last_ts = [
        win.funding.last().map(|f| f.ts),
        win.oi.last().map(|o| o.ts),
        win.candles.last().map(|c| c.ts),
    ]
    .into_iter()
    .flatten()
    .max();
    let (Some(first_ts), Some(last_ts)) = (first_ts, last_ts) else {
        return DivergenceData {
            time: Vec::new(),
            funding: Vec::new(),
            oi: Vec::new(),
            price: Vec::new(),
        };
    };

    let first_bucket = bucket_start(first_ts, bucket_ms);
    let last_bucket = bucket_start(last_ts, bucket_ms);
    let n_buckets = ((last_bucket - first_bucket) / bucket_ms + 1) as usize;

    let mut time = Vec::with_capacity(n_buckets);
    let mut funding = Vec::with_capacity(n_buckets);
    let mut oi = Vec::with_capacity(n_buckets);
    let mut price = Vec::with_capacity(n_buckets);

    // Running carry-forward values and stream cursors.
    let (mut last_funding, mut last_oi, mut last_price) = (0.0_f64, 0.0_f64, 0.0_f64);
    let (mut fp, mut op, mut cp) = (0usize, 0usize, 0usize);
    let mut bucket = first_bucket;
    for _ in 0..n_buckets {
        let bucket_end = bucket + bucket_ms; // exclusive
        while fp < win.funding.len() && win.funding[fp].ts < bucket_end {
            last_funding = win.funding[fp].rate;
            fp += 1;
        }
        while op < win.oi.len() && win.oi[op].ts < bucket_end {
            last_oi = win.oi[op].oi;
            op += 1;
        }
        while cp < win.candles.len() && win.candles[cp].ts < bucket_end {
            last_price = win.candles[cp].close;
            cp += 1;
        }
        time.push(bucket);
        funding.push(round8(last_funding));
        oi.push(round8(last_oi));
        price.push(round8(last_price));
        bucket += bucket_ms;
    }

    DivergenceData {
        time,
        funding,
        oi,
        price,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::Candle;
    use crate::types::{FundingEvent, OiEvent};

    fn candle(ts: i64, close: f64) -> Candle {
        Candle {
            ts,
            open: close,
            high: close,
            low: close,
            close,
            volume: 0.0,
        }
    }

    #[test]
    fn carries_each_stream_forward() {
        let win = Dataset {
            candles: vec![candle(1000, 100.0), candle(3000, 110.0)],
            funding: vec![
                FundingEvent {
                    ts: 1000,
                    rate: 0.0001,
                },
                FundingEvent {
                    ts: 3000,
                    rate: 0.0002,
                },
            ],
            oi: vec![
                OiEvent {
                    ts: 1000,
                    oi: 500.0,
                },
                OiEvent {
                    ts: 2000,
                    oi: 600.0,
                },
            ],
            ..Dataset::default()
        };
        let d = build(&win, 1000);
        assert_eq!(d.time, vec![1000, 2000, 3000]);
        assert_eq!(d.funding, vec![0.0001, 0.0001, 0.0002]);
        assert_eq!(d.oi, vec![500.0, 600.0, 600.0]);
        assert_eq!(d.price, vec![100.0, 100.0, 110.0]);
    }

    #[test]
    fn missing_at_start_reads_zero() {
        // Candles from ts 1000, but funding only arrives at ts 3000.
        let win = Dataset {
            candles: vec![
                candle(1000, 100.0),
                candle(2000, 101.0),
                candle(3000, 102.0),
            ],
            funding: vec![FundingEvent {
                ts: 3000,
                rate: 0.0005,
            }],
            ..Dataset::default()
        };
        let d = build(&win, 1000);
        assert_eq!(d.time, vec![1000, 2000, 3000]);
        assert_eq!(d.funding, vec![0.0, 0.0, 0.0005]);
        assert_eq!(d.oi, vec![0.0, 0.0, 0.0]);
        assert_eq!(d.price, vec![100.0, 101.0, 102.0]);
    }

    #[test]
    fn empty_window_yields_empty_series() {
        let d = build(&Dataset::default(), 1000);
        assert!(d.time.is_empty());
        assert!(d.funding.is_empty());
        assert!(d.oi.is_empty());
        assert!(d.price.is_empty());
    }
}
