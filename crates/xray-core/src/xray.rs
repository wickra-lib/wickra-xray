//! The `Xray` handle, the `command_json` FFI protocol and the pure
//! `build_frame` builder (§6.6).
//!
//! `Xray` holds an optional spec and a loaded dataset. Every binding drives it
//! through a single entry point, [`Xray::command_json`], whose envelope is
//! `{"cmd": "...", ...}` and whose reply is always a JSON string — a domain
//! error becomes `{"ok":false,"error":"..."}` rather than a panic or a thrown
//! exception. [`build_frame`] is the pure builder underneath: it windows the
//! dataset and folds each panel into an [`XrayFrame`].

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::dataset::Dataset;
use crate::error::{Error, Result};
use crate::frame::XrayFrame;
use crate::panels::build_panel_data;
use crate::spec::XraySpec;

/// The dataset bounds returned by the `bounds` command, for the scrubber.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    /// Earliest event timestamp across all streams.
    pub from_ts: i64,
    /// Latest event timestamp across all streams.
    pub to_ts: i64,
    /// Total number of events.
    pub count: usize,
}

/// A data-driven X-Ray: a spec plus a loaded dataset, driven via `command_json`.
#[derive(Clone, Debug, Default)]
pub struct Xray {
    spec: Option<XraySpec>,
    dataset: Dataset,
}

impl Xray {
    /// Create an X-Ray. `spec_json` may be `""` or `"{}"` for an empty handle
    /// (the spec is set later via `set_spec`); any other string is parsed and
    /// validated as an [`XraySpec`].
    pub fn new(spec_json: &str) -> Result<Self> {
        let spec = match spec_json.trim() {
            "" | "{}" => None,
            s => Some(XraySpec::from_json(s)?),
        };
        Ok(Self {
            spec,
            dataset: Dataset::default(),
        })
    }

    /// The crate version, e.g. `"0.1.0"`.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Replace the spec.
    pub fn set_spec(&mut self, spec: XraySpec) {
        self.spec = Some(spec);
    }

    /// Load a dataset, sorting every stream by timestamp. Returns the event
    /// count.
    pub fn load(&mut self, mut dataset: Dataset) -> Result<usize> {
        dataset.sort();
        let n = dataset.n_events();
        self.dataset = dataset;
        Ok(n)
    }

    /// The full-window frame (`cursor_ts = to_ts`, or the dataset end when the
    /// spec leaves `to_ts` open).
    pub fn frame(&self) -> Result<XrayFrame> {
        let spec = self.require_spec()?;
        let (_, hi, _) = self
            .dataset
            .bounds()
            .ok_or_else(|| Error::Data("no dataset".into()))?;
        let cursor = spec.to_ts.unwrap_or(hi);
        build_frame(&self.dataset, spec, cursor)
    }

    /// The frame folded up to `ts` (the scrubber path); the window end is
    /// `min(ts, to_ts)`.
    pub fn frame_at(&self, ts: i64) -> Result<XrayFrame> {
        let spec = self.require_spec()?;
        if self.dataset.bounds().is_none() {
            return Err(Error::Data("no dataset".into()));
        }
        build_frame(&self.dataset, spec, ts)
    }

    /// The dataset bounds for the scrubber.
    pub fn bounds(&self) -> Result<Bounds> {
        let (from_ts, to_ts, count) = self
            .dataset
            .bounds()
            .ok_or_else(|| Error::Data("no dataset".into()))?;
        Ok(Bounds {
            from_ts,
            to_ts,
            count,
        })
    }

    /// Clear the dataset, keeping the spec.
    pub fn reset(&mut self) {
        self.dataset = Dataset::default();
    }

