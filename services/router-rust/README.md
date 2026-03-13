# Router / API Gateway (Rust)

REST facade for the frontend.

Responsibilities:
- expose REST endpoints with Axum
- call internal gRPC services with Tonic
- validate JWT on private routes through `AuthService.ValidateToken`
- apply coarse authentication at the router level
- export OpenTelemetry traces
- expose OpenAPI / Swagger UI documentation

Routes:
- Swagger UI: `/swagger-ui`
- OpenAPI JSON: `/api-docs/openapi.json`
- Health: `/health`
