# Contributing to HealDep

Thank you for your interest in contributing!  
HealDep is an open-source project and we welcome all kinds of contributions — from bug reports and feature requests to code improvements and documentation.

## Code of Conduct
This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to contribute

### Reporting a bug
1. Check [existing issues](https://github.com/Nersisiian/HealDep/issues) to avoid duplicates.
2. If the bug is new, create a new issue using the “Bug report” template.
3. Include:
   - HealDep version (`healdep --version`)
   - Operating system
   - Steps to reproduce
   - Expected vs actual behaviour

### Suggesting a feature
1. Check [existing issues](https://github.com/Nersisiian/HealDep/issues) and [discussions](https://github.com/Nersisiian/HealDep/discussions).
2. Open a new issue with the “Feature request” template.
3. Describe the problem you want to solve and how your suggestion helps.

### Making a code change
1. **Fork** the repository.
2. Create a new branch: `git checkout -b feat/my-feature` or `fix/my-fix`.
3. Write your code and tests.
4. Run `cargo fmt --all` and `cargo clippy --all-targets --all-features`.
5. Commit with a clear message (e.g., `fix: handle empty dependency list`).
6. Push and open a Pull Request.

### Development setup
```bash
git clone https://github.com/Nersisiian/HealDep.git
cd healdep
cargo build --release
python -m venv .venv
.venv\Scripts\Activate.ps1   # on Windows
pip install -r python/requirements.txt
```

### Project structure
- `crates/healdep` — main CLI
- `crates/healdep-synthesizer` — shim generator
- `crates/healdep-sandbox` — build/test sandbox
- `crates/healdep-registry` — shim registry
- `python/` — Python analysis scripts and web dashboard

### Questions?
Feel free to open a [discussion](https://github.com/Nersisiian/HealDep/discussions) or ask in an issue.

Thank you for helping HealDep grow! 🩺
