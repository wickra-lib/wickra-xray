# Roadmap

`wickra-xray` is built out in phases, mirroring the proven structure of the
Wickra exchange, backtester, terminal and screener repos. Each phase lands as
reviewed, CI-green pull requests. Status below is updated as phases complete.

## Phases

0. **Scaffold** — workspace, governance, supply-chain config, `.github`
   scaffolding. *Done.*
1. **`xray-core`** — the `XraySpec`, the dataset fold (trades, book diffs,
   funding/OI), the four microstructure panels, the `XrayFrame` data-model and
   `frame_at(ts)`, with near-total coverage via inline tests. *Done.*
2. **`xray-cli`** — the reference `wickra-xray` binary: load a spec and a dataset,
   build a frame, render it as text or JSON. *Done.*
3. **Bindings** — the C ABI hub first, then native Python, Node and WASM, then C,
   C++, C#, Go, Java and R over the hub; each exposes the `Xray` handle +
   `command` + `version`, with a completeness guard. *Done* — and every surface is
   held to the header by `scripts/check_binding_surface.py` rather than by each
   language's own tests.
4. **Web renderer** — a Vue + Canvas front-end over the WASM binding that draws
   the four panels from the same `XrayFrame`, with a time-machine scrubber. *Done.*
5. **Golden harness + test rigor** — a fixed deterministic dataset and canonical
   specs whose blessed frames are the byte-exact, cross-language parity corpus,
   plus conformance, `frame_at`-equivalence, property and fuzz tests and a
   criterion benchmark suite. *Done* — the golden corpus and the scrubber equality
   are both checked from every binding, including the C ABI and WASM.
6. **ABI harness + examples** — cbindgen header sync-check and one runnable
   example per language, with a C/C++ CMake harness. *Done* — the `examples` CI
   job runs all eight language examples and parse-checks the WASM demo.
7. **CI/CD** — the full workflow matrix (all languages), OpenSSF Scorecard, Best
   Practices, link check, and the release workflow. *Done* — publishing runs
   behind a gate that refuses unless the tagged commit's CI is green, and behind a
   guard that refuses any ref that is not a `v*` tag.
8. **README, badges, docs** — the banner + badge treatment and the docs guides.
   *Done.*
9. **Deploy** — the web front-end published to a static host. *Open (USER-GO).*
   `web-deploy.yml` builds the renderer on every push that touches it and skips
   the deploy step until the Cloudflare Pages project and its token exist.

`0.1.0` is the first release: crates.io, PyPI, npm, NuGet, Maven Central, the
`wickra-xray-go` module mirror and r-universe.

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
