# 📖 Zero-Config Swagger & OpenAPI 3.1.0 in Kora

One of Kora's most powerful architectural features is **Zero-Config Automatic API Documentation**.

> 💡 **Core Philosophy**: You never need to write manual Swagger annotations, decorator spaghetti, or YAML schemas. Kora's compiler reads your existing `model`, `@entity`, DTOs, controllers, and routes to synthesize an **OpenAPI 3.1.0** specification and an interactive **Swagger UI** automatically!

---

## ⚡ How It Works

During static analysis, the Kora compiler automatically inspects:
1. **Routes (`get`, `post`, `put`, `delete`, `patch`)**:
   - Path parameters (`:id` or `{id}`) are extracted and typed.
   - Operations are organized and tagged by domain (e.g. `/api/users` -> `Users`).
2. **Controller Methods & Handlers**:
   - Method signatures provide input parameter types and return schemas.
   - Calling `ctrl.create(dto)` correlates the `POST` route to `CreateUserDto` as the request body.
   - Return types like `List<UserResponseDto>` or `UserResponseDto` populate the 200/201 response schemas.
3. **Data Models (`model`) & Entities (`@entity`)**:
   - Fields, primitive types, and decorators (`@email`, `@required`, `@min`, `@default`, `@table`) are converted to standard JSON Schema specifications under `components.schemas`.
4. **Enums (`enum`)**:
   - Synthesized into type-safe string enumerations in the schema.

---

## 🚀 Generating Swagger Documentation

### Option 1: Automatic on `kora run` & `kora build`
Whenever you build or run your project:
```bash
kora run src/main.kora
```
Kora automatically outputs:
- `.kora-build/openapi.json`: OpenAPI 3.1.0 specification.
- `.kora-build/swagger-ui.html`: Standalone interactive Swagger UI.

### Option 2: Explicit CLI Command
You can generate the specification at any time:

```bash
# Generates openapi.json and swagger-ui.html in the current directory:
kora swagger

# Specify custom entrypoint or output destination:
kora swagger src/main.kora --out dist/openapi.json
```

---

## 🌐 Interactive Swagger UI

Open the generated `swagger-ui.html` directly in your browser:

```bash
# On Linux:
xdg-open swagger-ui.html

# On macOS:
open swagger-ui.html
```

You get a modern, interactive dashboard:
- Browse all endpoints grouped by domain feature tags (`Users`, `Auth`, `Orders`).
- Inspect exact request and response JSON schemas.
- Test endpoints interactively with "Try it out".
- Zero external runtime server dependencies required!

