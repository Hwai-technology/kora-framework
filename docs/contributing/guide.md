# Contributing to Kora

Thank you for your interest in contributing to Kora.

## Development Workflow

1. **Fork & Branch**: Create a feature branch matching conventional patterns:
   - `feat/<topic>`
   - `fix/<issue>`
   - `docs/<topic>`
2. **Quality Checks**:
   - Run formatting: `cargo fmt --all`
   - Run linter: `cargo clippy --workspace --all-targets`
   - Run tests: `cargo test --workspace`
   - Check Kora files: `./target/debug/kora check <file>`
3. **Submit PR**: Open a pull request with a descriptive title using Conventional Commits.

## Automated Recognition
Upon merging your pull request, our automated GitHub Actions workflow will automatically recognize your contribution in `README.md`.

