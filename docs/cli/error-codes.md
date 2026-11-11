# Kora Compiler Diagnostics & Error Codes Catalog

This catalog documents the standard diagnostic error codes emitted by the Kora compiler and CLI toolchain, along with causes and solutions.

---

## 📂 File System & Environment Errors (`E1000 - E1999`)

### `error[E1001]: file not found`
- **Cause**: The specified source file does not exist at the given path relative to your current working directory.
- **Diagnostic output**:
  ```text
  error[E1001]: file not found: `examples/hello.kora`
    --> Current working directory: /home/ahmet

    💡 The example file was found in the Kora repository at:
       /home/ahmet/Desktop/Projects/Kora/examples/hello.kora
    You can run it directly with its full path:
       kora /home/ahmet/Desktop/Projects/Kora/examples/hello.kora
  ```
- **Remedy**: Check the path, run from the project root, or use the copy-paste command suggested by the CLI.

---

### `error[E1002]: path is a directory`
- **Cause**: The argument provided is a directory instead of a `.kora` source file.
- **Remedy**: Specify the `.kora` file explicitly or use `kora run` to let Kora auto-resolve the default entrypoint.

---

### `error[E1003]: failed to read file`
- **Cause**: An IO error or permission denied occurred while reading the file.
- **Remedy**: Verify read permissions on the file (`chmod +r <file>`).

---

### `error[E1004]: module or import not found`
- **Cause**: An `import { ... } from "./path"` statement failed to resolve to a matching `.kora` or `.kr` file.
- **Remedy**: Verify relative import paths. Ensure the module exists in the specified feature or shared directory.

---

### `error[E1005]: Rust compiler (cargo) not found`
- **Cause**: The `cargo` executable was not found on your system `PATH`. Kora lowers code to safe Rust for native compilation.
- **Remedy**: Install the official Rust toolchain via:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

---

## 🔠 Type & Interface Errors (`E2000 - E2999`)

### `error[E2001]: type mismatch`
- **Cause**: An expression's evaluated type does not match the expected type annotation or assignment.
- **Remedy**: Align types or perform explicit conversions.

---

### `error[E2002]: missing interface method`
- **Cause**: A `class` declared that it implements an `interface`, but failed to provide an implementation for one of the interface's required methods.
- **Remedy**: Implement the missing method signature in the class body.

---

### `error[E2003]: undefined is forbidden`
- **Cause**: Use of `undefined` keyword or undefined state. Kora strictly adheres to a **Single-Null** philosophy (`null` / `Option<T>` only).
- **Remedy**: Replace `undefined` with `null`, `None`, or an `Option<T>`.

---

## 🔄 Dependency Injection & Ownership Errors (`E3000 - E3999`)

### `error[E3001]: circular dependency detected`
- **Cause**: Two or more classes depend on each other cyclically in their constructors (`init`).
- **Remedy**: Break the circular dependency by refactoring shared logic into a separate repository or utility service.

---

## 🏛️ Clean Architecture Violations (`E5000 - E5999`)

### `error[E5001]: executable logic in enums directory`
- **Cause**: A file located inside `src/enums/` attempted to define a class, function, or HTTP route.
- **Remedy**: `src/enums/` must contain only pure `pub enum` definitions.

---

### `error[E5002]: executable logic in models or interfaces`
- **Cause**: A file inside `src/models/` or `src/interfaces/` attempted to execute statements or start a server.
- **Remedy**: Keep models and interfaces declarative. Move behavioral code into services or controllers.

---

### `error[E5003]: inline models in services or controllers`
- **Cause**: A service or controller declared an inline `@entity` or `model` instead of importing it from `models/`.
- **Remedy**: Move the model into a dedicated `models/` file and import it explicitly.

---

### `error[E5004]: inline definitions in main entrypoint`
- **Cause**: `src/main.kora` declared inline domain models or enums.
- **Remedy**: Import domain types from their respective features in `src/features/<name>/`.

