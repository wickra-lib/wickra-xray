//! The run pipeline: load a spec and dataset, build a frame, render it (§2.3).

use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

use xray_core::{Dataset, Xray};
use xray_core::{
    DivergenceData, FootprintData, HeatmapData, LiqMapData, OrderedF64, PanelData, XrayFrame,
    XraySpec,
};

use crate::args::{Args, Format};

/// Load the spec, load the dataset, build the frame and render it.
pub fn run(args: &Args) -> Result<String, Box<dyn Error>> {
    let spec = load_spec(&args.spec)?;
    let dataset = load_dataset(args)?;

    let mut xray = Xray::new("")?;
    xray.set_spec(spec);
    xray.load(dataset)?;

    let frame = match args.at {
        Some(ts) => xray.frame_at(ts)?,
        None => xray.frame()?,
    };

    Ok(match args.format {
        Format::Json => serde_json::to_string(&frame)?,
        Format::Text => render_text(&frame),
    })
}

/// Parse the spec file, choosing TOML or JSON by extension (JSON by default).
fn load_spec(path: &Path) -> Result<XraySpec, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let is_toml = path.extension().and_then(|ext| ext.to_str()) == Some("toml");
    let spec = if is_toml {
        XraySpec::from_toml(&text)?
    } else {
        XraySpec::from_json(&text)?
    };
    Ok(spec)
}

/// Load the dataset from stdin (`--stdin`) or a directory (`--data`).
fn load_dataset(args: &Args) -> Result<Dataset, Box<dyn Error>> {
    if args.stdin {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        Ok(Dataset::from_json(&text)?)
    } else if let Some(dir) = &args.data {
        load_dir(dir)
    } else {
        Err("either --data <dir> or --stdin is required".into())
    }
}

