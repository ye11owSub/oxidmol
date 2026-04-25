import asyncio
from enum import StrEnum
from pathlib import Path

import httpx

from cordyceps.lsd import Molecule
from cordyceps.utils.loading import load

CACHE_DIR = Path.home() / ".cordyceps" / "cache"


class Format(StrEnum):
    BCIF = "bcif"
    CIF = "cif"
    PDB = "pdb"


_URLS = {
    "rcsb": {
        Format.BCIF: "https://files.rcsb.org/download/{id}.bcif",
        Format.CIF: "https://files.rcsb.org/download/{id}.cif",
        Format.PDB: "https://files.rcsb.org/download/{id}.pdb",
    },
    "pdbe": {
        # PDBe не отдаёт BCIF
        Format.CIF: "https://www.ebi.ac.uk/pdbe/entry-files/download/{id}.cif",
        Format.PDB: "https://www.ebi.ac.uk/pdbe/entry-files/download/{id}.pdb",
    },
}


def _urls_for(entry_id: str, fmt: Format) -> list[str]:
    """Build prioritized URL list: RCSB first, PDBe as fallback."""
    urls = []
    for source in ("rcsb", "pdbe"):
        if fmt in _URLS[source]:
            urls.append(_URLS[source][fmt].format(id=entry_id))
    return urls


def _cached_path(entry_id: str, fmt: Format) -> Path:
    """Return the local cache path for a given structure and format."""
    return CACHE_DIR / fmt.value / f"{entry_id}.{fmt.value}"


async def _download(urls: list[str]) -> bytes:
    """Try each URL in order, return content from the first successful one."""
    last_error: Exception | None = None

    async with httpx.AsyncClient(
        timeout=30.0,
        follow_redirects=True,
    ) as client:
        for url in urls:
            try:
                r = await client.get(url)
                r.raise_for_status()
            except httpx.HTTPError as e:
                last_error = e
            else:
                return r.content

    raise RuntimeError(f"Can't download. All sources aren't available : {last_error}")


async def _download_file(
    entry_id: str,
    fmt: Format,
    force: bool,
) -> str:
    """Download a structure file and return its local path."""
    cached = _cached_path(entry_id, fmt)

    if cached.exists() and not force:
        return str(cached)

    cached.parent.mkdir(parents=True, exist_ok=True)
    data = await _download(_urls_for(entry_id, fmt))
    cached.write_bytes(data)
    return str(cached)


async def _fetch_async(
    entry_id: str,
    fmt: Format,
    force: bool,
) -> Molecule:
    path = await _download_file(entry_id, fmt=fmt, force=force)
    return load(path)


async def _fetch_many_async(
    entry_ids: list[str],
    fmt: Format,
    force: bool,
) -> dict[str, Molecule | BaseException]:
    tasks = {entry_id: _fetch_async(entry_id, fmt=fmt, force=force) for entry_id in entry_ids}

    results = await asyncio.gather(*tasks.values(), return_exceptions=True)

    return {
        entry_id: mol for entry_id, mol in zip(tasks.keys(), results, strict=False) if not isinstance(mol, Exception)
    }


def fetch(
    entry_id: str,
    fmt: Format = Format.BCIF,
    force: bool = False,
) -> Molecule:
    """
    Download and parse a molecular structure.

    Args:
        entry_id: PDB entry identifier, e.g. "1CRN" or "1crn"
        fmt:      download format
        force:    re-download even if the file is already cached

    Returns:
        Parsed Molecule

    Raises:
        ValueError:   if entry_id is empty or too short
        RuntimeError: if all sources are unavailable
    """
    entry_id = entry_id.upper().strip()

    if not entry_id or len(entry_id) < 4:
        raise ValueError(f"Invalid entry ID: '{entry_id}'")

    return asyncio.run(_fetch_async(entry_id, fmt=fmt, force=force))


def fetch_many(
    entry_ids: list[str],
    fmt: Format = Format.BCIF,
    force: bool = False,
) -> dict[str, Molecule | BaseException]:
    """
    Download and parse multiple structures in parallel.

    Structures that fail are silently omitted from the result.

    Args:
        entry_ids: list of PDB entry identifiers
        fmt:       download format for all structures
        force:     re-download even if files are already cached

    Returns:
        Mapping of entry ID to Molecule for each successfully loaded structure.
    """
    return asyncio.run(_fetch_many_async(entry_ids, fmt=fmt, force=force))
