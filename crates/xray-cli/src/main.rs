//! The `wickra-xray` reference CLI.
//!
//! Loads an [`XraySpec`](xray_core::XraySpec) and a recorded dataset, builds a
//! frame through `xray-core`, and prints it as text or JSON. The run pipeline
//! (`run`) arrives in P-XRAY-2.3..2.4; this unit parses the arguments and echoes
//! the resolved run configuration.

mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let source = if args.stdin {
        "stdin".to_owned()
    } else {
        args.data
            .as_ref()
            .map_or_else(|| "<none>".to_owned(), |dir| dir.display().to_string())
    };
    let mode = args
        .at
        .map_or_else(|| "frame".to_owned(), |ts| format!("frame_at({ts})"));
    println!(
        "wickra-xray {}: spec={} source={source} mode={mode} format={:?}",
        xray_core::version(),
        args.spec.display(),
        args.format,
    );
}
