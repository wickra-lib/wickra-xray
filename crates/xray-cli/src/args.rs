//! Command-line arguments for `wickra-xray` (§2.2).

use std::path::PathBuf;

use clap::Parser;

/// Build an X-Ray frame from a spec and a recorded dataset.
#[derive(Parser, Debug)]
#[command(name = "wickra-xray", version, about)]
pub struct Args {
    /// Path to the spec file (JSON or TOML, chosen by extension).
    #[arg(long)]
    pub spec: PathBuf,

    /// Directory of per-stream JSON dataset files
    /// (`candles.json`, `trades.json`, `book.json`, `funding.json`,
    /// `oi.json`, `liquidations.json`; missing streams are empty).
    #[arg(long, conflicts_with = "stdin")]
    pub data: Option<PathBuf>,

    /// Read the whole dataset as one JSON object from stdin instead of `--data`.
    #[arg(long, conflicts_with = "data")]
    pub stdin: bool,

    /// Fold up to this timestamp (`frame_at`); omitted builds the full frame.
    #[arg(long)]
    pub at: Option<i64>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

/// How to render the frame.
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// A compact human-readable per-panel summary.
    Text,
    /// The frame serialized as JSON.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_options() {
        let args = Args::try_parse_from([
            "wickra-xray",
            "--spec",
            "s.json",
            "--data",
            "d",
            "--at",
            "1500",
            "--format",
            "json",
        ])
        .unwrap();
        assert_eq!(args.spec, PathBuf::from("s.json"));
        assert_eq!(args.data, Some(PathBuf::from("d")));
        assert!(!args.stdin);
        assert_eq!(args.at, Some(1500));
        assert_eq!(args.format, Format::Json);
    }

    #[test]
    fn format_defaults_to_text() {
        let args = Args::try_parse_from(["wickra-xray", "--spec", "s.json", "--stdin"]).unwrap();
        assert_eq!(args.format, Format::Text);
        assert!(args.stdin);
    }

    #[test]
    fn data_and_stdin_conflict() {
        let err =
            Args::try_parse_from(["wickra-xray", "--spec", "s.json", "--data", "d", "--stdin"]);
        assert!(err.is_err());
    }
}
