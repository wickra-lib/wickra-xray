"""Cross-language golden: every binding must produce byte-identical frame JSON.

The fixtures live in the repository-root ``golden/`` directory (specs + a shared
dataset + expected responses), blessed from ``wickra-xray-core``. This binding
must reproduce them byte-for-byte.
"""

import json
import pathlib

from wickra_xray import Xray

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _spec_files() -> list[pathlib.Path]:
    specs = GOLDEN / "specs"
    if not specs.exists():
        return []
    return sorted(specs.glob("*.json"))


def test_golden_corpus_is_present() -> None:
    """A loop over an empty list checks nothing and passes.

    Without this the whole cross-language guarantee could evaporate from the
    Python side by moving a directory, and the suite would stay green.
    """
    assert _spec_files(), f"no golden specs under {GOLDEN / 'specs'}"


def test_golden_frame_is_byte_identical() -> None:
    # A plain loop rather than pytest.mark.parametrize, so the Python 3.9 CI
    # row can run this module without pytest (see run_without_pytest.py); the
    # spec name is in the message so a failure still says which case.
    dataset = json.loads((GOLDEN / "data.json").read_text(encoding="utf-8"))
    for spec_path in _spec_files():
        expected = (GOLDEN / "expected" / f"{spec_path.stem}.json").read_text(
            encoding="utf-8"
        )
        xray = Xray(spec_path.read_text(encoding="utf-8"))
        xray.command(json.dumps({"cmd": "load", "dataset": dataset}))
        response = xray.command(json.dumps({"cmd": "frame"}))
        assert response == expected.strip(), spec_path.stem
