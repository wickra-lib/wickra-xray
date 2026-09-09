"""The scrubber equality, through the Python binding.

``frame_at(t)`` over the whole dataset must return exactly what ``frame``
returns over a dataset that ends at ``t``. That equality is what "scrub the
market like a video" means, and no golden fixture can check it: every blessed
frame is a full-window frame, so the cursor is never anywhere but the end.

The two routes are genuinely different inside the core -- one clips a loaded
window, the other never sees the later events -- so a window bound that is
inclusive on the wrong side, a bucket that rounds outward, or a panel that
keeps state past the cursor separates them. Byte equality of the serialized
frame is the strongest statement available here and the same one the golden
tests make, so the assertion is on the JSON rather than on a parsed subset.
"""

import json
import pathlib

import pytest

from wickra_xray import Xray

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"

# An event timestamp in the golden dataset (1000..24000 in 1000-unit steps),
# not a gap between two: the frame over the clipped dataset takes its cursor
# from the last event it holds, so a cut inside a gap would leave the two
# frames on different cursors and the comparison would be about nothing.
CUT = 12000


def _clip(dataset: dict, cut: int) -> tuple[dict, int, int]:
    """The dataset with every event after ``cut`` removed, plus the counts."""
    clipped: dict = {}
    kept = dropped = 0
    for stream, events in dataset.items():
        keep = [e for e in events if e["ts"] <= cut]
        kept += len(keep)
        dropped += len(events) - len(keep)
        clipped[stream] = keep
    return clipped, kept, dropped


@pytest.mark.skipif(not GOLDEN.exists(), reason="golden fixtures not present yet")
def test_frame_at_equals_a_dataset_that_ends_there() -> None:
    spec = (GOLDEN / "specs" / "multi_panel.json").read_text(encoding="utf-8")
    dataset = json.loads((GOLDEN / "data.json").read_text(encoding="utf-8"))
    clipped, kept, dropped = _clip(dataset, CUT)
    # A cut that keeps everything or nothing would compare a frame with itself.
    assert kept > 0 and dropped > 0, f"the cut at {CUT} kept {kept} and dropped {dropped}"

    scrubbed = Xray(spec)
    scrubbed.command(json.dumps({"cmd": "load", "dataset": dataset}))
    at_cut = scrubbed.command(json.dumps({"cmd": "frame_at", "ts": CUT}))

    truncated = Xray(spec)
    truncated.command(json.dumps({"cmd": "load", "dataset": clipped}))
    whole = truncated.command(json.dumps({"cmd": "frame"}))

    assert at_cut == whole


@pytest.mark.skipif(not GOLDEN.exists(), reason="golden fixtures not present yet")
def test_folding_to_the_end_reproduces_the_full_frame() -> None:
    """The upper bound is where an off-by-one hides: the spec leaves ``to_ts``
    open, so ``frame``'s cursor is the dataset's own end and ``frame_at`` asked
    for that same timestamp must not produce a different frame."""
    spec = (GOLDEN / "specs" / "multi_panel.json").read_text(encoding="utf-8")
    dataset = json.loads((GOLDEN / "data.json").read_text(encoding="utf-8"))
    end = max(e["ts"] for events in dataset.values() for e in events)

    xray = Xray(spec)
    xray.command(json.dumps({"cmd": "load", "dataset": dataset}))
    assert xray.command(json.dumps({"cmd": "frame_at", "ts": end})) == xray.command(
        json.dumps({"cmd": "frame"})
    )


@pytest.mark.skipif(not GOLDEN.exists(), reason="golden fixtures not present yet")
def test_a_midpoint_frame_reports_its_own_cursor() -> None:
    spec = (GOLDEN / "specs" / "multi_panel.json").read_text(encoding="utf-8")
    dataset = json.loads((GOLDEN / "data.json").read_text(encoding="utf-8"))

    xray = Xray(spec)
    xray.command(json.dumps({"cmd": "load", "dataset": dataset}))
    frame = json.loads(xray.command(json.dumps({"cmd": "frame_at", "ts": CUT})))
    assert frame["cursor_ts"] == CUT
