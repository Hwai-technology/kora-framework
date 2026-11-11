# kora-auth

> **Kora Authentication, RBAC & Token Security**

Provides enterprise authentication: Argon2id password hashing with unique cryptographic salts, HMAC-SHA256 JWT tokens, and RBAC guards.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Security Subsystem — Protects endpoints and data access.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `PasswordAuth`
- `JwtSigner`
- `RbacGuard`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Constant-time comparison prevents timing side-channels. Rejects tampered payloads, signature forgery, and key mismatches.

---

## 🚀 How to Contribute & Improve

Add OAuth2 / OIDC provider integration and WebAuthn (Passkeys) hardware security key support.

### Testing this Crate
```bash
cargo test -p kora-auth
cargo clippy -p kora-auth --all-targets -- -D warnings
```
