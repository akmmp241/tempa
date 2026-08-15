use axum::response::Html;
use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;

const OPENAPI_YML: &str = include_str!("../../../../openapi.yml");

const SCALAR_UI_HTML: &str = r#"
<!doctype html>
<html>
  <head>
    <title>LogiTrack API Reference</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      body { margin: 0; }
    </style>
  </head>
  <body>
    <script id="api-reference" data-url="/openapi.yml"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>
"#;

pub fn create_routes() -> Router {
    Router::new()
        .merge(system_routes())
}

fn system_routes() -> Router {
    Router::new()
        .route("/openapi.yml", get(|| async {
            ([("content-type", "text/yml")], OPENAPI_YML)
        }))
        .route("/docs", get(|| async {
            Html(SCALAR_UI_HTML)
        }))
        .route("/health", get(|| async {
            Json(json!({ "status": "ok" }))
        }))
}