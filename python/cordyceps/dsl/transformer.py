from lark import Transformer, v_args

from cordyceps import fetch, load
from cordyceps.dsl.commands import CommandResult, LoadMoleculeResult
from cordyceps.utils.fetch import Format


@v_args(inline=True)
class CordycepsTransformer(Transformer):

    def entry_id(self, token: str) -> str:
        return str(token)

    def format_arg(self, token: str) -> str:
        return str(token)

    def path(self, path: str) -> str:
        return str(path)

    def fetch_cmd(self, entry_id: str, *args: str) -> CommandResult:
        fmt = Format(args[0]) if args else Format.CIF
        return LoadMoleculeResult(molecule=fetch(entry_id, fmt=fmt))

    def load_cmd(self, path: str) -> CommandResult:
        return LoadMoleculeResult(molecule=load(path))
