from pathlib import Path
from unittest.mock import AsyncMock, MagicMock, patch

import httpx
import oxidmol.utils.fetch as fetch_mod
import pytest
from oxidmol.utils.fetch import Format, _cached_path, _urls_for, fetch


def test_urls_for_cif_has_rcsb_first() -> None:
    urls = _urls_for("1CRN", Format.CIF)
    assert urls[0].startswith("https://files.rcsb.org")


def test_urls_for_cif_has_pdbe_fallback() -> None:
    urls = _urls_for("1CRN", Format.CIF)
    assert any("ebi.ac.uk" in u for u in urls)


def test_urls_for_pdb_has_rcsb() -> None:
    urls = _urls_for("1CRN", Format.PDB)
    assert any("rcsb.org" in u for u in urls)
    assert any("1CRN" in u for u in urls)


def test_urls_for_contains_entry_id() -> None:
    urls = _urls_for("4HHB", Format.CIF)
    assert all("4HHB" in u for u in urls)


def test_cached_path_structure() -> None:
    p = _cached_path("1CRN", Format.CIF)
    assert p.name == "1CRN.cif"
    assert p.parent.name == "cif"


def test_cached_path_pdb() -> None:
    p = _cached_path("1CRN", Format.PDB)
    assert p.suffix == ".pdb"


def test_fetch_empty_id_raises() -> None:
    with pytest.raises(ValueError, match="Invalid entry ID"):
        fetch("")


def test_fetch_short_id_raises() -> None:
    with pytest.raises(ValueError, match="Invalid entry ID"):
        fetch("1CR")


def test_fetch_normalises_to_uppercase(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """Lower-case ID gets uppercased before cache lookup."""
    monkeypatch.setattr(fetch_mod, "CACHE_DIR", tmp_path)

    # Write a pre-cached file under the uppercased name
    cached = tmp_path / "cif" / "1CRN.cif"
    cached.parent.mkdir(parents=True)
    cached.write_text("data_1CRN\n")

    mock_mol = MagicMock()
    with patch("oxidmol.utils.fetch.load", return_value=mock_mol):
        result = fetch("1crn", fmt=Format.CIF)

    assert result is mock_mol


@pytest.mark.asyncio
async def test_download_returns_content() -> None:
    mock_response = MagicMock()
    mock_response.content = b"data_TEST\n"
    mock_response.raise_for_status = MagicMock()

    mock_client = AsyncMock()
    mock_client.__aenter__ = AsyncMock(return_value=mock_client)
    mock_client.__aexit__ = AsyncMock(return_value=False)
    mock_client.get = AsyncMock(return_value=mock_response)

    with patch("httpx.AsyncClient", return_value=mock_client):
        result = await fetch_mod._download(["https://example.com/test.cif"])

    assert result == b"data_TEST\n"


@pytest.mark.asyncio
async def test_download_falls_back_to_second_url() -> None:
    ok_response = MagicMock()
    ok_response.content = b"ok"
    ok_response.raise_for_status = MagicMock()

    fail_response = MagicMock()
    fail_response.raise_for_status = MagicMock(
        side_effect=httpx.HTTPStatusError("404", request=MagicMock(), response=MagicMock())
    )

    mock_client = AsyncMock()
    mock_client.__aenter__ = AsyncMock(return_value=mock_client)
    mock_client.__aexit__ = AsyncMock(return_value=False)
    mock_client.get = AsyncMock(side_effect=[fail_response, ok_response])

    with patch("httpx.AsyncClient", return_value=mock_client):
        result = await fetch_mod._download(["https://fail.com", "https://ok.com"])

    assert result == b"ok"


@pytest.mark.asyncio
async def test_download_raises_when_all_fail() -> None:
    fail_response = MagicMock()
    fail_response.raise_for_status = MagicMock(
        side_effect=httpx.HTTPStatusError("503", request=MagicMock(), response=MagicMock())
    )

    mock_client = AsyncMock()
    mock_client.__aenter__ = AsyncMock(return_value=mock_client)
    mock_client.__aexit__ = AsyncMock(return_value=False)
    mock_client.get = AsyncMock(return_value=fail_response)

    with patch("httpx.AsyncClient", return_value=mock_client), pytest.raises(RuntimeError, match="Can't download"):
        await fetch_mod._download(["https://fail1.com", "https://fail2.com"])


def test_fetch_many_returns_dict(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(fetch_mod, "CACHE_DIR", tmp_path)

    for pdb_id in ("1ABC", "2DEF"):
        cached = tmp_path / "cif" / f"{pdb_id}.cif"
        cached.parent.mkdir(parents=True, exist_ok=True)
        cached.write_text(f"data_{pdb_id}\n")

    mock_mol = MagicMock()
    with patch("oxidmol.utils.fetch.load", return_value=mock_mol):
        result = fetch_mod.fetch_many(["1ABC", "2DEF"], fmt=Format.CIF)

    assert set(result.keys()) == {"1ABC", "2DEF"}