/// Read a stream file's text if it exists, `None` otherwise.
fn read_if_exists(dir: &Path, name: &str) -> std::io::Result<Option<String>> {
    let path = dir.join(name);
    if path.exists() {
        Ok(Some(fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

/// Deserialize one stream file into a `Vec`, or an empty `Vec` when absent. The
/// element type is inferred from the `Dataset` field being assigned.
macro_rules! load_stream {
    ($dir:expr, $name:expr) => {
        match read_if_exists($dir, $name)? {
            Some(text) => serde_json::from_str(&text)?,
            None => Vec::new(),
        }
    };
}

/// Assemble a dataset from per-stream JSON files in `dir` (missing = empty).
fn load_dir(dir: &Path) -> Result<Dataset, Box<dyn Error>> {
    Ok(Dataset {
        candles: load_stream!(dir, "candles.json"),
        trades: load_stream!(dir, "trades.json"),
        book: load_stream!(dir, "book.json"),
        funding: load_stream!(dir, "funding.json"),
        oi: load_stream!(dir, "oi.json"),
        liquidations: load_stream!(dir, "liquidations.json"),
    })
}

/// A compact human-readable per-panel summary.
fn render_text(frame: &XrayFrame) -> String {
    let mut lines = vec![format!(
        "{} @ cursor {} ({} panels)",
        frame.symbol,
        frame.cursor_ts,
        frame.panels.len()
    )];
    for (idx, panel) in frame.panels.iter().enumerate() {
        lines.push(match panel {
            PanelData::Footprint(fp) => render_footprint(idx, fp),
            PanelData::BookHeatmap(hm) => render_heatmap(idx, hm),
            PanelData::LiquidationMap(lm) => render_liqmap(idx, lm),
            PanelData::FundingOiDivergence(dv) => render_divergence(idx, dv),
        });
    }
    lines.join("\n")
}

fn render_footprint(idx: usize, fp: &FootprintData) -> String {
    let mut bins: Vec<(f64, f64, f64)> = fp
        .price_bins
        .iter()
        .zip(&fp.buy_vol)
        .zip(&fp.sell_vol)
        .map(|((&price, &buy), &sell)| (price, buy, sell))
        .collect();
    // Descending by absolute delta (buy - sell).
    bins.sort_by(|a, b| (b.1 - b.2).abs().total_cmp(&(a.1 - a.2).abs()));
    let top: Vec<String> = bins
        .iter()
        .take(3)
        .map(|(price, buy, sell)| format!("{price}: buy {buy} sell {sell} (d {})", buy - sell))
        .collect();
    format!(
        "[{idx}] footprint - {} bins; top |delta|: {}",
        fp.price_bins.len(),
        top.join(", ")
    )
}

fn render_heatmap(idx: usize, hm: &HeatmapData) -> String {
    let peak = hm
        .intensity
        .iter()
        .flatten()
        .copied()
        .fold(0.0_f64, f64::max);
    format!(
        "[{idx}] book_heatmap - {}x{} grid; peak intensity {peak}",
        hm.time.len(),
        hm.price.len()
    )
}

fn render_liqmap(idx: usize, lm: &LiqMapData) -> String {
    use std::collections::BTreeMap;
    let mut clusters: BTreeMap<OrderedF64, f64> = BTreeMap::new();
    for event in &lm.events {
        *clusters.entry(OrderedF64(event.price_bin)).or_insert(0.0) += event.qty;
    }
    let biggest = clusters
        .iter()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map_or_else(String::new, |(bin, qty)| {
            format!("; biggest cluster bin {} (qty {qty})", bin.0)
        });
    format!(
        "[{idx}] liquidation_map - {} events{biggest}",
        lm.events.len()
    )
}

fn render_divergence(idx: usize, dv: &DivergenceData) -> String {
    let last = |series: &[f64]| series.last().copied().unwrap_or(0.0);
    format!(
        "[{idx}] funding_oi_divergence - {} buckets; last funding {} oi {} price {}",
        dv.time.len(),
        last(&dv.funding),
        last(&dv.oi),
        last(&dv.price)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r#"{ "dataset_ref": "m", "symbol": "AAA",
        "panels": [ { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 } ] }"#;
    const TRADES: &str = r#"[ { "ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy" },
        { "ts": 1400, "price": 101.8, "qty": 0.5, "side": "buy" } ]"#;

    #[test]
    fn render_text_summarizes_panels() {
        let mut xray = Xray::new(SPEC).unwrap();
        xray.load(Dataset::from_json(&format!(r#"{{ "trades": {TRADES} }}"#)).unwrap())
            .unwrap();
        let text = render_text(&xray.frame().unwrap());
        assert!(text.contains("AAA @ cursor 1400"));
        assert!(text.contains("footprint - 2 bins"));
    }

    #[test]
    fn run_loads_a_directory_and_renders_json() {
        let base = std::env::temp_dir().join("wickra-xray-run-dir-test");
        let data = base.join("data");
        fs::create_dir_all(&data).unwrap();
        fs::write(base.join("spec.json"), SPEC).unwrap();
        fs::write(data.join("trades.json"), TRADES).unwrap();

        let args = Args {
            spec: base.join("spec.json"),
            data: Some(data.clone()),
            stdin: false,
            at: None,
            format: Format::Json,
        };
        let out = run(&args).unwrap();
        assert!(out.contains(r#""kind":"footprint""#));
        assert!(out.contains(r#""cursor_ts":1400"#));

        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn missing_dataset_source_errors() {
        // A readable spec, but neither --data nor --stdin, hits the source guard.
        let base = std::env::temp_dir().join("wickra-xray-run-nosrc-test");
        fs::create_dir_all(&base).unwrap();
        let spec_path = base.join("spec.json");
        fs::write(&spec_path, SPEC).unwrap();
        let args = Args {
            spec: spec_path,
            data: None,
            stdin: false,
            at: None,
            format: Format::Text,
        };
        let err = run(&args).unwrap_err();
        assert!(err.to_string().contains("--data"));
        fs::remove_dir_all(&base).unwrap();
    }
}
