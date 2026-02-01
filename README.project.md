# {{project-name}}

Rust Axum service scaffolded from the **Barrzen Axum Template**.

## What you get

- Axum app wired with Barrzen Axum Kit crates
- Config + feature flags via the kit
- Observability setup (tracing + optional OpenTelemetry)
- Optional OpenAPI docs

## Quick start

```bash
cp .env.example .env
cargo run
```

## Configuration

- Edit `.env` for runtime settings
- Toggle features in `Cargo.toml` as needed
- Logs default to `LOG_FORMAT=compact` (single‑line, no color). Set `pretty` or `json` if desired.

## Kit docs

- https://github.com/Barrzen/barrzen-axum-kit
- https://docs.rs/barrzen-axum-core
- https://docs.rs/barrzen-axum-infra
- https://docs.rs/barrzen-axum-obs
- https://docs.rs/barrzen-axum-openapi
