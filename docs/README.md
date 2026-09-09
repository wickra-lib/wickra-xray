# Documentation

These pages are the guides that live beside the code, because they describe how
this repository behaves and have to change in the same commit the behaviour does.

| Page | What it answers |
|------|-----------------|
| [PANELS.md](PANELS.md) | The four panels — footprint, order-book heatmap, liquidation map, funding/OI divergence — and what each one puts in a frame |
| [DATASETS.md](DATASETS.md) | The recorded inputs: trades, order-book diffs, liquidations, funding and open interest, and the shapes they arrive in |
| [RENDERING.md](RENDERING.md) | Why a frame is a render data-model and not a draw call, and what a front-end is expected to do with one |
| [STREAMING.md](STREAMING.md) | Replaying a dataset against building it in one pass, and where the two are identical |
| [Cookbook.md](Cookbook.md) | Worked specs |
| [ARCHITECTURE.md](ARCHITECTURE.md) | The internals: how a dataset is folded into a frame |

The API reference for each language is generated from the source rather than
committed here — `cargo doc` for Rust, the `.d.ts` beside the Node binding, the
docstrings in the Python module, the C header. Keeping a second copy in this
repository would drift from the code that generates it, and a reader opening
`docs/` would have no way to tell which copy was current.

The X-Ray's site, with the in-browser demo and the benchmark figures, is at
<https://xray.wickra.org>. The indicator library the frames are computed over
documents itself at <https://docs.wickra.org>.

What stays here is what a generator cannot produce: the meaning of a field, the
reason a case is refused rather than answered, and the worked examples.

Elsewhere in the repository:

- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — the crate and binding layout
- [`../BENCHMARKS.md`](../BENCHMARKS.md) — what is measured and how
- [`../golden/README.md`](../golden/README.md) — the cross-language corpus and how to regenerate it
- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — how to build, test and propose a change
- [`../THREAT_MODEL.md`](../THREAT_MODEL.md) — what the X-Ray does and does not touch
