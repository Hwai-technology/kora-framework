# kora-pkg

> **Kora Distributed Package Manager (kora-pkg)**

Manages distributed dependencies, SemVer 2.0.0 constraint solving, cryptographic SHA-256 lockfile (kora.lock) generation, and publishing.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Package Tooling — Powers kora pkg <install|add|remove|list|publish>.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `PackageManager`
- `SemVer`
- `KoraManifest`
- `KoraLockfile`
- `RegistryIndexEntry`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Strict path traversal validation (rejects ../, /, \0). Enforces SHA-256 integrity checksums on all packages.

---

## 🚀 How to Contribute & Improve

Add GPG package signature verification and decentralized IPFS/S3 package registry mirror fallbacks.

### Testing this Crate
```bash
cargo test -p kora-pkg
cargo clippy -p kora-pkg --all-targets -- -D warnings
```
