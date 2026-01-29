use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use barrzen_axum_core::{AppBuilder, BuildInfo, Config};
use tower::ServiceExt;

#[tokio::test]
async fn test_template_healthz() {
    // Setup config (mock env)
    let config: Config = serde_json::from_str(r#"{
        "app_name": "test-template",
        "app_env": "dev",
        "app_host": "127.0.0.1",
        "app_port": 0,
        "app_debug": true,
        "app_shutdown_grace_seconds": 1,
        "feature_startup_banner": false,
        "feature_db": false,
        "feature_cache": false,
        "feature_search": false,
        "feature_broker": false,
        "feature_openapi": false,
        "feature_request_log": false,
        "feature_tracing": false,
        "feature_otel": false,
        "feature_cors": false,
        "feature_session": false,
        "feature_response_envelope": true,
        "http_body_limit_bytes": 1024,
        "http_request_timeout_seconds": 1,
        "log_level": "info",
        "log_format": "pretty",
        "log_include_target": false,
        "log_include_fileline": false,
        "request_log_headers_denylist": "",
        "cache_backend": "none",
        "cache_ttl_seconds": 60,
        "cache_max_entries": 1000,
        "cache_redis_pool_size": 1,
        "cache_redis_connect_timeout_seconds": 1,
        "cors_allow_methods": "GET",
        "cors_allow_headers": "content-type",
        "cors_allow_credentials": false,
        "cors_max_age_seconds": 60,
        "banner_show_secrets": false,
        "banner_show_env_vars": false
    }"#).unwrap();

    let build = BuildInfo::new("test", "0.1.0", None, "1.0.0", None);
    let app = AppBuilder::new(config, build).build();

    let response = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
