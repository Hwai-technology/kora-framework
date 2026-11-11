# Kora Tooling, Editor & Infrastructure Integrations

This directory contains official integrations, editor plugins, shell completions, container configurations, and infrastructure automation templates for the **Kora** programming language and framework.

---

## 📁 Directory Structure

```
integrations/
├── completions/           # Shell autocompletion scripts
│   ├── kora.bash          # Bash completion
│   ├── _kora              # Zsh completion
│   └── kora.fish          # Fish completion
├── devcontainer/          # Cloud / Codespaces / DevContainer config
│   └── devcontainer.json  # VS Code & GitHub Codespaces dev environment
├── docker/                # Containerization manifests
│   ├── Dockerfile         # Multi-stage optimized Alpine container
│   └── docker-compose.yml # Full stack: Kora + PostgreSQL + Redis
├── github-action/         # CI/CD GitHub Action
│   └── action.yml         # setup-kora composite action for workflows
├── linux/                 # Linux desktop & MIME integration
│   └── kora.xml           # FreeDesktop XML MIME definition (.kora / .kr)
├── nvim/                  # Neovim integration
│   └── kora-lsp.lua       # Native Neovim LSP setup for `kora lsp`
├── pre-commit/            # Git hook integration
│   └── .pre-commit-hooks.yaml # Pre-commit hook for formatting & checking
├── systemd/               # Linux service deployment
│   └── kora-app.service   # Hardened production systemd service template
├── vim/                   # Vim / Vi editor integration
│   ├── ftdetect/kora.vim  # Filetype detection (*.kora, *.kr)
│   └── syntax/kora.vim    # Syntax highlighting rules
└── vscode/                # Visual Studio Code / Antigravity IDE Extension
    ├── package.json       # Extension manifest
    ├── language-configuration.json
    ├── syntaxes/          # TextMate syntax definitions
    ├── snippets/          # Language snippets
    └── icons/             # Custom file & module icons
```

---

## ⚡ 1. Shell Completions

### Bash
```bash
sudo cp integrations/completions/kora.bash /etc/bash_completion.d/kora
source /etc/bash_completion.d/kora
```

### Zsh
```zsh
mkdir -p ~/.zsh/completion
cp integrations/completions/_kora ~/.zsh/completion/
echo 'fpath=(~/.zsh/completion $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc
```

### Fish
```fish
mkdir -p ~/.config/fish/completions
cp integrations/completions/kora.fish ~/.config/fish/completions/
```

---

## 🛠️ 2. Editor Integrations

### Visual Studio Code / Antigravity IDE
Symlink or copy the extension directory into your VS Code extensions folder:
```bash
mkdir -p ~/.vscode/extensions/kora-lang
cp -r integrations/vscode/* ~/.vscode/extensions/kora-lang/
```

### Vim & Neovim
Copy the syntax and filetype detection files to your runtime directory:
```bash
# For Vim:
cp -r integrations/vim/* ~/.vim/

# For Neovim:
cp -r integrations/vim/* ~/.config/nvim/
cp integrations/nvim/kora-lsp.lua ~/.config/nvim/after/ftplugin/kora.lua
```

---

## 🐙 3. GitHub Actions (`setup-kora`)

In your repository's `.github/workflows/ci.yml`:
```yaml
steps:
  - uses: actions/checkout@v4
  - uses: Hwai-technology/kora-framework/integrations/github-action@main
  - run: kora check src/main.kora
```

---

## 🪝 4. Git Pre-Commit Hooks

Add Kora checks to your `.pre-commit-config.yaml`:
```yaml
repos:
  - repo: https://github.com/Hwai-technology/kora-framework
    rev: main
    hooks:
      - id: kora-fmt
      - id: kora-check
```

---

## 🐳 5. Docker & Production Deployment

### Running with Docker Compose:
```bash
cd integrations/docker
docker compose up -d
```

### Running as a Systemd Service (Linux Server):
```bash
sudo cp integrations/systemd/kora-app.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now kora-app
```

