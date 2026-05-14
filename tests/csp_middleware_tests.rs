//! CSP Middleware Integration Tests
//!
//! Tests that verify the `csp_nonce_layer` middleware:
//!   1. Generates a fresh nonce per request
//!   2. Injects the nonce into request extensions (extractable as `CspNonce`)
//!   3. Sets the `Content-Security-Policy` response header
//!   4. Custom policy builder works via `csp_nonce_layer_with`

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware,
    routing::get,
};
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use tower::ServiceExt;

use azumi::csp::{CspNonce, ContentSecurityPolicy, csp_nonce_layer, csp_nonce_layer_with};

async fn handler_extract_nonce(nonce: CspNonce) -> impl IntoResponse {
    nonce.as_str().to_string()
}

async fn handler_ok() -> &'static str {
    "ok"
}

#[tokio::test]
async fn test_csp_nonce_layer_sets_header() {
    let app = Router::new()
        .route("/", get(handler_ok))
        .layer(middleware::from_fn(csp_nonce_layer()));

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let csp = response
        .headers()
        .get(header::CONTENT_SECURITY_POLICY)
        .expect("CSP header should be set")
        .to_str()
        .unwrap();

    assert!(csp.contains("default-src 'self'"), "should contain default-src");
    assert!(csp.contains("'nonce-"), "should contain nonce directive");
    assert!(!csp.contains("'unsafe-inline'"), "nonce policy should not have unsafe-inline");
}

#[tokio::test]
async fn test_csp_nonce_layer_nonce_in_extensions() {
    let app = Router::new()
        .route("/", get(handler_extract_nonce))
        .layer(middleware::from_fn(csp_nonce_layer()));

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let nonce_value = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(nonce_value.len(), 24, "nonce should be 24 chars (16 bytes base64)");
}

#[tokio::test]
async fn test_csp_nonce_layer_unique_per_request() {
    let app = Router::new()
        .route("/", get(handler_extract_nonce))
        .layer(middleware::from_fn(csp_nonce_layer()));

    let nonce1 = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap()
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let nonce2 = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap()
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    assert_ne!(nonce1, nonce2, "each request should get a unique nonce");
}

#[tokio::test]
async fn test_csp_nonce_layer_with_custom_policy() {
    let app = Router::new()
        .route("/", get(handler_ok))
        .layer(middleware::from_fn(csp_nonce_layer_with(|nonce| {
            ContentSecurityPolicy::azumi_nonce_defaults(nonce)
                .connect_src("'self' ws://localhost:3000")
        })));

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let csp = response
        .headers()
        .get(header::CONTENT_SECURITY_POLICY)
        .unwrap()
        .to_str()
        .unwrap();

    assert!(csp.contains("connect-src 'self' ws://localhost:3000"), "custom connect-src should appear");
    assert!(csp.contains("'nonce-"), "should still contain nonce");
}

#[tokio::test]
async fn test_csp_nonce_defaults_format() {
    let nonce = CspNonce::generate();
    let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();

    assert!(csp.contains(&format!("script-src 'self' 'nonce-{}'", nonce.as_str())));
    assert!(csp.contains(&format!("style-src 'self' 'nonce-{}'", nonce.as_str())));
    assert!(csp.contains("default-src 'self'"));
    assert!(csp.contains("frame-ancestors 'none'"));
}
