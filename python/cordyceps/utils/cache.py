import time

from cordyceps.utils.fetch import CACHE_DIR, Format, _cached_path


def info() -> dict[str, int | float | str | dict[str, Format]]:
    """
    Return current cache statistics.

    Returns:
        Dict with keys: size_mb, files, path, by_format
    """
    if not CACHE_DIR.exists():
        return {"size_mb": 0.0, "files": 0, "path": str(CACHE_DIR)}

    files = [f for f in CACHE_DIR.rglob("*") if f.is_file()]
    size = sum(f.stat().st_size for f in files)

    by_fmt = {}
    for fmt in Format:
        fmt_dir = CACHE_DIR / fmt.value
        fmt_files = list(fmt_dir.glob("*")) if fmt_dir.exists() else []
        by_fmt[fmt.value] = len(fmt_files)

    return {
        "size_mb": round(size / 1024 / 1024, 2),
        "files": len(files),
        "path": str(CACHE_DIR),
        "by_format": by_fmt,
    }


def clear(
    older_than_days: int | None = None,
    fmt: Format | None = None,
) -> int:
    """
    Remove cached files.

    Args:
        older_than_days: remove files older than N days (None = remove all)
        fmt:             remove only this format (None = remove all formats)

    Returns:
        Number of deleted files
    """
    if not CACHE_DIR.exists():
        return 0

    now = time.time()
    removed = 0

    search_dirs = [CACHE_DIR / fmt.value] if fmt else [CACHE_DIR / f.value for f in Format]

    for search_dir in search_dirs:
        if not search_dir.exists():
            continue

        for f in search_dir.glob("*"):
            if not f.is_file():
                continue

            if older_than_days is not None:
                age = (now - f.stat().st_mtime) / 86400
                if age < older_than_days:
                    continue

            f.unlink()
            removed += 1

    for d in sorted(CACHE_DIR.rglob("*"), reverse=True):
        if d.is_dir() and not any(d.iterdir()):
            d.rmdir()

    return removed


def is_cached(pdb_id: str, fmt: Format = Format.CIF) -> bool:
    """
    Check whether a structure is already present in the local cache.

    Args:
        pdb_id: PDB identifier
        fmt:    format to check for

    Returns:
        True if the file exists in cache, False otherwise
    """
    return _cached_path(pdb_id.upper(), fmt).exists()
