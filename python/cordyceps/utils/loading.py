from pathlib import Path

import gemmi

from cordyceps.lsd import Molecule


def load(path: str) -> Molecule:
    """
    Parse a molecular structure file into a Molecule.

    Supports .bcif (via gemmi), .cif and .pdb (via Rust/pdbtbx).
    This is the only place where format conversion happens.

    Args:
        path: path to a local structure file

    Returns:
        Parsed Molecule ready for rendering

    Raises:
        ValueError:   if the file format is not supported
        RuntimeError: if .bcif is provided but gemmi is not installed
    """
    p = Path(path)

    match p.suffix.lower():
        case ".bcif":
            return _load_bcif(p)
        case ".cif" | ".mmcif":
            return Molecule.from_cif(str(p))
        case ".pdb" | ".ent":
            return Molecule.from_pdb(str(p))
        case _:
            raise ValueError(f"Unsupported format: '{p.suffix}'. Supported: .bcif, .cif, .pdb")


def _load_bcif(path: Path) -> Molecule:
    """
    Load a BinaryCIF file via gemmi and pass it to the Rust parser.

    gemmi reads BCIF natively — no temporary files needed.
    """
    structure = gemmi.read_structure(str(path))

    cif_str = structure.make_mmcif_document().as_string()
    return Molecule.from_str(cif_str)
