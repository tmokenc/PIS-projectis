# University Project Registration System - Polyglot Microservices Layout

This repository is a **generic, polyglot-friendly monorepo** for a university project registration information system.

## Goal

Allow each microservice to be implemented in a different language and runtime:

- `auth-service` -> Rust
- `notification-service` -> Rust
- `router` -> Rust
- `subject-service` -> Dart template
- `project-service` -> .NET template

The **only intentionally shared artifact** is the protobuf contract in `contracts/proto`.

## Core rule

- No shared Rust crate
- No shared domain library
- No shared repository layer
- No direct cross-service database access
- Only shared `.proto` files

## Why this layout

This keeps the system truly polyglot and prevents accidental coupling through shared helper libraries. Each service can build, release, and evolve independently.

## Folder structure

```text
.
├── contracts/
│   ├── proto/
│   └── buf.yaml
├── docs/
├── infra/
└── services/
    ├── auth-service-rust/
    ├── notification-service-rust/
    ├── router-rust/
    ├── subject-service-dart-template/
    └── project-service-dotnet-template/
```

## Important architectural note

If services share code, database tables, and release cycles, the system becomes closer to a distributed monolith than microservices.


## Frontend

A React + Vite + Material UI frontend is included in `frontend/`.

```bash
cd frontend
npm install
npm run dev
```

Use `.env.example` inside `frontend/` to configure the router base URL and API mode.


## Docker

Run the full stack from the repository root:

```bash
docker compose up --build
```

Frontend: `http://localhost:3000`

Router API: `http://localhost:8080`

Swagger UI: `http://localhost:3000/swagger-ui` or `http://localhost:8080/swagger-ui`


## Local storage in this stack

- `auth-service-rust` uses embedded SurrealDB (SurrealKV) with a Docker volume.
- `notification-service-rust` uses embedded Fjall with a Docker volume.
- No standalone database container is required for these two Rust services.
