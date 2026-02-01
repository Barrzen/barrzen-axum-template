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
- Default `LOG_BACKEND` is `tracing`.
- Set `LOG_BACKEND=fast_log` to use the fast_log backend (requires the `fast-log`
  feature on `barrzen-axum-obs`); `LOG_FORMAT` is ignored in that mode and
  `FEATURE_OTEL=true` is not supported.

## Kit docs

- https://github.com/Barrzen/barrzen-axum-kit
- https://docs.rs/barrzen-axum-core
- https://docs.rs/barrzen-axum-infra
- https://docs.rs/barrzen-axum-obs
- https://docs.rs/barrzen-axum-openapi
