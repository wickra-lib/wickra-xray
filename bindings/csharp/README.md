<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/nuget.svg)](https://www.nuget.org/packages/Wickra.Xray)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — C#

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for C#. `dotnet add package Wickra.Xray` — prebuilt native library, no system dependencies.**

.NET bindings for [`wickra-xray`](https://github.com/wickra-lib/wickra-xray) over
the C ABI hub, via source-generated P/Invoke. Build an `Xray` from a spec JSON,
drive it with command JSON and read back render frames — the same protocol the
CLI and every other binding speak.

## Install

```bash
dotnet add package Wickra.Xray
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_xray`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS. Licensed under `MIT OR Apache-2.0`.

### Building from this repository (contributors)

| Path | What it is |
| --- | --- |
| `WickraXray/` | The published package. Its own `README.md` is the long description NuGet renders. |
| `WickraXray.Tests/` | xUnit suite: golden parity against the shared fixtures and the X-Ray command protocol. |

See [`WickraXray/README.md`](https://github.com/wickra-lib/wickra-xray/blob/main/bindings/csharp/WickraXray/README.md) for the full API walk-through,
and [`examples/csharp/`](https://github.com/wickra-lib/wickra-xray/blob/main/examples/csharp) for a runnable program.

## Quick start

```csharp
using Wickra.Xray;

const string spec = """
{"dataset_ref":"m","symbol":"AAA",
"panels":[{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}
""";

using var xray = new Xray(spec);
xray.Command("""{"cmd":"load","dataset":{ … }}""");
string frame = xray.Command("""{"cmd":"frame"}""");
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/csharp)

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
