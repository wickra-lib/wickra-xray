"""Smoke test: construct an xray, build a frame, parse the response."""

import json

from wickra_xray import Xray, __version__

SPEC = json.dumps(
    {
        "dataset_ref": "m",
        "symbol": "AAA",
        "panels": [{"kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000}],
    }
)

TRADES = [
    {"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"},
    {"ts": 1400, "price": 101.8, "qty": 0.5, "side": "buy"},
]


def test_frame_roundtrip() -> None:
    xray = Xray(SPEC)
    xray.command(json.dumps({"cmd": "load", "dataset": {"trades": TRADES}}))
    frame = json.loads(xray.command(json.dumps({"cmd": "frame"})))
    assert frame["symbol"] == "AAA"
    assert frame["cursor_ts"] == 1400
    assert frame["panels"][0]["kind"] == "footprint"


def test_version_matches_module() -> None:
    assert Xray.version() == __version__


def test_bad_spec_raises() -> None:
    try:
        Xray("not json")
    except ValueError:
        return
    raise AssertionError("expected ValueError for a malformed spec")
