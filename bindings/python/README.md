<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/pypi.svg)](https://pypi.org/project/wickra-xray/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — Python

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for Python. `pip install wickra-xray` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for [`wickra-xray-core`](https://github.com/wickra-lib/wickra-xray),
built with [PyO3] and [maturin]. The surface mirrors every other Wickra binding:
build an `Xray` from a spec JSON, drive it with command JSONs, and read back
render frames.

## Install

```bash
pip install wickra-xray
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

### Building from this repository (contributors)

```sh
maturin develop --release
pytest -q
```

## Quick start

```python
import json
from wickra_xray import Xray

spec = json.dumps({
    "dataset_ref": "mini", "symbol": "AAA",
    "panels": [{"kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000}],
})

xray = Xray(spec)
xray.command(json.dumps({"cmd": "load", "dataset": {
    "trades": [{"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"}],
}}))
frame = json.loads(xray.command(json.dumps({"cmd": "frame"})))
print(frame["symbol"], frame["cursor_ts"])
```

### Surface

- **`Xray(spec_json)`** builds an xray from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Raises `ValueError` on a malformed spec.
- **`xray.command(cmd_json)`** applies a command JSON (`set_spec`, `load`,
  `frame`, `frame_at`, `bounds`, `reset`, `version`) and returns the response
  JSON. Domain errors come back in-band as `{"ok":false,"error":...}`.
- **`Xray.version()`** returns the library version.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/python)

Wickra X-Ray ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-xray/blob/main/SECURITY.md>.

## Disclaimer

Wickra X-Ray is analysis software: it computes microstructure views over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-MIT) at your option.

[PyO3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
