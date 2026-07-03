# Cookbook

Short, runnable recipes. Each drives the same core through the JSON command
protocol; the CLI examples assume you have built the workspace (`cargo build`).

## Build a footprint from the CLI

```bash
cargo run -p wickra-xray -- \
  --spec golden/specs/footprint.json --stdin --format json < golden/data.json
```

The `--format json` output is exactly the bytes every binding returns from a
`frame` command. Drop `--format json` for a human-readable summary.

## All four panels at once

Use the `multi_panel` spec — one footprint, one book heatmap, one liquidation map
and one funding/OI divergence:

```bash
cargo run -p wickra-xray -- \
  --spec golden/specs/multi_panel.json --stdin --format json < golden/data.json
```

The frame's `panels` array is in spec order, so `panels[0]` is the footprint,
`panels[1]` the heatmap, and so on.

## Scrub to a point in time (Python)

```python
import json
from wickra_xray import Xray

spec = json.dumps({"dataset_ref": "m", "symbol": "AAA",
                   "panels": [{"kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000}]})
x = Xray(spec)
x.command(json.dumps({"cmd": "load", "dataset": {"trades": [
    {"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"},
    {"ts": 1400, "price": 101.8, "qty": 0.5, "side": "sell"}]}}))

bounds = json.loads(x.command('{"cmd":"bounds"}'))
mid = (bounds["from_ts"] + bounds["to_ts"]) // 2
early = x.command(json.dumps({"cmd": "frame_at", "ts": mid}))  # only ts <= mid
full  = x.command('{"cmd":"frame"}')                            # everything
```

## Add your own panel to a spec

A spec's `panels` is just a list. Append another kind and the frame grows a
matching `PanelData`:

```json
{
  "dataset_ref": "m",
  "symbol": "AAA",
  "panels": [
    { "kind": "footprint", "price_bin": 0.5, "bucket_ms": 30000 },
    { "kind": "liquidation_map", "price_bin": 1.0 }
  ]
}
```

## Check the version

```bash
cargo run -p wickra-xray -- --version
```

or, from any binding, `{"cmd":"version"}` → `{"version":"0.1.0"}`.

## See also

[Architecture](ARCHITECTURE.md) · [Panels](PANELS.md) · [Datasets](DATASETS.md) ·
[Streaming](STREAMING.md) · [Rendering](RENDERING.md).
