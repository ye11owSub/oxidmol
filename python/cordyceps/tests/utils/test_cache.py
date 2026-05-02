import os
import time
from pathlib import Path
from typing import cast

import pytest
from cordyceps.utils import cache, fetch
from cordyceps.utils.fetch import Format


@pytest.fixture(autouse=True)
def tmp_cache(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Path:
    monkeypatch.setattr(cache, "CACHE_DIR", tmp_path)
    monkeypatch.setattr(fetch, "CACHE_DIR", tmp_path)
    return tmp_path


def _write_cached(cache_dir: Path, pdb_id: str, fmt: Format, content: str = "data") -> Path:
    p = cache_dir / fmt.value / f"{pdb_id}.{fmt.value}"
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(content)
    return p


def test_info_no_cache_dir() -> None:
    result = cache.info()
    assert result["size_mb"] == 0.0
    assert result["files"] == 0


def test_info_counts_files(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF, "x" * 1024 * 1024)  # 1 MB
    _write_cached(tmp_cache, "2DEF", Format.PDB, "x" * 512)

    result = cache.info()
    assert result["files"] == 2
    assert float(result["size_mb"]) > 0  # type: ignore[arg-type]


def test_info_by_format(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    _write_cached(tmp_cache, "2DEF", Format.CIF)
    _write_cached(tmp_cache, "3GHI", Format.PDB)

    result = cache.info()
    by_fmt = cast(dict[str, int], result["by_format"])
    assert by_fmt[Format.CIF] == 2
    assert by_fmt[Format.PDB] == 1


def test_is_cached_miss() -> None:
    assert cache.is_cached("1ABC") is False


def test_is_cached_hit(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    assert cache.is_cached("1ABC", Format.CIF) is True


def test_is_cached_case_insensitive(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    assert cache.is_cached("1abc", Format.CIF) is True


def test_is_cached_wrong_format(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    assert cache.is_cached("1ABC", Format.PDB) is False


def test_clear_all(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    _write_cached(tmp_cache, "2DEF", Format.PDB)

    removed = cache.clear()
    assert removed == 2
    assert cache.info()["files"] == 0


def test_clear_no_cache_returns_zero() -> None:
    assert cache.clear() == 0


def test_clear_by_format(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    _write_cached(tmp_cache, "2DEF", Format.PDB)

    removed = cache.clear(fmt=Format.CIF)
    assert removed == 1
    assert cache.is_cached("2DEF", Format.PDB) is True
    assert cache.is_cached("1ABC", Format.CIF) is False


def test_clear_older_than_keeps_new(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)

    removed = cache.clear(older_than_days=1)
    # File was just written — should NOT be removed
    assert removed == 0
    assert cache.is_cached("1ABC", Format.CIF) is True


def test_clear_older_than_removes_old(tmp_cache: Path) -> None:
    path = _write_cached(tmp_cache, "1ABC", Format.CIF)
    # Backdate mtime by 2 days
    old_time = time.time() - 2 * 86400

    os.utime(path, (old_time, old_time))

    removed = cache.clear(older_than_days=1)
    assert removed == 1


def test_clear_removes_empty_dirs(tmp_cache: Path) -> None:
    _write_cached(tmp_cache, "1ABC", Format.CIF)
    cache.clear()

    fmt_dir = tmp_cache / Format.CIF.value
    assert not fmt_dir.exists()
