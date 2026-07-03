"""A runnable Python example: build a frame through the binding.

    pip install wickra-xray
    python examples/python/frame.py
"""

import json

from wickra_xray import Xray

SPEC = json.dumps(
    {
        "dataset_ref": "m",
        "symbol": "AAA",
        "panels": [{"kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000}],
    }
)

DATASET = {
    "trades": [
        {"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"},
        {"ts": 1400, "price": 101.8, "qty": 0.5, "side": "sell"},
    ]
}


def main() -> None:
    xray = Xray(SPEC)
    xray.command(json.dumps({"cmd": "load", "dataset": DATASET}))
    response = xray.command(json.dumps({"cmd": "frame"}))
    frame = json.loads(response)

    print(f"wickra-xray {Xray.version()}")
    print(response)
    print(f"  panels: {len(frame['panels'])}")


if __name__ == "__main__":
    main()
