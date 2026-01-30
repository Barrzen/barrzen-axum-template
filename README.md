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
cp .env.example .env
cargo run
```

## Kit docs

- https://github.com/Barrzen/barrzen-axum-kit
- https://docs.rs/barrzen-axum-core
- https://docs.rs/barrzen-axum-infra
- https://docs.rs/barrzen-axum-obs
- https://docs.rs/barrzen-axum-openapi
