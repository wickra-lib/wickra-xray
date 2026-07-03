//! Criterion benchmarks for `build_frame`: how the fold scales with the event
//! count (1k / 10k / 100k) and the panel count (1 footprint panel vs. all four).
//! The same benchmark, run with and without the `parallel` feature, measures the
//! rayon path against the sequential one.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use xray_core::{
    build_frame, BookEvent, BookKind, Candle, Dataset, FundingEvent, LiqSide, LiquidationEvent,
    OiEvent, Side, Trade, XrayPanel, XraySpec,
};

/// A synthetic dataset of `events` bars: a deterministic sine price path with a
/// trade, a book event and a candle per bar, plus lower-cadence funding, open
/// interest and liquidation streams — enough to exercise all four panels.
fn dataset(events: usize) -> Dataset {
    let mut candles = Vec::with_capacity(events);
    let mut trades = Vec::with_capacity(events);
    let mut book = Vec::with_capacity(events);
    let mut funding = Vec::new();
    let mut oi = Vec::new();
    let mut liquidations = Vec::new();
    for index in 0..events {
        let step = f64::from(u32::try_from(index % 1_000_000).unwrap());
        let ts = i64::try_from(index).unwrap() * 1_000;
        let price = 100.0 + 10.0 * (step / 8.0).sin() + 0.05 * step;
        candles.push(Candle {
            ts,
            open: price,
            high: price + 0.3,
            low: price - 0.3,
            close: price,
            volume: 10.0,
        });
        trades.push(Trade {
            ts,
            price,
            qty: 1.0,
            side: if index % 2 == 0 {
                Side::Buy
            } else {
                Side::Sell
            },
        });
        let kind = if index == 0 {
            BookKind::Snapshot
        } else {
            BookKind::Diff
        };
        book.push(BookEvent {
            ts,
            kind,
            bids: vec![[price - 0.5, 5.0], [price - 1.0, 3.0]],
            asks: vec![[price + 0.5, 5.0], [price + 1.0, 3.0]],
        });
        if index % 4 == 1 {
            funding.push(FundingEvent { ts, rate: 0.0001 });
        }
        if index % 2 == 0 {
            oi.push(OiEvent {
                ts,
                oi: 500.0 + step,
            });
        }
        if index % 50 == 0 {
            liquidations.push(LiquidationEvent {
                ts,
                price,
                qty: 2.0,
                side: LiqSide::Long,
            });
        }
    }
    Dataset {
        candles,
        trades,
        book,
        funding,
        oi,
        liquidations,
    }
}

fn footprint_only() -> Vec<XrayPanel> {
    vec![XrayPanel::Footprint {
        price_bin: 1.0,
        bucket_ms: 60_000,
    }]
}

fn all_panels() -> Vec<XrayPanel> {
    vec![
        XrayPanel::Footprint {
            price_bin: 1.0,
            bucket_ms: 60_000,
        },
        XrayPanel::BookHeatmap {
            price_bin: 0.5,
            bucket_ms: 2_000,
            depth_levels: 8,
        },
        XrayPanel::LiquidationMap { price_bin: 1.0 },
        XrayPanel::FundingOiDivergence { bucket_ms: 2_000 },
    ]
}

fn spec(panels: Vec<XrayPanel>) -> XraySpec {
    XraySpec {
        dataset_ref: "bench".to_owned(),
        symbol: "SYM".to_owned(),
        from_ts: None,
        to_ts: None,
        panels,
    }
}

fn bench_build_frame(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build_frame");
    group.sample_size(10);
    for &events in &[1_000usize, 10_000, 100_000] {
        let data = dataset(events);
        let cursor = data.bounds().map_or(0, |(_, hi, _)| hi);
        for (label, panels) in [("1panel", footprint_only()), ("4panel", all_panels())] {
            let spec = spec(panels);
            group.throughput(Throughput::Elements(u64::try_from(events).unwrap()));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{events}ev_{label}")),
                &(&data, &spec, cursor),
                |bencher, (data, spec, cursor)| {
                    bencher
                        .iter(|| black_box(build_frame(black_box(data), black_box(spec), *cursor)));
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_build_frame);
criterion_main!(benches);
