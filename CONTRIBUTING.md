# Contributing to Kora

Thank you for helping build Kora into a world-class backend programming language.

## Getting Started

1. Ensure you have Rust stable installed (`rustup update stable`).
2. Fork the repository and create your feature branch:
   ```bash
   git checkout -b feat/your-feature-name
   ```

## Pre-Commit Verification Checklist

Before submitting a Pull Request, run:
```bash
# 1. Format code
cargo fmt --all

# 2. Run Clippy
cargo clippy --workspace --all-targets -- -D warnings

# 3. Run workspace test suite
cargo test --workspace

# 4. Check all Kora real-world examples
for f in $(find examples/realworld_clean_architecture/src -name "*.kora"); do
  ./target/debug/kora check "$f"
done
```

## Architectural Guidelines
- Preserve the Struct-Free model: Use `model` for data schemas and `class` for services.
- Never use `undefined`: Use `null` exclusively for optional values.
- Adhere to directory purity: Never define inline enums or models inside services or controllers.
- Use `kebab-case` for public API contract fields.

## Automatic Contributor Recognition
Whenever your pull request is merged into `main`, our GitHub Actions automation dynamically updates the Contributors gallery in `README.md` with your profile and avatar.
