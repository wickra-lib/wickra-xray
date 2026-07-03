# Wickra X-Ray — Java

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

## Usage

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

## API

| Member | Description |
|--------|-------------|
| `new Xray(String specJson)` | Build an xray from a spec JSON (throws `IllegalArgumentException` on an invalid spec). |
| `String command(String cmdJson)` | Apply a command JSON, return the response JSON. |
| `static String version()` | The library version. |
| `close()` | Free the native handle (via `AutoCloseable`). |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON, not as an exception.

## License

`MIT OR Apache-2.0`.
