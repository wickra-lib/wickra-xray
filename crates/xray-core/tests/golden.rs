//! Golden test: the core reproduces every blessed `expected/<spec>.json`
//! byte-for-byte.
//!
//! For each spec under `golden/specs/`, build an [`Xray`], load the shared
//! `golden/data.json`, ask for the full-window `frame`, and assert the reply
//! string equals the blessed `golden/expected/<spec>.json` exactly. The reply
//! is `serde_json::to_string(&frame)` — the same compact string every binding
//! returns from a `frame` command and the same string the CLI's `--format json`
//! prints — so this file is the in-core anchor of the cross-language
//! byte-equality guarantee. The blessed files carry a trailing newline from the
//! CLI's `println!`; the command reply does not, so the expectation is trimmed.

use std::fs;
use std::path::{Path, PathBuf};

use xray_core::Xray;

/// The repository-root `golden/` directory, resolved from this crate's manifest.
fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("golden")
}

#[test]
fn every_spec_reproduces_its_blessed_frame() {
    let golden = golden_dir();
    let dataset = fs::read_to_string(golden.join("data.json")).unwrap();

    let mut specs: Vec<PathBuf> = fs::read_dir(golden.join("specs"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    specs.sort();
    assert!(!specs.is_empty(), "no golden specs found");

    for spec_path in specs {
        let name = spec_path.file_stem().unwrap().to_str().unwrap();
        let spec_json = fs::read_to_string(&spec_path).unwrap();
        let expected = fs::read_to_string(golden.join("expected").join(format!("{name}.json")))
            .unwrap_or_else(|e| panic!("{name}: missing expected: {e}"));

        let mut xray = Xray::new(&spec_json).unwrap();
        let load = format!(r#"{{"cmd":"load","dataset":{dataset}}}"#);
        xray.command_json(&load).unwrap();
        let frame = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();

        assert_eq!(frame, expected.trim_end(), "{name}: golden mismatch");
    }
}
