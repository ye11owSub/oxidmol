# OxidMol

Molecular visualization tool inspired by PyMOL, built from the ground up on modern technology: a Rust/WebGPU rendering core exposed to Python via PyO3, with a PyQt6 UI on top.

The goal is a fast, hackable, open-source visualizer — GPU-accelerated rendering without legacy baggage, a scriptable Python interface, and a clean architecture that's easy to extend.

```bash
pip install oxidmol
```

## Development

```bash
git clone https://github.com/ye11owSub/oxidmol
cd oxidmol
uv sync --group dev
uv run maturin develop
uv run oxidmol
```

```bash
cargo test
uv run pytest -m "not gpu"
```
