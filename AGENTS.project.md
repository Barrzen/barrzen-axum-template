# {{project-name}} - Agent Notes

## Purpose
- Service generated from Barrzen Axum Template.
- Uses Barrzen Axum Kit crates for config, infra, observability, and OpenAPI.

## Key files
- `src/main.rs`: app entrypoint; uses kit flow (config -> tracing -> infra -> router -> optional OpenAPI -> AppBuilder -> serve).
- `.env.example`: runtime defaults; copy to `.env` for local runs.
- `Cargo.toml`: kit dependencies and selected cargo features from generation.

## Selected kit features (compile-time)
- core openapi: {{#if feature_openapi}}enabled{{else}}disabled{{/if}}
- infra db: {{#if feature_db}}enabled{{else}}disabled{{/if}}
- infra cache moka: {{#if feature_cache_moka}}enabled{{else}}disabled{{/if}}
- infra cache redis: {{#if feature_cache_redis}}enabled{{else}}disabled{{/if}}
- infra meilisearch: {{#if feature_meilisearch}}enabled{{else}}disabled{{/if}}
- infra nats: {{#if feature_nats}}enabled{{else}}disabled{{/if}}
- obs otel: {{#if feature_otel}}enabled{{else}}disabled{{/if}}
- obs fast-log: {{#if feature_fast_log}}enabled{{else}}disabled{{/if}}
- openapi crate: {{#if feature_openapi}}enabled{{else}}disabled{{/if}}

## Runtime flow (main.rs)
1) `Config::from_env()`
2) `barrzen_axum_obs::init_tracing(&config)`
3) `Infra::init(&config)`
4) `BuildInfo::from_env_or_defaults()`
5) Build `Router<()>` with sample routes.
6) Mount OpenAPI when `FEATURE_OPENAPI=true` (only if compiled with OpenAPI support).
7) `AppBuilder::new(...).with_ready_checker(infra).merge_stateless(app_routes).serve()`
8) `barrzen_axum_obs::shutdown()`

## Config notes
- `.env.example` documents `FEATURE_*` toggles and runtime settings.
- The kit currently reads `DATABASE_URL` for DB init; this repo uses `DB_URL` in `.env.example`.

## Useful commands
- `cp .env.example .env`
- `cargo run`
- `cargo test`
