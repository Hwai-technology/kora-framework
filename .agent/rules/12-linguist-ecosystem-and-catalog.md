# Rule 12: GitHub Linguist Ecosystem & 2,000 Production Examples Matrix

---

## 1. Context & Objective
To achieve official language recognition and automated syntax highlighting on GitHub, Kora must satisfy GitHub Linguist's strict threshold:
- **≥ 2,000 indexed files** matching `NOT is:fork path:*.kora` distributed across public repositories.
- Zero "Hello World" or tutorial code.
- Strict Clean Architecture with complete compilation and test passes.

## 2. Invariants for Catalog Examples
1. **100% English Code Policy**: All keywords, identifiers, types, function names, variables, decorators, comments, string literals, and log messages MUST be written strictly in idiomatic English. No non-English or Arabic characters are allowed anywhere within `.kora` code files.
2. **Strict Clean Architecture Layout**:
   - `models/`: Pure DTOs and value types (`model`).
   - `entities/`: Persistent database entities (`@table` or domain state).
   - `interfaces/`: Contracts for repositories and external services (`interface`).
   - `services/`: Domain business logic with Dependency Injection (`init(repo: IRepo)`).
   - `controllers/` & `routes/`: Presentation layer, HTTP routing, and status codes.
   - `main.kora`: Encapsulated within `Application` class with `bootstrap() -> Unit`.
   - `tests/`: Automated unit and peak load test suites.
3. **Compilability & Verification**:
   - `kora check <entrypoint>` must pass with 0 errors.
   - `kora test tests/<name>_unit.test.kora` must pass with 0 failures.
   - `kora test tests/<name>_load.test.kora` must pass with 0 failures and include mandatory teardown.
4. **No Placeholders**: Every file must contain complete, production-grade business logic without empty stubs or dummy code.
