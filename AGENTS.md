# Barrzen Axum Template - Agent Notes

## Purpose
- `cargo-generate` template that scaffolds a service using Barrzen Axum Kit crates.
- Provides a minimal app entrypoint, sample routes, and env defaults.

## Key files
- `src/main.rs`: app entrypoint; loads config, init tracing, init infra, builds router, mounts OpenAPI, serves.
- `cargo-generate.toml`: template metadata and generation prompts.
- `hooks/post-script.rhai`: renames `README.project.md` and `AGENTS.project.md`, removes hooks, prints next steps.
- `AGENTS.project.md`: copied into generated project and renamed to `AGENTS.md`.
- `scripts/bump-kit.py`: updates kit versions using crates.io.
- `tests/integration.rs`: simple healthz test using `AppBuilder`.

## Runtime flow (main.rs)
1) `Config::from_env()`
2) `barrzen_axum_obs::init_tracing(&config)`
3) `Infra::init(&config)`
4) `BuildInfo::from_env_or_defaults()`
5) Build `Router<()>` with sample routes.
6) Optionally mount OpenAPI if `FEATURE_OPENAPI=true`.
7) `AppBuilder::new(...).with_ready_checker(infra).merge_stateless(app_routes).serve()`
8) `barrzen_axum_obs::shutdown()`

## Config notes
- `.env.example` documents runtime env vars and `FEATURE_*` toggles.
- The kit currently reads `DATABASE_URL` for DB init; `.env.example` uses `DB_URL` and other DB_* fields.

## Workflow note
- After each commit and push, switch to a fresh branch based on `origin/main`.

## Useful commands
- Generate: `cargo generate --git https://github.com/Barrzen/barrzen-axum-template.git --name my-service`
- Run locally: `cp .env.example .env` then `cargo run`
- Test: `cargo test`
- Update kit versions: `python scripts/bump-kit.py`
