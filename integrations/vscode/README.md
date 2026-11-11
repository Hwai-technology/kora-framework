# Kora Visual Studio Code & Antigravity IDE Extension

> Official language extension for the **Kora** programming language and framework (`.kora` and `.kr`).

---

## 🌟 Key Features

1. **⚡ Real-time Diagnostics & Architectural Checking**:
   - Runs compiler diagnostics in real-time with debouncing.
   - Enforces clean architecture rules (`E5001`–`E5004`) directly in the editor.

2. **▶ CodeLens Direct Execution (`▶ Run File` & `⚡ Check File`)**:
   - Interactive buttons placed above `fn main()` and top-level entrypoints.
   - Click to run your Kora app natively inside the integrated terminal without typing commands.

3. **🎨 Canonical Code Formatting (`Shift+Alt+F`)**:
   - Registered `DocumentFormattingEditProvider` powered by `kora fmt`.
   - Supports `editor.formatOnSave: true` for zero-friction formatting.

4. **🔍 Go To Definition (`F12` / `Ctrl+Click`)**:
   - Instant symbol navigation to declarations of `model`, `class`, `interface`, `enum`, `fn`, `let`, and `var`.
   - Navigates across local declarations and multi-file `import` statements.

5. **📑 Document Symbols & Outline View (`Ctrl+Shift+O`)**:
   - Renders a clean structural outline of all classes, models, interfaces, enums, and functions in the side panel and editor breadcrumbs.

6. **💡 Contextual Hover Documentation**:
   - Rich Markdown documentation displayed when hovering over keywords, decorators, types, and language primitives.

7. **✨ Intelligent Autocompletion**:
   - Suggestions for keywords, built-in types, ORM/validation decorators (`@table`, `@unique`, `@index`), and dynamic file symbols.

8. **🏷️ Dot-Free Custom Icons**:
   - Clean, modern file icons for `.kora`, `.kr`, models, modules, and tests.

---

## 🚀 Installation & Setup

### Automatic Installation
Run the root setup script:
```bash
./scripts/setup.sh
```

### Manual Installation
Copy the extension into your VS Code / Antigravity IDE extensions directory:
```bash
# For VS Code:
mkdir -p ~/.vscode/extensions/kora-vscode
cp -r * ~/.vscode/extensions/kora-vscode/

# For Antigravity IDE:
mkdir -p ~/.antigravity-ide/extensions/kora-lang.kora-vscode-0.1.0
cp -r * ~/.antigravity-ide/extensions/kora-lang.kora-vscode-0.1.0/
```

