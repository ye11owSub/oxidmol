import pytest

lsd = pytest.importorskip("oxidmol.lsd", reason="Rust bindings not available")


def test_get_backend_info() -> None:
    info = lsd.get_backend_info()
    assert isinstance(info, dict)
    assert "backend" in info
    assert "version" in info
    assert "supported_apis" in info


def test_molecule_from_str() -> None:
    info = lsd.get_backend_info()
    assert isinstance(info, dict)
    assert "backend" in info
    assert "version" in info
    assert "supported_apis" in info
