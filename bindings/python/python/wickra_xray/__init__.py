"""Wickra X-Ray — the data-driven market-microstructure frame core.

Build an :class:`Xray` from a spec JSON, drive it with command JSONs, and read
back render frames. The same command protocol crosses every language binding, so
this Python front-end drives the exact same core as the native CLI.
"""

from ._wickra_xray import Xray, __version__

__all__ = ["Xray", "__version__"]
