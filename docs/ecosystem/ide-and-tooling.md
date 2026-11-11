# IDE & Tooling Ecosystem

Kora provides comprehensive editor integration, Language Server Protocol (LSP) support, shell completions, container tooling, and CI/CD actions.

---

## 1. Visual Studio Code & Antigravity IDE

The official Kora IDE extension (`integrations/vscode/`) provides:
- **Syntax Highlighting**: Full TextMate grammar for `.kora` and `.kr` files.
- **CodeLens Superpowers**: Interactive `▶ Run File` and `⚡ Check File` buttons above entry points.
- **Go to Definition**: `F12` and `Ctrl+Click` navigation across local declarations and relative `import` statements.
- **Document Formatting**: `Shift+Alt+F` and `editor.formatOnSave` integration calling canonical `kora fmt`.
- **Breadcrumbs & Outline**: Full Document Symbol Provider (`Ctrl+Shift+O`).
- **Rich Snippets**: High-speed templates for `model`, `class`, `enum`, `interface`, `route`, and Clean Architecture components.

---

## 2. Language Server Protocol (LSP) Daemon

Run the LSP server directly for any LSP-compatible editor:

```bash
kora lsp
```

### Capabilities:
- `textDocument/completion`: Keywords, types, model names, and snippet completions.
- `textDocument/hover`: Type information and docstring hovers.
- `textDocument/formatting`: Automatic canonical formatting.

---

## 3. Neovim & Vim Integration

- **Neovim (Lua LSP)**: Place `integrations/nvim/kora-lsp.lua` into `~/.config/nvim/after/ftplugin/kora.lua`.
- **Vim**: Install syntax and filetype detection:
  ```bash
  cp -r integrations/vim/ftdetect ~/.vim/
  cp -r integrations/vim/syntax ~/.vim/
  ```

---

## 4. Shell Auto-Completions

Auto-completions are installed automatically by `./scripts/setup.sh` or `./scripts/update.sh`:
- **Bash**: `integrations/completions/kora.bash`
- **Zsh**: `integrations/completions/_kora`
- **Fish**: `integrations/completions/kora.fish`

---

## 5. Docker & Multi-Container Stacks

Production-ready multi-stage Dockerfile and Docker Compose configurations are included in `integrations/docker/`:

```bash
cd integrations/docker
docker compose up -d
```

Launches:
- **Kora App**: Lightweight Alpine-based native binary container (< 15 MB).
- **PostgreSQL 16**: Enterprise relational database.
- **Redis 7**: Cache and session store.

---

## 6. GitHub Actions CI/CD Composite Action

Use the official composite GitHub Action in `.github/workflows/ci.yml`:

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: ./integrations/github-action
    with:
      version: 'latest'
  - run: kora fmt --check
  - run: kora check src/main.kora
  - run: kora build src/main.kora
```

