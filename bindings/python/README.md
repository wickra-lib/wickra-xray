# Wickra X-Ray — Python

Python bindings for [`xray-core`](https://github.com/wickra-lib/wickra-xray),
built with [PyO3] and [maturin]. The surface mirrors every other Wickra binding:
build an `Xray` from a spec JSON, drive it with command JSONs, and read back
render frames.

## Install

```sh
pip install wickra-xray
```

## Usage

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

## Surface

- **`Xray(spec_json)`** builds an xray from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Raises `ValueError` on a malformed spec.
- **`xray.command(cmd_json)`** applies a command JSON (`set_spec`, `load`,
  `frame`, `frame_at`, `bounds`, `reset`, `version`) and returns the response
  JSON. Domain errors come back in-band as `{"ok":false,"error":...}`.
- **`Xray.version()`** returns the library version.

## Build from source

```sh
maturin develop --release
pytest -q
```

[PyO3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
