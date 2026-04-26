GRAMMAR = r"""
    // ─── program ───────────────────────────────────
    program: statement+

    statement: command NEWLINE
             | NEWLINE
             | COMMENT

    COMMENT: /#[^\n]*/

    // ─── command ─────────────────────────────────────
    command: fetch_cmd
           | load_cmd

    fetch_cmd:  "fetch"  entry_id ("," format_arg)?
    load_cmd:   "load"   path     ("," name)?

    // ─── terminals ────────────────────────────
    ENTRY_ID: /[A-Za-z0-9]+/
    PATH:     /[^\s,]+/

    // ─── cmd args ─────────────────────────────
    entry_id:    ENTRY_ID
    path:        PATH
    !format_arg: "cif" | "pdb" | "bcif"

    // ─── imports ────────────────────────────────────
    %import common.NEWLINE
    %import common.WS
    %ignore WS
"""
