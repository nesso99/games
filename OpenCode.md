# OpenCode.md

## Build, Lint, and Test

- **Build all:**  
  `cargo build`

- **Run a specific game:**  
  `cargo run -p <package_name>`

- **Test all:**  
  `cargo test`

- **Run a single test (by name):**  
  `cargo test <test_name>`

- **Lint (format, check, clippy):**  
  `cargo fmt -- --check`  
  `cargo clippy -- -D warnings`

## Code Style Guidelines

- **Imports:** Group and order: std, external crates, internal modules. No unused imports.
- **Formatting:** Always run `cargo fmt` before committing.
- **Types & Naming:**
  - Use `CamelCase` for types and structs.
  - Use `snake_case` for variables/functions.
  - Use `SCREAMING_SNAKE_CASE` for constants.
- **Error Handling:** Prefer `Result<T, E>`, use `?` operator for propagation. Avoid panics except in tests or unreachable states.
- **Documentation:** Public structs/functions should have `///` doc comments.
- **Other:**
  - Avoid large functions (split for clarity).
  - Prefer explicit lifetimes/types for clarity and codegen.
