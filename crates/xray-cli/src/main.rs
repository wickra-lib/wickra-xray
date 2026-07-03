//! The `wickra-xray` reference CLI.
//!
//! Loads an [`XraySpec`](xray_core::XraySpec) and a recorded dataset, builds a
//! frame through `xray-core`, and prints it as text or JSON. Argument parsing
//! (`args`) and the run pipeline (`run`) arrive in P-XRAY-2.2..2.4; this initial
//! unit reports the core version so the crate is a real, runnable member.

fn main() {
    println!("wickra-xray {}", xray_core::version());
}
