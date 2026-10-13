---
description: Run every check that CI runs (Rust and Python), without the GPU and app launch steps
---

Run these from the repository root in the activated virtual environment and report the result of each one.
Do not stop at the first failure: run all of them, then summarize what failed and why.

```bash
source .venv/bin/activate
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
uv run --no-sync maturin develop
ruff format --check python/
ruff check python/
mypy python/
QT_QPA_PLATFORM=offscreen pytest -m "not gpu"
```

Notes:
- `cargo test` needs the activated `.venv`, otherwise it fails to link with `-lpython3.9`.
- GPU tests print `skipping: no GPU adapter available` and still pass on machines without a GPU.
  Say so in the summary if you see it.
- Fix only what the user asked for. If a check fails for an unrelated reason, report it instead of fixing it.
