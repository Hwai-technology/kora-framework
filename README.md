# Kora

<p align="center">
  <img src="integrations/vscode/icon.svg" width="100" height="100" alt="Kora Logo" />
</p>

<p align="center">
  <strong>Write Less. Build Faster. Run Smarter.</strong><br>
  <em>A modern compiled backend language and framework powered by Rust.</em>
</p>

<p align="center">
  <a href="#vision"><img src="https://img.shields.io/badge/vision-TS%20%2B%20Rust%20%2B%20Go-00D2FF" alt="Vision" /></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT-0047FF" alt="License" /></a>
  <a href="#status"><img src="https://img.shields.io/badge/status-active%20development-00FFA3" alt="Status" /></a>
</p>

---

## 🌟 The Kora Vision

Kora is engineered around one central philosophy:

> **The approachable developer experience and clean syntax of TypeScript, the raw performance and memory safety of Rust, and the simple concurrency model of Go — unified in a single backend language and framework.**

Backend developers shouldn't have to choose between developer speed and system performance. Kora compiles directly to native Rust binaries with zero garbage collection pauses and zero manual borrow-checker friction.

---

## 🚀 Key Language Features

### 1. Hybrid Object & Data Model (Struct-Free)
Kora eliminates `struct` confusion in favor of two distinct, purposeful constructs:
- **`model`**: Declarative data entities and API DTOs with schema attributes (`@unique`, `@index`, `@required`, `@email`, `@min`, `@max`, `@default`).
- **`class`**: Stateful business logic, controllers, and services with constructor parameter promotion in `init(...)`.
- **`interface`**: Polymorphic contracts verified strictly at compile time.

```kora
model User {
    @unique
    @required
    id: Int

    @unique
    @index
    @required
    @email
    email: String

    @default("customer")
    role: String

    @min(0)
    balance: Int
}

interface UserRepository {
    fn findByEmail(email: String) -> Option<User>
    fn save(user: User) -> Result<User, String>
}

class UserService implements UserRepository {
    repo: UserRepository

    init(repo: UserRepository) {
        this.repo = repo
    }

    fn findByEmail(email: String) -> Option<User> {
        if email == "" {
            return null
        }
        return this.repo.findByEmail(email)
    }

    fn save(user: User) -> Result<User, String> {
        return this.repo.save(user)
    }
}
```

### 2. First-Class `null` & Strict Exclusion of `undefined`
Kora introduces **`null`** as the universal representation of value absence (`Option<T>`).
- Writing `undefined` generates a compile error (`E2005`), eliminating dual-null ambiguity.
- Works seamlessly with direct comparisons and pattern matching:

```kora
let authHeader = req.header("authorization")

if authHeader == null {
    return Err("Missing required authorization header")
}

return match authHeader {
    Some(token) => Ok(token),
    null => Err("Header was empty"),
}
```

### 3. Automatic Constructor & Visibility Modifiers
Classes support `public`, `private`, `protected`, and `readonly` field modifiers, as well as automatic parameter promotion inside `init(...)`.

### 4. Enforced `kebab-case` API Serialization
All API payloads, route responses, and DTOs automatically serialize with `kebab-case` keys (`user-id`, `status-code`, `total-amount`), guaranteeing API consistency across client integrations.

### 5. Architectural Purity Enforcement (Compiler Diagnostics)
The Kora compiler (`kora check`) guards directory boundaries at build time:
- **`E5001`**: Files in `enums/` cannot contain functions or routes.
- **`E5002`**: Files in `models/` or `interfaces/` cannot contain executable server logic or routes.
- **`E5003`**: Controllers and services cannot define inline enums or models; they must be imported.
- **`E5004`**: `main.kora` cannot define inline models or enums.

---

## 🏛️ Compiler Pipeline & Architecture

```mermaid
flowchart LR
    A["Kora Source Code (.kora)"] --> B["kora-lexer\nToken Stream"]
    B --> C["kora-parser\nAbstract Syntax Tree"]
    C --> D["kora-hir\nName & Scope Resolution"]
    D --> E["kora-ty\nType & Purity Checker"]
    E --> F["kora-codegen\nSafe Native Rust"]
    F --> G["Native Binary\n(Zero GC, High Throughput)"]
```

---

## 📦 Workspace Architecture

All components live in a modular Cargo workspace under `crates/`:

| Crate | Role |
|:---|:---|
| `kora-lexer` | High-speed tokenizer with keyword and attribute scanning. |
| `kora-ast` | Abstract Syntax Tree node definitions. |
| `kora-parser` | Recursive-descent parser with diagnostics. |
| `kora-hir` | High-level Intermediate Representation and symbol resolution. |
| `kora-ty` | Static type system, interface verification, and purity checker. |
| `kora-codegen` | Transpilation from Kora AST to safe native Rust code. |
| `kora-http` | Async HTTP engine powered by Axum and Tokio. |
| `kora-orm` | Compile-time query and schema integration. |
| `kora-realtime`| WebSocket channels and domain event streaming. |
| `kora-cli` | Command-line interface (`check`, `build`, `run`, `codegen`). |

---

## 🛠️ Quickstart Guide

### Prerequisites
- [Rust](https://rustup.rs/) (stable toolchain)
- Cargo

### Building the Toolchain
```bash
# Clone the repository
git clone https://github.com/kora-lang/kora.git
cd kora

# Build Kora CLI
cargo build --bin kora

# Verify installation
./target/debug/kora --help
```

### Checking and Compiling Files
```bash
# Type check a Kora file
./target/debug/kora check examples/hello.kora

# Generate native Rust code
./target/debug/kora codegen examples/hello.kora
```

### Running the Clean Architecture Showcase
```bash
# Verify all 40 architectural files in the real-world showcase
for f in $(find examples/realworld_clean_architecture/src -name "*.kora"); do
  ./target/debug/kora check "$f"
done
```

---

## 🤖 Continuous Quality & Automation

Kora includes an enterprise GitHub Actions workflow suite:
- **CI Pipeline (`ci.yml`)**: Automated formatting (`rustfmt`), linting (`clippy`), unit tests, and compiler verification on every pull request.
- **PR Conventions (`pr-validation.yml`)**: Conventional Commits validation for titles and branches.
- **Native Auto-Merge (`auto-merge.yml`)**: Automated merge queue for approved pull requests passing all checks.
- **Automated Contributor Recognition (`contributors.yml`)**: Automatically updates the contributors gallery upon pull request merge.
- **AI Code Review Agent (`ai-code-review.yml`)**: Automated senior technical reviewer evaluating diffs for Rust correctness, compiler safety, performance, and security.

---

## 👥 Contributors

A huge thank you to everyone helping build Kora into the next-generation backend language and framework!

<!-- CONTRIBUTORS:START -->
<!-- Automatically maintained by GitHub Actions -->
<a href="https://github.com/Eng-Ahmet/Eng-Ahmet"><img src="https://avatars.githubusercontent.com/Eng-Ahmet?v=4&s=64" width="48" height="48" alt="Eng-Ahmet" title="Eng-Ahmet" style="border-radius: 50%; margin: 4px;" /></a>
<p><a href="https://github.com/Eng-Ahmet/Eng-Ahmet"><strong>Eng-Ahmet</strong></a></p>
<!-- CONTRIBUTORS:END -->

---

## 📄 License

Kora is open-source software licensed under the [MIT License](LICENSE).
