# Golden fixtures

The golden fixtures pin the X-Ray's output byte-for-byte. They are generated
once and replayed everywhere: the Rust core, the CLI and every language binding
must reproduce `expected/<spec>.json` **exactly**. Byte equality holds across all
languages because each binding returns the core's `command_json` string
verbatim — there is no per-language JSON re-formatting or float reformat.

> **Do not edit any file under `golden/` by hand.** Regenerate them with the
> bless command below and commit the result.

## Layout

| Path | What |
|------|------|
| `data.json` | The canonical dataset — the input every binding feeds to a `load` command. |
| `specs/*.json` | The five canonical specs: one per panel kind plus `multi_panel` with all four. |
| `expected/<spec>.json` | The byte-exact `XrayFrame` produced by `set_spec`/`load`/`frame` for each spec. |

## Data formula

`data.json` is deterministic, generated over `N = 24` one-second bars
(`ts(i) = i * 1000` ms, `i = 1..24`). The price of bar `i` is

```
price(i) = base + amp * sin(i / k) + drift * i
```

with `base = 100.0`, `amp = 2.0`, `k = 6.0`, `drift = 0.05`, rounded to eight
decimals. From that price the six streams are:

- **candles** — one per bar: `close = price(i)`, `open = price(i-1)`,
  `high = max(open, close) + 0.3`, `low = min(open, close) - 0.3`,
  `volume = 10 + (i mod 5)`.
- **trades** — one per bar at `price(i)`, `qty = 1 + (i mod 3) * 0.5`, aggressor
  side alternating by parity (`buy` for odd `i`, `sell` for even).
- **book** — a snapshot at `ts = 1000` (two bid and two ask levels around
  `floor(price(1))`), then a formula-based diff at every later bar adjusting one
  bid and one ask level.
- **funding** — at bars where `i mod 4 == 1`: `rate = 0.0001 + sin(i/k) * 0.00005`.
- **oi** — at even bars: `oi = 500 + i * 10`.
- **liquidations** — sparse, at `i in {7, 14, 21}`: `price(i)`,
  `qty = 1 + (i mod 3)`, `side = long` when `sin(i/k) >= 0` else `short`.

## Bless

Regenerate every `expected/<spec>.json` from the dataset (byte-exact) and commit
the result:

```sh
for spec in footprint book_heatmap liquidation_map funding_oi_divergence multi_panel; do
  cargo run -q -p wickra-xray -- --spec "golden/specs/$spec.json" --stdin --format json \
    < golden/data.json > "golden/expected/$spec.json"
done
```

The CLI's `--format json` output is exactly `serde_json::to_string(&frame)`, the
same string every binding returns from a `frame` command — so the blessed files
are the cross-language byte-equality target.
