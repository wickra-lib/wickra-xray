# Roadmap

`wickra-xray` is built out in phases, mirroring the proven structure of the
Wickra exchange, backtester, terminal and screener repos. Each phase lands as
reviewed, CI-green pull requests. Status below is updated as phases complete.

## Phases

0. **Scaffold** — workspace, governance, supply-chain config, `.github`
   scaffolding. *In progress.*
1. **`xray-core`** — the `XraySpec`, the dataset fold (trades, book diffs,
   funding/OI), the four microstructure panels, the `XrayFrame` data-model and
   `frame_at(ts)`, with near-total coverage via inline tests.
2. **`xray-cli`** — the reference `wickra-xray` binary: load a spec and a dataset,
   build a frame, render it as text or JSON.
3. **Bindings** — the C ABI hub first, then native Python, Node and WASM, then C,
   C++, C#, Go, Java and R over the hub; each exposes the `Xray` handle +
   `command` + `version`, with a completeness guard.
4. **Web renderer** — a Vue + Canvas front-end over the WASM binding that draws
   the four panels from the same `XrayFrame`, with a time-machine scrubber.
5. **Golden harness + test rigor** — a fixed deterministic dataset and canonical
   specs whose blessed frames are the byte-exact, cross-language parity corpus,
   plus conformance, `frame_at`-equivalence, property and fuzz tests and a
   criterion benchmark suite.
6. **ABI harness + examples** — cbindgen header sync-check and one runnable
   example per language, with a C/C++ CMake harness.
7. **CI/CD** — the full workflow matrix (all languages), OpenSSF Scorecard, Best
   Practices, link check, and the release workflow.
8. **README, badges, docs** — the banner + badge treatment and the docs guides.
9. **Deploy** — the web front-end published to a static host (USER-gated).

## Beyond 1.0

- Additional panels and richer per-panel controls as the corpus grows.
- A live dataset sourced from an exchange feed (the optional `live` feature),
  still read-only.

## Non-goals

- **Indicator code in this repository.** Indicators come from the `wickra-core`
  registry; xray composes them, it does not reimplement them.
- **Renderer commands as core output.** A view is a serde `XrayFrame` data-model,
  never draw calls, so it crosses the C ABI and WASM unchanged and every front-end
  draws the same frame.
- **A hosted service or stored credentials.** xray runs locally; it holds no
  order-secret material and places no orders.
