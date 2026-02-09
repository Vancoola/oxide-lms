# Contributing to Oxide LMS 🦀

First off, thank you for considering contributing to Oxide! It’s people like you who will help make this the most flexible, high-performance LMS on the planet.

As a contributor, please follow these guidelines to ensure a smooth process for everyone.

---

## 🧭 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Style Guide](#style-guide)
- [Pull Request Process](#pull-request-process)

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