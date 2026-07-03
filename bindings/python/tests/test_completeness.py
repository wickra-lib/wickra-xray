"""Pin the public surface of the wickra_xray module and Xray class.

The class-surface guard mirrors the Node/R completeness checks; the module guard
pins the package's exported names (via ``__all__``) so a stray export — or a
dropped one — fails loudly, matching the exact-surface guard in the Node binding.
"""

import wickra_xray
from wickra_xray import Xray

EXPECTED_METHODS = {"command", "version"}
EXPECTED_EXPORTS = ["Xray", "__version__"]


def test_expected_methods_present() -> None:
    for name in EXPECTED_METHODS:
        assert hasattr(Xray, name), f"missing method: {name}"


def test_no_unexpected_public_methods() -> None:
    public = {name for name in dir(Xray) if not name.startswith("_")}
    assert public == EXPECTED_METHODS


def test_module_all_is_exact() -> None:
    assert wickra_xray.__all__ == EXPECTED_EXPORTS


def test_module_exposes_xray_and_version() -> None:
    assert isinstance(wickra_xray.Xray, type)
    assert isinstance(wickra_xray.__version__, str)
    assert wickra_xray.__version__ == Xray.version()
