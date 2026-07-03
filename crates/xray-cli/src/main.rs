//! The `wickra-xray` reference CLI.
//!
//! Loads an [`XraySpec`](xray_core::XraySpec) and a recorded dataset (a
//! directory of per-stream JSON files or a dataset JSON on stdin), builds a
//! frame through `xray-core`, and prints it as text or JSON.

mod args;
mod run;

use args::Args;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("wickra-xray: {err}");
            ExitCode::FAILURE
        }
    }
}
