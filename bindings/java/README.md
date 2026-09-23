<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-xray)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — Java

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for Java. `org.wickra:wickra-xray` — prebuilt native library inside the jar, no JNI, no system dependencies.**

JVM bindings for the `wickra-xray` data-driven core over its C ABI hub
(FFM / Panama, `java.lang.foreign`). Build an `Xray` from a spec JSON, drive it
with command JSON, read back render frames — the same protocol as every other
binding.

## Requirements

- Java 22+ (the Foreign Function & Memory API is stable since 22).
- Run with `--enable-native-access=ALL-UNNAMED`.
- The native library (`wickra_xray`) must be resolvable — either on the library
  path or via the `native.lib.dir` system property pointing at the directory
  that holds `libwickra_xray.{so,dylib}` / `wickra_xray.dll`.

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-xray</artifactId>
  <version>0.1.5</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-xray:0.1.5")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

## Quick start

```java
import org.wickra.xray.Xray;

String spec = """
    {"dataset_ref":"mini","symbol":"AAA",
    "panels":[{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}""";

try (Xray xray = new Xray(spec)) {
    xray.command("""
        {"cmd":"load","dataset":{"trades":[
        {"ts":1000,"price":100.4,"qty":2.0,"side":"buy"}]}}""");
    System.out.println(xray.command("{\"cmd\":\"frame\"}"));
}
System.out.println(Xray.version());
```

### API

| Member | Description |
|--------|-------------|
| `new Xray(String specJson)` | Build an xray from a spec JSON (throws `IllegalArgumentException` on an invalid spec). |
| `String command(String cmdJson)` | Apply a command JSON, return the response JSON. |
| `static String version()` | The library version. |
| `close()` | Free the native handle (via `AutoCloseable`). |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON, not as an exception.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/java)

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
