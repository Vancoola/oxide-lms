# Contributing to Oxide LMS 🦀

First off, thank you for considering contributing to Oxide! It’s people like you who will help make this the most flexible, high-performance LMS on the planet.

As a contributor, please follow these guidelines to ensure a smooth process for everyone.

---

## 🧭 Table of Contents

- [📜 Code of Conduct](#-code-of-conduct)
- [💡 How Can I Contribute?](#-how-can-i-contribute)
- [🛠️ Development Setup](#-development-setup)
- [🏗️ Creating a New Domain Module](#-creating-a-new-domain-module)
- [🎨 Style Guide](#-style-guide)
- [🗄️ Database Migrations](#-database-migrations)
- [🧪 Testing](#-testing)
- [🔌 WASM Interface Changes](#-wasm-interface-changes)
- [💬 Commit Messages](#-commit-messages)
- [✅ Pull Request Checklist](#-pull-request-checklist)

---

## 📜 Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct (based on the Contributor Covenant). Be respectful, inclusive, and professional.

---

## 💡 How Can I Contribute?

### Reporting Bugs
- Use the **GitHub Issues** tab.
- Describe the bug, provide reproduction steps, and include your OS/Browser/Rust version.

### Suggesting Enhancements
- Open an issue titled `[Feature Request] ...`.
- Explain *why* this feature is needed and how it fits the WASM-plugin philosophy.

### Submitting a PR
1. Fork the repo.
2. Create a branch (`feat/something-cool` or `fix/annoying-bug`).
3. Push your changes and open a Pull Request.

---

## 🛠️ Development Setup

Since Oxide is a workspace-based Rust project, you'll need a few tools beyond just `cargo`.

### 1. Requirements
* **Rust Toolchain:** Latest stable (Edition 2024).
* **Trunk:** For building the Leptos frontend. `cargo install trunk`.
* **SQLx CLI:** For database migrations. `cargo install sqlx-cli`.
* **Docker:** To run the PostgreSQL instance.
* **Wasmtime/Wit-bindgen:** To work on the plugin engine.

### 2. Local Environment
```bash
# 1. Start the database
docker-compose up -d

# 2. Run migrations
sqlx database setup

# 3. Build shared types (if changed)
cargo build -p oxide-shared-types

# 4. Start the backend
cargo run -p oxide-api

# 5. Start the frontend (in a separate terminal)
cd client/oxide-web
trunk serve
```

### 🏗️ Creating a New Domain Module
When adding a new feature (e.g., `library`), follow the established pattern:
1. Define entities and traits in `oxide-domain/src/library/`.
2. Implement repository logic in `oxide-data/`.
3. Orchestrate logic in `oxide-business/`.
4. Expose endpoints in `oxide-api/`.
5. Update `oxide-shared-types` if the frontend needs access to new models.

---

## 🎨 Style Guide

### 🦀 Rust Standards
- **Formating:** Always run `cargo fmt --all` before committing.
- **Linting:** We don't accept PRs with Clippy warnings. Run:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
- **Type Safety:** Avoid `unwrap()`. Prefer `anyhow` or `thiserror` for graceful error handling.

### 📝 Documentation
- Public functions in oxide-domain and oxide-business must have doc-comments (`///`).
- API endpoints must be annotated with `#[utoipa::path]` for Swagger generation.

## 🗄️ Database Migrations
We use `sqlx-cli` for database schema management.
- **Creating a migration:**
```bash
sqlx migrate add -r <name_of_migration>
```
- **Running migrations:**
```bash
sqlx migrate run 
```
>[!IMPORTANT] 
>Always provide both up and down (reversible) migrations if possible.

---

## 🧪 Testing
Before submitting a PR, ensure all tests are passing.

- **Run all tests:** 
```bash
cargo test --workspace
```
- **Test a specific crate:**
```bash
cargo test -p oxide-wasm-provider
```
- **Documentation tests:**
```bash
cargo test --doc
```

---

## 🔌 WASM Interface Changes
If you modify `.wit` files in the domain:
1. Ensure `wit-bindgen` is updated.
2. Run `cargo build -p oxide-wasm-provider` to regenerate guest/host bindings.
3. Check if any existing plugins in `/examples` are broken by the change.

---

## 💬 Commit Messages
We follow [Conventional Commits](https://www.conventionalcommits.org/). This helps us automate our release process.
- `feat: ...` — for new features.
- `fix: ...` — for bug fixes.
- `docs: ...` — for documentation changes.
- `refactor: ...` — for code changes that neither fix a bug nor add a feature.

**Example:** `feat(domain): add course enrollment event`

---
## ✅ Pull Request Checklist

- [ ] Code is formatted with cargo fmt.
- [ ] cargo clippy passes without warnings.
- [ ] All tests pass locally.
- [ ] Documentation is updated (including Swagger/Utoipa annotations).
- [ ] For database changes, migrations are reversible (up and down).
- [ ] My branch is rebased on the latest main.

<div align="center"> Happy coding! 🦀 </div>