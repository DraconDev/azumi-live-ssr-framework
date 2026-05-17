use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use tower::ServiceExt;

fn response_to_string(response: axum::response::Response) -> String {
    let body = response.into_body();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let bytes = rt.block_on(async {
        axum::body::to_bytes(body, usize::MAX).await.unwrap()
    });
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_register_actions_adds_azumi_js_route() {
    let router = azumi::action::register_actions(Router::new());
    let response = router
        .oneshot(Request::builder().uri("/azumi.js").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_azumi_js_route_returns_javascript_content_type() {
    let router = azumi::action::register_actions(Router::new());
    let response = router
        .oneshot(Request::builder().uri("/azumi.js").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let content_type = response
        .headers()
        .get("content-type")
        .expect("content-type header should be present")
        .to_str()
        .unwrap();
    assert!(
        content_type.contains("javascript"),
        "Expected javascript content type, got: {}",
        content_type
    );
}

#[tokio::test]
async fn test_azumi_js_route_returns_runtime() {
    let router = azumi::action::register_actions(Router::new());
    let response = router
        .oneshot(Request::builder().uri("/azumi.js").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response_to_string(response.into());
    assert!(
        body.contains("azumi") || body.contains("Azumi") || !body.is_empty(),
        "azumi.js route should serve the client runtime, got {} bytes",
        body.len()
    );
    assert_eq!(body, azumi::AZUMI_JS, "Response body must match embedded AZUMI_JS");
}

#[tokio::test]
async fn test_register_actions_preserves_existing_routes() {
    let router = azumi::action::register_actions(
        Router::new().route("/health", axum::routing::get(|| async { "ok" })),
    );
    let response = router
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_register_actions_unknown_route_404() {
    let router = azumi::action::register_actions(Router::new());
    let response = router
        .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
