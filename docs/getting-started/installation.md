# Kora Installation & Toolchain Setup

## Prerequisites
- Rust stable toolchain (1.78 or newer)
- Cargo package manager
- Git

## Building from Source

```bash
# Clone the repository
git clone https://github.com/kora-lang/kora.git
cd kora

# Build Kora CLI in release mode
cargo build --release --bin kora

# Verify binary execution
./target/release/kora --version
```

## Adding Kora to System PATH

```bash
# Add release target directory to PATH in ~/.bashrc or ~/.zshrc
export PATH="$HOME/Desktop/Projects/Kora/target/release:$PATH"
```

## Available CLI Commands

| Command | Usage | Description |
|:---|:---|:---|
| `kora check <file.kora>` | `kora check main.kora` | Validates syntax, types, and architectural purity rules. |
| `kora codegen <file.kora>` | `kora codegen main.kora` | Generates safe, zero-cost native Rust code. |
| `kora build <file.kora>` | `kora build main.kora` | Compiles source file to a standalone native executable. |
| `kora run <file.kora>` | `kora run main.kora` | Compiles and executes the backend server. |
| `kora fmt <file.kora>` | `kora fmt main.kora` | Formats Kora code according to language guidelines. |

