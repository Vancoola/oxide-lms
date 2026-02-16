<div align="center">
  <img src="/logo.png" width="150" alt="Oxide Logo">
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

## 🧩 Plugin System

Oxide provides a unique "Sandboxed Extensibility" model.
- **Language Agnostic:** Write plugins in Rust, Zig, C++, or Go.
- **Hot Reloading:** Inject new logic without restarting the server.
- **Safety:** Plugins cannot access the host file system or network unless explicitly granted permission.

> [!TIP]
> See `/examples/wasm-plugin-rust` for a template on how to create a custom grading hook.

### 🔌 Plugin Contract (WIT)
Oxide uses the **WASM Component Model**. Interfaces are defined using `WIT` (WebAssembly Interface Type) files, ensuring language-independent type safety.

```wit
interface grading-provider {
    record score {
        points: f32,
        max-points: f32
    }
    calculate: func(attempt: string) -> score
}
```

---

## 🚀 Scalability & Future-Proofing

Oxide is built as a **Modular Monolith**. This means:
* **Strong Boundaries:** Modules in `oxide-domain` do not share state.
* **Event-Driven:** Cross-module communication happens strictly through the `DomainEvent` bus.
* **Cloud Native:** The design is pre-optimized for splitting into **Microservices**. If the `course` module grows too large, it can be moved to a separate container with its own database, communicating via the existing Event System (e.g., over NATS or RabbitMQ).

---

## 📂 Project Structure

<details>
<summary>Explore the Cargo Workspace</summary>

```text
.
├── client/                 # Frontend applications & web resources
│   ├── oxide-web/          # Main Student Portal (Leptos CSR/SPA)
│   ├── oxide-admin/        # System Administration dashboard for IT-staff
│   ├── oxide-dean/         # Educational management & Dean's office UI
│   ├── oxide-ui/           # Shared Design System (Components, Tailwind, Styles)
│   ├── oxide-web-common/   # Common frontend logic, API-fetchers, and hooks
│   └── oxide-i18n/         # Localization engine and translation resources
├── server/                 # Backend services & business logic
│   ├── oxide-api/          # Composition Root: Axum routes, Swagger UI, and App state
│   ├── oxide-business/     # Application layer: Use-case orchestration and Handlers
│   ├── oxide-domain/       # The Core: Pure Entities, Value Objects, and Event definitions
│   ├── oxide-infrastructure/ # Technical layer: Crypto, JWT, Mailers, and External Gateways
│   ├── oxide-data/         # Persistence layer: SQLx Repository implementations
│   ├── oxide-wasm-provider/# Plugin System: Wasmtime host and runtime isolation logic
│   └── oxide-macros/       # Procedural macros for boilerplate reduction (DTOs, etc.)
├── oxide-shared-types/     # Unified DTOs and types for Client-Server synchronization
├── migrations/             # Database schema evolution (SQLx/PostgreSQL)
├── compose.yaml            # Local development environment (DB, Redis, etc.)
└── Cargo.toml              # Workspace manifest and centralized dependency management 
```
</details>

---

## 🏗️ Architecture

Oxide uses a **Hexagonal/Clean Architecture** within a Cargo Workspace to keep the business logic separated from the infrastructure.

```mermaid
graph LR
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

    subgraph External [External storage]
        redis[(Redis)]
        db[(PostgreSQL)]
        ES[(Elasticsearch)]
    end

    subgraph Server [Backend Engine]
        api[oxide-api]
        subgraph IL [Infrastructure Layer]
            inf[oxide-infrastructure]
            data[oxide-data]
        end
        biz[oxide-business]
        wasm[oxide-wasm-provider]
        
        
        api --> biz
        biz --> inf
        biz --> data
        biz --> wasm
        wasm --x plugins{{WASM Plugins}}

        dom[oxide-domain]
        macros[oxide-macros]
        
        dom ==> macros
        
        data --> dom
        inf --> dom
        biz --> dom
        wasm --> dom
        
    end
    
    
    
    data --> External
```

### 🧩 Domain Module Anatomy
Each domain module (e.g., `course`, `user`) follows a strict internal structure to ensure separation of concerns:

```mermaid
    graph LR
    subgraph Module [Domain Module Structure]
        model[mod.rs: Core Entity]
        vo[object.rs: Value Objects]
        events[event.rs: Domain Events]
        repo[repository.rs: Repository Traits]

        subgraph Plugins [Plugin System]
            registry[mod.rs: Registry Entity]
            hook[guard.rs / hook.rs / middleware.rs: Plugin Traits]
        end
    end

    model --> vo
    model --> events
    repo -.-> model
    hook --> model
```
### Centralized Event System 
Oxide uses a unified DomainEvent enum that aggregates events from all sub-modules. This allows for a clean, decoupled way to handle cross-module side effects (e.g., sending an email when a user is registered).
```mermaid
    graph TD
        subgraph Domain [oxide-domain]
            direction TB
            subgraph Modules [Domain Modules]
                U[User Module]
                C[Course Module]
                S[Student Module]
            end
    
            GlobalEvent{{event.rs: DomainEvent Enum}}
    
            U -- "UserEvent" --> GlobalEvent
            C -- "CourseEvent" --> GlobalEvent
            S -- "StudentEvent" --> GlobalEvent
        end
    
        GlobalEvent ==> Bus[Internal Event Bus / Subscriber]
```
### Business Module Anatomy
Each business module (e.g., `course`, `user`) follows a strict internal structure to ensure separation of concerns:
```mermaid
    graph LR
    subgraph Module [Business Module Structure]
        model[mod.rs: Core Entity]
        H[handler.rs: Event Reactions]
        S[service.rs: Use Cases / Logic]
        P[ports.rs: External Interfaces]
    end
    model --> H
    model --> S
    model --> P
```

### Cross-Layer Orchestration
The Business layer acts as the central mediator between pure Domain logic and technical Infrastructure, enforcing the `Dependency Inversion` principle.
```mermaid
    graph LR
        subgraph BusinessModule [Business Module]
            S[service.rs: Use Cases / Logic]
            P[ports.rs: External Interfaces]
            H[handler.rs: Event Reactions]
        end
    
        subgraph Domain [Domain Layer]
            E[mod.rs: Entities / VO]
        end
    
        subgraph Infra [Infrastructure Layer]
            Impl[Gateways / Repos Impl]
        end
        
        Impl -- Implements --> P
        H --> S
        S --> P
        S --> E
```

---
## 🛠️ Getting Started

### Prerequisites
* **Rust:** v1.82+ (Edition 2024)
* **Trunk:** `cargo install trunk` (for frontend)
* **SQLx CLI:** `cargo install sqlx-cli`
* **Docker:** To run PostgreSQL and Redis

### Quick Run
1. **Infrastructure:** `docker-compose up -d`
2. **Database:** `sqlx database setup`
3. **Server:** `cargo run -p oxide-api`
4. **Client:** `cd client/oxide-web && trunk serve`


---

## 🗺️ Roadmap

- [ ] **Phase 1:** Core LMS (Users, Courses, Lessons) - *In Progress*
- [ ] **Phase 2:** WASM Plugin Registry & Hot-swapping
- [ ] **Phase 3:** Mobile Client (Tauri / Leptos Native)
- [ ] **Phase 4:** Advanced Analytics & AI-assisted grading

---

## 🤝 Contributing

We are looking for help with:
- Implementing core domain logic.
- Improving the WASM host environment.
- Translating the UI (check `client/oxide-i18n`).

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

<div align="center">
  Built with ❤️ and 🦀 by Suzdaltsev Denis
</div>