<div align="center">

  <h1>Oxide LMS🦀</h1>
  <p><b>A high-performance, modular Learning Management System powered by Rust and WebAssembly.</b></p>

  <p>
    <img src="https://img.shields.io/badge/Rust-2024-orange?logo=rust" alt="Rust Version">
    <img src="https://img.shields.io/badge/License-Apache_2.0-green" alt="License">
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen" alt="PRs Welcome">
    <img src="https://img.shields.io/badge/Plugins-Wasmtime-6C4FBB?logo=webassembly" alt="Powered by Wasm">
    <img src="https://img.shields.io/badge/Backend-Axum-lightgrey" alt="Backend">
    <img src="https://img.shields.io/badge/Frontend-Leptos-blue" alt="Frontend">
  </p>

  <p>
    <a href="#-features">Features</a> •
    <a href="#-architecture">Architecture</a> •
    <a href="#-getting-started">Getting Started</a> •
    <a href="#-plugin-system">Plugin System</a>
  </p>
</div>

---

## 🌟 Overview

**Oxide** is a modern alternative to legacy LMS platforms like Moodle. Built from the ground up in **Rust**, it prioritizes type safety, extreme performance, and a revolutionary plugin system based on **WebAssembly (WASM)**.

Unlike traditional systems where plugins can crash the entire server, Oxide runs extensions in an isolated, sandboxed environment using `Wasmtime`.

> [!IMPORTANT]
> This project is built using the **Rust 2024 Edition**. Ensure your toolchain is up to date (`rustup update`).

---

## ✨ Features

* **🚀 Blazing Fast:** Core engine built with `Axum` and `SQLx` for sub-millisecond response times.
* **🧩 WASM Plugins:** Extend the LMS logic (grading, custom content types) using any language that compiles to WASM.
* **💻 Modern UI:** Reactive, type-safe frontend built with `Leptos` (No JavaScript fatigue).
* **🛡️ Secure by Design:** Argon2 password hashing, AES-GCM encryption, and JWT-based authentication.
* **📖 Auto-documented API:** Full OpenAPI/Swagger support integrated via `utoipa`.

---

## 🏗️ Architecture

Oxide uses a **Hexagonal/Clean Architecture** within a Cargo Workspace to keep the business logic separated from the infrastructure.

```mermaid
graph TD
    subgraph Client
        A[oxide-web] --> B[oxide-ui]
        A --> C[oxide-shared-types]
    end

    subgraph Server
        D[oxide-api] --> E[oxide-business]
        E --> F[oxide-data]
        E --> G[oxide-wasm-provider]
        G --> H[WASM Plugins]
    end

    F --> I[(PostgreSQL)]