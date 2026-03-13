# VUT FIT PIS 2026

A microservices-based information system for university project registration.

This repository contains:

- a **React frontend**
- a **Rust API router** exposing REST endpoints to the frontend
- a **Rust authentication service**
- a **Rust notification service**

TODO

- subject service
- project service

Services communicate internally with **gRPC**, while the frontend communicates with the backend through **REST API**.

---

## Current status

This is the **current working repository structure**, not the original design only.

Implemented or partially implemented:

- frontend in **React**
- API router in **Rust** using **Axum**
- Swagger / OpenAPI using **utoipa**
- JWT-based authentication middleware for private routes
- authentication service in **Rust**
- embedded database for auth service using **SurrealDB (embedded)**
- notification service in **Rust**
- embedded database for notification service using **Fjall**
- OpenTelemetry integration scaffold
- Dockerfiles and Docker Compose for local development

Still incomplete / template-level:

- subject service is currently a **Dart template**
- project service is currently a **.NET template**
- some frontend pages support **mock / hybrid / live** behavior depending on backend readiness
- some cross-service flows are scaffolded but may still need finishing

---

## Architecture overview

### High-level architecture

- **Frontend** → React + Material UI
- **Router** → REST API gateway in Rust/Axum
- **Internal communication** → gRPC using `tonic`
- **Auth service** → Rust + embedded SurrealDB
- **Notification service** → Rust + Fjall
- **Subject service** → Dart template
- **Project service** → .NET template
- **Observability** → OpenTelemetry + OTEL Collector

### Communication model

- Frontend → Router: **REST**
- Router → Services: **gRPC**
- Router handles:
  - public and private REST routes
  - JWT validation for protected endpoints
  - request routing to internal services
  - Swagger UI / OpenAPI docs

---

## Entity model

The system is based on these logical entities:

### User
- Id
- Firstname
- Lastname
- Email
- Password
- Role (`student`, `teacher`, `admin`)

### Subject
- Id
- Name
- Description
- Abbreviation

### Project
- Id
- Title
- Description
- Teacher
- Max Students
- Current Students
- Start Date
- End Date
- SubjectId

### Notification
- Id
- UserId
- Message
- Date

### Team
- Id
- ProjectId
- list of StudentId

---

## Services

### 1. Router
Rust service acting as API gateway.

Responsibilities:
- expose REST API to frontend
- validate JWT for protected routes
- forward requests to internal gRPC services
- expose Swagger UI
- request logging / tracing

Main stack:
- `axum`
- `tonic`
- `utoipa`
- `tower-http`

### 2. Auth Service
Rust service for authentication and authorization.

Responsibilities:
- register user
- login user
- validate JWT
- seed demo users for development
- manage user persistence

Current storage:
- **embedded SurrealDB**

Current JWT mode:
- **shared-secret JWT (HS256)**

### 3. Notification Service
Rust service for notifications.

Responsibilities:
- create notifications
- get user notifications
- mark notifications as read

Current storage:
- **Fjall**

### 4. Subject Service
Currently a **Dart template**.

Intended responsibilities:
- subject CRUD
- student subject registration

### 5. Project Service
Currently a **.NET template**.

Intended responsibilities:
- project CRUD
- team creation
- project registration

---

## Frontend use cases

### Student
- register account
- login
- view available projects
- view available subjects
- register for a subject
- register for a project
- create a team
- add members to a team
- receive notifications

### Teacher
- register account
- login
- create and manage projects
- receive notifications about registrations or updates

### Admin
- register account
- login
- manage users
- manage subjects
- manage projects

---

## Repository structure

```text
.
├── frontend/
├── proto/
├── services/
│   ├── auth-service-rust/
│   ├── notification-service-rust/
│   ├── router/
│   ├── subject-service-dart-template/
│   └── project-service-dotnet-template/
├── infra/
│   ├── docker-compose.yml
│   └── otel-collector-config.yaml
├── docker-compose.yml
└── README.md
```

### Important folders

#### `frontend/`
React frontend application.

#### `proto/`
Shared gRPC contract definitions.  
This is the main shared boundary between services.

#### `services/router/`
Rust REST gateway / router.

#### `services/auth-service-rust/`
Rust authentication service with embedded SurrealDB.

#### `services/notification-service-rust/`
Rust notification service with Fjall.

#### `services/subject-service-dart-template/`
Dart subject-service template.

#### `services/project-service-dotnet-template/`
.NET project-service template.

---

## Technology stack

### Frontend
- React
- Vite
- Material UI
- React Router

### Backend
- Rust
- Axum
- Tonic
- Utoipa / Swagger UI
- Tokio

### Storage
- Embedded SurrealDB for auth
- Fjall for notifications

### Observability
- OpenTelemetry
- OTEL Collector

### Containers
- Docker
- Docker Compose

---

## API and documentation

The router exposes Swagger UI for REST API exploration.

Typical endpoints:

- Swagger UI: `/swagger-ui`
- OpenAPI JSON: `/api-docs/openapi.json`

Depending on Docker setup, this is typically available at:

- `http://localhost:8080/swagger-ui`
- or via frontend proxy at `http://localhost:3000/swagger-ui`

---

## Authentication

The current implementation uses **JWT with shared secret (HS256)**.

### Flow
1. User logs in via router REST endpoint
2. Router forwards login request to auth-service over gRPC
3. Auth-service validates credentials
4. Auth-service returns JWT
5. Router returns token to frontend
6. Frontend includes token in `Authorization: Bearer <token>`

### Protected routes
Private routes are guarded by router middleware that validates the JWT before allowing access.

---

## Demo users

For development, demo users are seeded automatically on startup.

Available accounts:

- `student@example.com` / `student123`
- `teacher@example.com` / `teacher123`
- `admin@example.com` / `admin123`

These are intended for local development only.

---

## Running the project with Docker

From the repository root:

```bash
docker compose up --build
```

### Typical exposed ports

- Frontend: `http://localhost:3000`
- Router: `http://localhost:8080`
- OTEL Collector:
  - `4317`
  - `4318`

Exact port bindings depend on the current `docker-compose.yml`.

---

## Running frontend locally

From the `frontend/` folder:

```bash
npm install
npm run dev
```

If you run the frontend outside Docker, make sure its API base URL points to the router.

---

## Internal service contracts

Services should **not share internal code** with each other.

The intended service boundary is:

- each service owns its own codebase
- each service owns its own storage
- the only shared contract between services is the **proto file definitions**

This keeps services language-independent and supports polyglot development across:
- Rust
- Dart
- .NET

---

## Notes on current design

### Why only proto files should be shared
Sharing only `.proto` contracts is a good microservice boundary because:
- contracts stay language-neutral
- services remain loosely coupled
- each service can evolve independently
- Rust, Dart, and .NET can generate their own code from the same contracts

### What should not be shared
Avoid sharing:
- internal libraries
- database models
- common persistence code
- direct access to another service's database

That would tightly couple the services and reduce the benefits of microservice architecture.

---

## Logging and observability

The router includes request logging middleware.

OpenTelemetry integration is scaffolded so the services can emit traces to the OTEL collector.

This is intended to support:
- distributed tracing
- request visibility
- monitoring during development

---

## Current limitations

This repository is still in active development.

Known limitations:
- subject service is template-level
- project service is template-level
- some router-to-service flows may still need finishing
- frontend may fall back to mock or hybrid behavior in incomplete areas
- not all business rules are fully enforced yet

