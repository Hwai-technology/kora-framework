# Workflow: Linguist Enterprise Example Scaffolding & Verification

This workflow details how to scaffold, implement, and verify any enterprise example from the 2,000 Production Examples Matrix.

---

## Stage 1: Directory Setup
Scaffold the standard Clean Architecture directory tree under `examples/XX_<project_slug>/`:
```bash
mkdir -p examples/XX_<project_slug>/{models,entities,interfaces,services,controllers,routes,tests}
```

## Stage 2: Clean Architecture Implementation (Strict English Only)
1. **Models & Entities**:
   - Create `models/*.dto.kora` for DTOs and request/response payloads.
   - Create `entities/*.entity.kora` for domain entities and database tables.
2. **Interfaces & Contracts**:
   - Define `interfaces/*.interface.kora` for data access and domain port contracts.
3. **Domain Business Logic**:
   - Implement `services/*.service.kora` implementing repository interfaces or injecting dependencies via `init(...)`.
4. **Presentation & Routing**:
   - Implement `controllers/*.controller.kora` with structured HTTP handlers.
   - Implement `routes/*.routes.kora` exposing `registerRoutes(app: App)`.
5. **Application Bootstrap**:
   - Implement `main.kora` with `class Application { fn bootstrap() -> Unit { ... } }`.
6. **Automated Testing Suite**:
   - Implement `tests/<slug>_unit.test.kora` with mock repositories and assertion tests.
   - Implement `tests/<slug>_load.test.kora` with stress iteration loops and `teardown()`.

## Stage 3: Automated CLI Verification Pipeline
Run the following verification sequence:
```bash
# 1. Syntax and type check
kora check examples/XX_<project_slug>/main.kora

# 2. Execute unit tests
kora test examples/XX_<project_slug>/tests/<slug>_unit.test.kora

# 3. Execute load tests
kora test examples/XX_<project_slug>/tests/<slug>_load.test.kora
```

## Stage 4: Documentation
Create `examples/XX_<project_slug>/README.md` containing:
- Project summary and domain context.
- Architecture tree and file structure.
- Execution and testing CLI commands.