    /// The single FFI entry point (§6.6). The reply is always a JSON string; a
    /// domain error is returned as `{"ok":false,"error":"..."}`.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        Ok(match self.dispatch(cmd_json) {
            Ok(reply) => reply,
            Err(e) => {
                let msg = serde_json::to_string(&e.to_string())
                    .unwrap_or_else(|_| "\"error\"".to_owned());
                format!(r#"{{"ok":false,"error":{msg}}}"#)
            }
        })
    }

    fn require_spec(&self) -> Result<&XraySpec> {
        self.spec
            .as_ref()
            .ok_or_else(|| Error::BadSpec("no spec set".into()))
    }

    /// Run one command, returning its JSON reply or an error to be wrapped.
    fn dispatch(&mut self, cmd_json: &str) -> Result<String> {
        let envelope: Value = serde_json::from_str(cmd_json)?;
        let cmd = envelope
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing 'cmd'".into()))?;
        match cmd {
            "set_spec" => {
                let spec_val = envelope
                    .get("spec")
                    .ok_or_else(|| Error::BadSpec("set_spec requires 'spec'".into()))?;
                let spec: XraySpec = serde_json::from_value(spec_val.clone())?;
                spec.validate()?;
                self.set_spec(spec);
                Ok(r#"{"ok":true}"#.to_owned())
            }
            "load" => {
                let ds_val = envelope
                    .get("dataset")
                    .ok_or_else(|| Error::BadSpec("load requires 'dataset'".into()))?;
                let dataset: Dataset = serde_json::from_value(ds_val.clone())?;
                let loaded = self.load(dataset)?;
                Ok(format!(r#"{{"ok":true,"loaded":{loaded}}}"#))
            }
            "frame_at" => {
                let ts = envelope
                    .get("ts")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| Error::BadSpec("frame_at requires integer 'ts'".into()))?;
                Ok(serde_json::to_string(&self.frame_at(ts)?)?)
            }
            "frame" => Ok(serde_json::to_string(&self.frame()?)?),
            "bounds" => Ok(serde_json::to_string(&self.bounds()?)?),
            "reset" => {
                self.reset();
                Ok(r#"{"ok":true}"#.to_owned())
            }
            "version" => Ok(format!(r#"{{"version":"{}"}}"#, Self::version())),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }
}

/// Build a frame from a dataset and spec, folded up to `cursor_ts`.
///
/// The window is `[from_ts, min(cursor_ts, to_ts)]` (a `None` spec bound is
/// unbounded on that side); the clipped copy is sorted so the fold is
/// deterministic, and each spec panel is built in order. The frame's
/// `cursor_ts` is the effective window end actually folded to.
pub fn build_frame(dataset: &Dataset, spec: &XraySpec, cursor_ts: i64) -> Result<XrayFrame> {
    spec.validate()?;
    let effective = spec.to_ts.map_or(cursor_ts, |to| cursor_ts.min(to));
    let mut win = dataset.window(spec.from_ts, Some(effective));
    win.sort();
    let panels = spec
        .panels
        .iter()
        .map(|panel| build_panel_data(panel, &win, effective))
        .collect::<Result<Vec<_>>>()?;
    Ok(XrayFrame {
        symbol: spec.symbol.clone(),
        cursor_ts: effective,
        panels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r#"{
        "dataset_ref": "mini", "symbol": "AAA",
        "panels": [ { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 } ]
    }"#;

    const DATASET: &str = r#"{ "trades": [
        { "ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy" },
        { "ts": 1400, "price": 101.8, "qty": 0.5, "side": "buy" }
    ] }"#;

    fn loaded() -> Xray {
        let mut xray = Xray::new(SPEC).unwrap();
        let dataset = Dataset::from_json(DATASET).unwrap();
        xray.load(dataset).unwrap();
        xray
    }

    #[test]
    fn version_is_the_crate_version() {
        assert_eq!(Xray::version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn frame_equals_frame_at_to_ts_byte_for_byte() {
        let xray = loaded();
        let full = serde_json::to_string(&xray.frame().unwrap()).unwrap();
        // to_ts is open, so the resolved cursor is the dataset end (1400).
        let at = serde_json::to_string(&xray.frame_at(1400).unwrap()).unwrap();
        assert_eq!(full, at);
        assert!(full.contains(r#""cursor_ts":1400"#));
    }

    #[test]
    fn frame_at_clips_the_window() {
        let xray = loaded();
        let early = xray.frame_at(1000).unwrap();
        assert_eq!(early.cursor_ts, 1000);
        // Only the ts=1000 trade is in range: bin 100 buy 2.0, no bin 101.
        let json = serde_json::to_string(&early).unwrap();
        assert!(json.contains(r#""price_bins":[100.0]"#));
    }

    #[test]
    fn command_json_runs_the_full_sequence() {
        let mut xray = Xray::new("").unwrap();
        assert_eq!(
            xray.command_json(&format!(r#"{{"cmd":"set_spec","spec":{SPEC}}}"#))
                .unwrap(),
            r#"{"ok":true}"#
        );
        assert_eq!(
            xray.command_json(&format!(r#"{{"cmd":"load","dataset":{DATASET}}}"#))
                .unwrap(),
            r#"{"ok":true,"loaded":2}"#
        );
        let bounds = xray.command_json(r#"{"cmd":"bounds"}"#).unwrap();
        assert_eq!(bounds, r#"{"from_ts":1000,"to_ts":1400,"count":2}"#);
        let frame = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();
        assert!(frame.contains(r#""kind":"footprint""#));
        assert_eq!(
            xray.command_json(r#"{"cmd":"version"}"#).unwrap(),
            format!(r#"{{"version":"{}"}}"#, Xray::version())
        );
    }

    #[test]
    fn reset_clears_the_dataset_but_keeps_the_spec() {
        let mut xray = loaded();
        assert_eq!(
            xray.command_json(r#"{"cmd":"reset"}"#).unwrap(),
            r#"{"ok":true}"#
        );
        // Dataset gone -> frame errors; spec still there.
        let frame = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();
        assert!(frame.contains(r#""ok":false"#));
        assert!(frame.contains("no dataset"));
    }

    #[test]
    fn unknown_command_is_an_error_json_not_a_panic() {
        let mut xray = Xray::new("").unwrap();
        let reply = xray.command_json(r#"{"cmd":"explode"}"#).unwrap();
        assert!(reply.contains(r#""ok":false"#));
        assert!(reply.contains("unknown cmd: explode"));
    }

    #[test]
    fn frame_without_dataset_is_an_error_json() {
        let mut xray = Xray::new(SPEC).unwrap();
        let reply = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();
        assert!(reply.contains(r#""ok":false"#));
        assert!(reply.contains("no dataset"));
    }

    #[test]
    fn build_frame_rejects_an_invalid_spec() {
        // price_bin <= 0 fails validation.
        let bad: XraySpec = serde_json::from_str(
            r#"{ "dataset_ref": "m", "symbol": "AAA",
                 "panels": [ { "kind": "footprint", "price_bin": 0.0 } ] }"#,
        )
        .unwrap();
        let err = build_frame(&Dataset::default(), &bad, 0);
        assert!(err.is_err());
    }
}
