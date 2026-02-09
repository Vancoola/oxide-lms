<div align="center">

  <h1>Oxide LMS🦀</h1>
  <p><b>A high-performance, modular Learning Management System powered by Rust and WebAssembly.</b></p>

  <p>
    <img src="https://img.shields.io/badge/Rust-2024-orange?logo=rust" alt="Rust Version">
    <img src="https://img.shields.io/badge/License-Apache_2.0-green" alt="License">
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen" alt="PRs Welcome">
    <a href="https://crates.io/crates/leptos">
      <img src="https://img.shields.io/crates/v/leptos?label=Leptos&color=blue&style=flat-square" alt="Leptos Version">
    </a>
    <a href="https://crates.io/crates/axum">
      <img src="https://img.shields.io/crates/v/axum?label=Axum&color=orange&style=flat-square" alt="Axum Version">
    </a>
    <a href="https://crates.io/crates/wasmtime">
      <img src="https://img.shields.io/crates/v/wasmtime?logo=webassembly&label=Wasmtime&color=black&style=flat-square" alt="Wasmtime Version">
    </a>
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
    subgraph Clients [Frontend Applications]
        direction TB
        admin[oxide-admin]
        dean[oxide-dean]
        web[oxide-web]
    end

    subgraph ClientShared [Client Shared]
        common[oxide-web-common]
        ui[oxide-ui]
        i18n[oxide-i18n]
    end

    admin --> common
    dean --> common
    web --> common
    
    common --> ui
    common --> i18n

    shared_types[oxide-shared-types]
    web -.-> shared_types
    api -.-> shared_types

    subgraph Server [Backend Engine]
        api[oxide-api]
        biz[oxide-business]
        data[oxide-data]
        wasm[oxide-wasm-provider]
        
        api --> biz
        biz --> data
        biz --> wasm
        wasm --> plugins[{WASM Plugins}]
    end

    data --> db[(PostgreSQL)]