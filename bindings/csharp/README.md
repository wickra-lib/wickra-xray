# wickra-xray (C#)

.NET bindings for [`wickra-xray`](https://github.com/wickra-lib/wickra-xray) over
the C ABI hub, via source-generated P/Invoke. Build an `Xray` from a spec JSON,
drive it with command JSON and read back render frames — the same protocol the
CLI and every other binding speak.

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

Requires .NET 8+. The native library (`wickra_xray`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS. Licensed under `MIT OR Apache-2.0`.

## Layout

| Path | What it is |
| --- | --- |
| `WickraXray/` | The published package. Its own `README.md` is the long description NuGet renders. |
| `WickraXray.Tests/` | xUnit suite: golden parity against the shared fixtures and the X-Ray command protocol. |

See [`WickraXray/README.md`](https://github.com/wickra-lib/wickra-xray/blob/main/bindings/csharp/WickraXray/README.md) for the full API walk-through,
and [`examples/csharp/`](https://github.com/wickra-lib/wickra-xray/blob/main/examples/csharp) for a runnable program.
