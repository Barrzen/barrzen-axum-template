# Barrzen Axum Template

A production‑ready Axum starter built on the Barrzen Axum Kit. Use this template to bootstrap
new services fast with sane defaults for config, observability, infra wiring, and OpenAPI.

## Benefits

- Clean Axum setup with ready‑to‑extend routing and handlers
- Config + feature flags already wired through the kit
- Observability (tracing + OTEL ready) and infra placeholders
- Optional OpenAPI docs for quick API exploration

## Prerequisites

- Rust toolchain (stable)
- `cargo-generate` installed

Install `cargo-generate`:

```bash
cargo install cargo-generate
```

## Quick start

```bash
cargo generate --git https://github.com/Barrzen/barrzen-axum-template.git --name my-service
cd my-service
cargo run
```

During generation you will be prompted to select kit features (OpenAPI, DB, cache, OTEL, etc.).

## Logging

The template defaults to `LOG_FORMAT=compact` for single‑line logs without color.
Set `LOG_FORMAT=pretty` or `LOG_FORMAT=json` if you prefer those formats.
Default `LOG_BACKEND` is `tracing`.
Set `LOG_BACKEND=fast_log` to use the fast_log backend (requires the `fast-log`
feature on `barrzen-axum-obs`); `LOG_FORMAT` is ignored in that mode and
`FEATURE_OTEL=true` is not supported.

## Database

If you generated the project with DB support, enable it at runtime by setting
`FEATURE_DB=true` and `DATABASE_URL=...` in `.env`.

## Kit docs

- https://github.com/Barrzen/barrzen-axum-kit
- https://docs.rs/barrzen-axum-core
- https://docs.rs/barrzen-axum-infra
- https://docs.rs/barrzen-axum-obs
- https://docs.rs/barrzen-axum-openapi
