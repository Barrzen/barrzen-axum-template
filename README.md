# {{project_name}}

Barrzen Axum application template.

## Quick start

```bash
cargo generate --git https://github.com/Barrzen/barrzen-axum-template.git --name {{project_name}}
cd {{project_name}}
cp .env.example .env
cargo run
```

## What you get

- Axum app wired with Barrzen kit crates
- Optional OpenAPI via feature flag
- Observability + infra stubs ready to extend

## Configuration

Edit `.env` and adjust features in `Cargo.toml` as needed.
