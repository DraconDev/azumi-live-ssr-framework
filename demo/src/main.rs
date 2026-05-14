mod actions;
pub mod components;
mod examples;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

// Include the generated assets manifest
pub mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets_manifest.rs"));
}

#[tokio::main]
async fn main() {
    // ⚡ Start the self-hosting hot reload system
    // Call this first! It manages sub-second patching and auto-restarts.
    azumi::devtools::auto_reload();

    // Initialize Global SEO
    azumi::seo::init_seo(azumi::seo::SeoConfig {
        title: "Azumi Demo".to_string(),
        open_graph: Some(azumi::seo::OpenGraph {
            site_name: Some("Azumi Framework".to_string()),
            image: Some("https://azumi.dev/og-default.jpg".to_string()),
            ..Default::default()
        }),
        twitter: Some(azumi::seo::TwitterCard {
            site: Some("@AzumiFramework".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    });

    // 🚀 Azumi Demo Server - Complete Learning Path
    let app = Router::new()
        // 🏠 Homepage - Learning Portal
        .route("/", get(examples::lessons::pages::homepage::homepage_handler))


        // 📚 Interactive Lessons (0-20)
        .route("/lesson-0", get(examples::lessons::pages::lesson0::handler))
        .route("/lesson-2", get(examples::lessons::pages::lesson2::lesson2_handler))
        .route("/lesson-3", get(examples::lessons::pages::lesson3::lesson3_handler))
        .route("/lesson-4", get(examples::lessons::pages::lesson4::lesson4_handler))
        .route("/lesson-5", get(examples::lessons::pages::lesson5::lesson5_handler))
        .route("/test-global-styles", get(examples::test_global_styles::handler))
        .route("/azumi-plus", get(examples::azumi_plus_demo::azumi_plus_demo_handler))
        .merge(azumi::action::register_actions(axum::Router::new()))
        .merge(azumi::devtools::router())
        .layer(axum::middleware::from_fn(azumi::devtools::no_cache_middleware))
        .route("/lesson-1", get(examples::lessons::pages::lesson1::lesson1_handler))
        .route("/lesson-6", get(examples::lessons::pages::lesson6::lesson6_handler))
        .route("/lesson-7", get(examples::lessons::pages::lesson7::lesson7_handler))
        .route("/lesson-8", get(examples::lessons::pages::lesson8::lesson8_handler))
        .route("/lesson-9", get(examples::lessons::pages::lesson9::lesson9_handler))
        .route("/lesson-10", get(examples::lessons::pages::lesson10::lesson10_handler))
        .route("/lesson-11", get(examples::lessons::pages::lesson11::lesson11_handler))
        .route("/lesson-12", get(examples::lessons::pages::lesson12::lesson12_handler))
        .route("/lesson-13", get(examples::lessons::pages::lesson13::lesson13_handler))
        .route("/lesson-14", get(examples::lessons::pages::lesson14::lesson14_handler))
        .route("/lesson-15-sql-basics", get(examples::lessons::pages::lesson15::lesson15_handler))
        .route("/lesson-16-async-db", get(examples::lessons::pages::lesson16::lesson16_handler))
        .route("/lesson-17-testing", get(examples::lessons::pages::lesson17_testing::handler))
        .route("/lesson-18-security", get(examples::lessons::pages::lesson18_security::handler))
        .route("/lesson-19-auth", get(examples::lessons::pages::lesson19_auth::handler))
        .route("/lesson-19-login", get(examples::lessons::pages::lesson19_auth::login_handler))
        .route("/lesson-20", get(examples::lessons::pages::lesson20_sliders::lesson20_handler))

        .route("/unified-demo", get(examples::live_component_demo::unified_demo_handler))

        // 🎮 Interactive Demo Endpoints
        .route("/api/click", post(|| async { "Button clicked! 🚀" }))
        .route("/api/innerhtml", post(|| async { "Updated content!" }))
        .route("/api/append", post(|| async { "<li class='todo-item'><span class='todo-text'>New task added! ✨</span><button hx-delete='/api/todos/delete' hx-target='closest .todo-item' hx-swap='outerHTML swap:0.3s' class='todo-delete'>Delete</button></li>" }))
        .route("/api/replace", post(|| async {
            "<div style='background: #10b981; color: white; padding: 1rem; border-radius: 0.5rem; text-align: center;'>🔄 Replaced!</div>"
        }))

        // HTMX Todo handlers
        .route("/api/todos/delete", axum::routing::delete(|| async { "" }))

        // 🔒 Global Auth Middleware (Passive)
        // Applies to ALL routes above this line (Homepage, Lessons, APIs)
        // Safe because it only "checks" the cookie, doesn't "block" the request.
        .layer(axum::middleware::from_fn(examples::lessons::components::auth_infra::auth_middleware))

        // 🛡️ CSP Nonce Middleware — auto-generates per-request nonce
        // Sets Content-Security-Policy header; CspNonce extractor available in handlers
        .layer(axum::middleware::from_fn(azumi::csp::csp_nonce_layer()))

        // 📁 Static files (Legacy)
        .nest_service("/static", ServeDir::new("static"))

        // 📦 Hashed Assets (Immutable Cache)
        .nest_service("/assets", ServeDir::new(concat!(env!("OUT_DIR"), "/assets")))
        .layer(
            tower_http::set_header::SetResponseHeaderLayer::if_not_present(
                axum::http::header::CACHE_CONTROL,
                axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
            ),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("🎓 Azumi Learning Platform");
    println!("=====================================");
    println!("📍 http://localhost:8080");

    // Add Context Middleware for SEO URL inference
    let app = app.layer(axum::middleware::from_fn(
        |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| async move {
            let path = req.uri().path().to_string();
            azumi::context::with_path(path, async move { next.run(req).await }).await
        },
    ));

    axum::serve(listener, app).await.unwrap();
}
