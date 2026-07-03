# Wickra X-Ray — C&#35;

C# bindings for the `wickra-xray` data-driven core over its C ABI hub
(source-generated P/Invoke). Build an `Xray` from a spec JSON, drive it with
command JSON, read back render frames — the same protocol as every other
binding.

## Install

```bash
dotnet add package Wickra.Xray
```

## Usage

```csharp
using Wickra.Xray;

const string spec = """
{"dataset_ref":"mini","symbol":"AAA",
"panels":[{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}
""";

using var xray = new Xray(spec);

xray.Command("""
{"cmd":"load","dataset":{"trades":[
{"ts":1000,"price":100.4,"qty":2.0,"side":"buy"}]}}
""");

string frame = xray.Command("""{"cmd":"frame"}""");
Console.WriteLine(frame);
Console.WriteLine(Xray.Version());
```

## API

| Member | Description |
|--------|-------------|
| `new Xray(string specJson)` | Build an xray from a spec JSON (throws `ArgumentException` on an invalid spec). |
| `string Command(string cmdJson)` | Apply a command JSON, return the response JSON. |
| `static string Version()` | The library version. |
| `Dispose()` | Free the native handle (via `IDisposable`). |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON. The native library is located by a
`DllImportResolver` that probes the default search paths and the Cargo `target/`
directory, validating each candidate with a sentinel export check.

## License

`MIT OR Apache-2.0`.
