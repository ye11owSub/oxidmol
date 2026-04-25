from pathlib import Path

import pytest
from cordyceps.utils.loading import load

DATA = Path(__file__).parents[3] / "tests" / "data"
MINIMAL_CIF = DATA / "minimal.cif"


def test_load_cif_returns_molecule():
    mol = load(str(MINIMAL_CIF))
    assert mol is not None


def test_load_unsupported_format_raises():
    with pytest.raises(ValueError, match="Unsupported format"):
        load("structure.xyz")


def test_load_unsupported_format_message_contains_suffix():
    with pytest.raises(ValueError, match=r"\.xyz"):
        load("structure.xyz")


def test_load_nonexistent_cif_raises():
    with pytest.raises(RuntimeError):
        load("nonexistent.cif")


def test_load_nonexistent_pdb_raises():
    with pytest.raises(RuntimeError):
        load("nonexistent.pdb")


def test_load_mmcif_extension():
    with pytest.raises(RuntimeError) as exc_info:
        load("structure.mmcif")
    assert "Unsupported format" not in str(exc_info.value)


def test_load_ent_extension():
    with pytest.raises(RuntimeError) as exc_info:
        load("structure.ent")
    assert "Unsupported format" not in str(exc_info.value)
