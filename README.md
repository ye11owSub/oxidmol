# OxidMol
## Install

```bash
pip install oxidmol
```

## Development

```bash
git clone https://github.com/ye11owSub/oxidmol
cd oxidmol
uv sync --group dev
uv run maturin develop   # rebuild the Rust extension after any src/**/*.rs change
uv run oxidmol
```

```bash
cargo test                 # Rust unit tests
uv run pytest -m "not gpu" # Python tests (GPU-marked tests need a real adapter)
uv run ruff check . && uv run mypy
```

## Web (wasm) build

The same Rust engine compiles to WebAssembly and renders into an HTML `<canvas>` via WebGPU (WebGL2 fallback) — no Python, no server.

```bash
wasm-pack build --target web --out-dir web/pkg
cd web && python3 -m http.server
```
