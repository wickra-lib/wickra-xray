<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/release.svg)](https://github.com/wickra-lib/wickra-xray/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — C / C++

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for C / C++. `cargo build -p wickra-xray-c --release` — a prebuilt shared/static library plus a generated `wickra_xray.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-xray-core` as a tiny, JSON-shaped surface built as both a
`cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-xray/releases) — each archive
has `wickra_xray.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-xray-c --release
# -> target/release/libwickra_xray.{so,dylib} or wickra_xray.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/frame.c`](https://github.com/wickra-lib/wickra-xray/blob/main/examples/c/frame.c) is the runnable example the CI smoke job executes; in full:

```c
/* A minimal C example: build a frame through the wickra-xray C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_xray.h"

static const char *SPEC =
    "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":["
    "{\"kind\":\"footprint\",\"price_bin\":1.0,\"bucket_ms\":60000}]}";

static const char *LOAD =
    "{\"cmd\":\"load\",\"dataset\":{\"trades\":["
    "{\"ts\":1000,\"price\":100.4,\"qty\":2.0,\"side\":\"buy\"},"
    "{\"ts\":1400,\"price\":101.8,\"qty\":0.5,\"side\":\"sell\"}]}}";

static const char *FRAME = "{\"cmd\":\"frame\"}";

/* Length-out protocol: learn the length, then read into a caller buffer.
   Returns a malloc'd NUL-terminated string the caller must free, or NULL. */
static char *run(WickraXray *xray, const char *cmd) {
    int len = wickra_xray_command(xray, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_xray_command(xray, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraXray *xray = wickra_xray_new(SPEC);
    if (!xray) {
        fprintf(stderr, "failed to build xray\n");
        return 1;
    }

    char *loaded = run(xray, LOAD);
    if (!loaded) {
        wickra_xray_free(xray);
        return 1;
    }
    char *frame = run(xray, FRAME);
    if (!frame) {
        free(loaded);
        wickra_xray_free(xray);
        return 1;
    }

    printf("wickra-xray %s\n", wickra_xray_version());
    printf("loaded: %s\n", loaded);
    printf("frame: %s\n", frame);

    free(frame);
    free(loaded);
    wickra_xray_free(xray);
    return 0;
}
```

### Surface

```c
#include "wickra_xray.h"

WickraXray *wickra_xray_new(const char *spec_json);
void        wickra_xray_free(WickraXray *handle);
int32_t     wickra_xray_command(WickraXray *handle,
                                const char *cmd_json,
                                char *out, size_t cap);
const char *wickra_xray_version(void);
```

- **`wickra_xray_new`** builds an xray from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Returns `NULL` if the argument is null,
  not UTF-8, or not a valid spec.
- **`wickra_xray_free`** destroys a handle (null is a no-op).
- **`wickra_xray_command`** applies a command JSON and writes the response JSON
  into the caller's buffer using a length-out protocol (below).
- **`wickra_xray_version`** returns a static, NUL-terminated version string (do
  not free).

### Command / response protocol

Everything after construction goes through `wickra_xray_command`. Commands are
JSON objects with a `"cmd"` field: `set_spec`, `load`, `frame`, `frame_at`,
`bounds`, `reset`, `version`. Responses are JSON, e.g. an `XrayFrame` for
`frame`/`frame_at`, `{"from_ts":...,"to_ts":...,"count":N}` for `bounds`, or
`{"ok":true}` for a mutation.

The response is returned via a caller-owned buffer with a length-out protocol —
the callee never allocates memory the caller must free:

1. Call with `out = NULL`, `cap = 0` to learn the response length `len`
   (excluding the terminating NUL).
2. Allocate `len + 1` bytes and call again; the response plus a NUL is written.

Whenever `len < cap`, the response is written on that call, so a
sufficiently-large buffer needs only one call.

Return codes:

| Return   | Meaning                                             |
|----------|-----------------------------------------------------|
| `>= 0`   | Response length in bytes (excluding the NUL).       |
| `-1`     | A required pointer (`handle` or `cmd_json`) is null. |
| `-2`     | `cmd_json` is not valid UTF-8.                       |
| `-3`     | A panic was caught at the boundary.                 |

Domain errors (a bad spec, an unknown command) are **not** negative — they come
back in-band as `{"ok":false,"error":...}` JSON in the buffer.

### Header generation

`include/wickra_xray.h` is generated with [cbindgen] and committed; CI fails if
it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-xray-c --output include/wickra_xray.h
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/c)

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

[cbindgen]: https://github.com/mozilla/cbindgen
