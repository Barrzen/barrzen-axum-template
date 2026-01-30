use axum::{routing::get, Router};
use barrzen_axum_core::{
    ApiResponse, AppBuilder, BuildInfo, Config,
};
use barrzen_axum_infra::Infra;
use barrzen_axum_openapi::mount as mount_openapi;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load config
    let config = Config::from_env()?;

    // 2. Init observability
    barrzen_axum_obs::init_tracing(&config)?;

    // 3. Init infrastructure
    let infra = Infra::init(&config).await?;

    // 4. Load build info
    let build_info = BuildInfo::from_env_or_defaults();

    // 5. Build router
    let app_routes = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler));

    // 6. OpenAPI (optional)
    #[derive(OpenApi)]
    #[openapi(paths(root_handler, hello_handler))]
    struct ApiDoc;
    
    // Mount OpenAPI if enabled
    let app_routes = if config.features.feature_openapi {
        mount_openapi(app_routes, ApiDoc::openapi())
    } else {
        app_routes
    };

    // 7. Run app
    AppBuilder::new(config, build_info)
        .with_ready_checker(infra)
        .merge_stateless(app_routes)
        .serve()
        .await?;

    // 8. Cleanup
    barrzen_axum_obs::shutdown();

    Ok(())
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome message", body = ApiResponse<String>)
    )
)]
async fn root_handler() -> ApiResponse<String> {
    ApiResponse::ok(format!("Welcome to {}!", "{{project_name}}"), "Success")
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello message", body = ApiResponse<String>)
    )
)]
async fn hello_handler() -> ApiResponse<String> {
    ApiResponse::ok("Hello World!".to_string(), "Greeting")
}
