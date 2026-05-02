from dataclasses import dataclass

from oxidmol.lsd import Molecule


@dataclass
class LoadMoleculeResult:
    molecule: Molecule


# Future results go here, e.g.:
# @dataclass
# class SelectResult:
#     selection: str

CommandResult = LoadMoleculeResult  # extend with | as new commands are added
