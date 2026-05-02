import httpx
import pytest


@pytest.fixture(autouse=True)
def no_real_http(monkeypatch: pytest.MonkeyPatch) -> None:
    """Block any accidental real HTTP calls. Tests that need HTTP must patch httpx.AsyncClient explicitly."""

    def _blocked(*_args: object, **_kwargs: object) -> None:
        raise RuntimeError("Real HTTP calls not allowed in tests — patch httpx.AsyncClient")

    monkeypatch.setattr(httpx, "AsyncClient", _blocked)
