# Threat Model

`wickra-xray` is analysis software. It folds recorded market data into
microstructure views and places no orders, opens no authenticated connections on
its default path, and holds no secret key material. The attack surface is
correspondingly narrow: it is dominated by the parsing of **untrusted input** — an
`XraySpec` and a dataset supplied by the caller — as it crosses the C ABI and WASM
boundary.

## Assets

- The **`XraySpec` and dataset** a caller supplies. These are inputs, not secrets,
  but a malformed or hostile one must never crash or corrupt the host.
- The **integrity and determinism** of the `XrayFrame`: the same spec and dataset
  must always produce the same frame, in every language.
- The **host process** embedding a binding. Building a frame must not be able to
  take it down (panic across FFI, unbounded allocation) or read memory it should
  not.

There is intentionally **no secret asset** on the default path — no API keys, no
credentials, no order flow.

## Trust boundaries

- **Caller → core.** Everything arriving through `Xray::command` (spec, dataset,
  command) is untrusted and validated before use.
- **Binding → C ABI hub.** The hub is the one place `unsafe` is allowed. It wraps
  every call in `catch_unwind`, guards null pointers, and uses a length-out
  buffer protocol so no panic or invalid pointer crosses into C / Go / C# / Java
  / R.
- **Optional `live` feature.** Only this pulls `wickra-exchange` to read a public
  data feed; it adds a network read but still no credentials or orders.

## Guarantees the code is held to

- `unsafe_code = "forbid"` workspace-wide; only `bindings/c` re-allows it locally.
- No panic crosses the FFI boundary; errors are returned as JSON, never as an
  abort.
- Parsing is bounded and total — a hostile spec or dataset yields an error, not
  an unbounded allocation or a hang.
- `frame_at(ts)` is deterministic: the same dataset and timestamp always yield the
  same `XrayFrame`, and because each binding returns the core's response verbatim,
  that frame is byte-identical in every language.

## Out of scope

- Incorrect panel mathematics — a functional bug, handled through normal issues
  and tests, not a vulnerability.
- Vulnerabilities in third-party crates, which are tracked and triaged through
  `deny.toml` and `osv-scanner.toml`.
- Resource exhaustion a caller inflicts on **their own** process by deliberately
  feeding an enormous dataset; the core bounds its own allocations but cannot
  bound the caller's data volume.
