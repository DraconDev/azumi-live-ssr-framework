//! Integration Tests for Safe Injection Macros
//!
//! These tests verify the macros work correctly in real-world scenarios,
//! including full page composition and edge cases.
//!
//! Run with: cargo test --test integration_inject_tests --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Full Page Composition
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_complete_page_with_all_macros() {
    let css = ".container { padding: 20px; }";
    let js = "console.log('page loaded');";
    let config = serde_json::json!({
        "apiUrl": "https://api.example.com",
        "version": "1.0.0"
    });

    let component = html! {
        <html>
            <head>
                <title>"Test Page"</title>
            </head>
            <body>
                <div class="container">
                    <h1>"Welcome"</h1>
                    {azumi::inline_css!(css)}
                    {azumi::inline_script!(js)}
                    {azumi::json_data!("APP_CONFIG" = &config)}
                </div>
            </body>
        </html>
    };

    let output = test::render(&component);
    assert!(output.contains("<html>"));
    assert!(output.contains("<head>"));
    assert!(output.contains("<body>"));
    assert!(output.contains("<title>Test Page</title>"));
    assert!(output.contains(".container { padding: 20px; }"));
    assert!(output.contains("console.log('page loaded')"));
    assert!(output.contains("APP_CONFIG"));
    assert!(output.contains("apiUrl"));
}

#[test]
fn test_macro_siblings_with_regular_html() {
    let css = ".card { background: white; }";
    let js = "init();";
    let data = serde_json::json!({"id": 42});

    let component = html! {
        <main>
            <article>
                <h2>"Article Title"</h2>
                <p>"Some content here"</p>
            </article>
            <aside>
                {azumi::inline_css!(css)}
                {azumi::json_data!("ARTICLE" = &data)}
                {azumi::inline_script!(js)}
                <div class="widget">"Widget content"</div>
            </aside>
        </main>
    };

    let output = test::render(&component);
    assert!(output.contains("Article Title"));
    assert!(output.contains("Some content here"));
    assert!(output.contains("Widget content"));
    assert!(output.contains("ARTICLE"));
    assert!(output.contains("id"));
}

#[test]
fn test_macro_inside_component() {
    let js = "setupNavigation();";
    let component = html! {
        <nav>
            <ul>
                <li>"Home"</li>
                <li>"About"</li>
            </ul>
            {azumi::inline_script!(js)}
        </nav>
    };

    let output = test::render(&component);
    assert!(output.contains("<nav>"));
    assert!(output.contains("Home"));
    assert!(output.contains("setupNavigation"));
}

#[test]
fn test_json_data_followed_by_script_usage() {
    let config = serde_json::json!({"theme": "dark"});
    let init_js = "applyTheme(config.theme);";

    let component = html! {
        <head>
            {azumi::json_data!("CONFIG" = &config)}
            {azumi::inline_script!(init_js)}
        </head>
    };

    let output = test::render(&component);
    let config_pos = output.find("CONFIG");
    let script_pos = output.find("applyTheme");
    assert!(config_pos.is_some());
    assert!(script_pos.is_some());
}

// ════════════════════════════════════════════════════════════════════════════
// Real-World Patterns
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_app_config_pattern() {
    let config = serde_json::json!({
        "apiUrl": "https://api.example.com/v1",
        "wsUrl": "wss://ws.example.com",
        "features": {
            "darkMode": true,
            "notifications": false
        }
    });

    let component = html! {
        <head>
            {azumi::json_data!("APP_CONFIG" = &config)}
        </head>
    };

    let output = test::render(&component);
    assert!(output.contains("APP_CONFIG"));
    assert!(output.contains("apiUrl"));
    assert!(output.contains("wss://ws.example.com"));
    assert!(output.contains("darkMode"));
}

#[test]
fn test_theme_css_pattern() {
    static DARK_THEME_CSS: &str = r#"
        :root {
            --bg-primary: #1a1a1a;
            --text-primary: #ffffff;
        }
        .card {
            background: var(--bg-primary);
            color: var(--text-primary);
        }
    "#;

    let component = html! {
        <head>
            {azumi::inline_css!(DARK_THEME_CSS)}
        </head>
    };

    let output = test::render(&component);
    assert!(output.contains("--bg-primary"));
    assert!(output.contains("--text-primary"));
    assert!(output.contains(".card"));
}

#[test]
fn test_tracking_script_pattern() {
    static GA_SCRIPT: &str = r#"
        window.ga = window.ga || function() {
            (window.ga.q = window.ga.q || []).push(arguments);
        };
        window.ga.l = 1 * new Date();
    "#;

    let component = html! {
        <head>
            {azumi::inline_script!(GA_SCRIPT)}
        </head>
    };

    let output = test::render(&component);
    assert!(output.contains("window.ga"));
    assert!(output.contains("window.ga.l"));
}

#[test]
fn test_inline_script_with_dom_ready() {
    let js = r#"
        document.addEventListener('DOMContentLoaded', function() {
            document.getElementById('app').classList.add('loaded');
        });
    "#;

    let component = html! {
        <body>
            <div id="app">"Loading..."</div>
            {azumi::inline_script!(js)}
        </body>
    };

    let output = test::render(&component);
    assert!(output.contains("DOMContentLoaded"));
    assert!(output.contains("app"));
}

#[test]
fn test_inline_css_with_component_scoped() {
    let css = r#"
        .profile-card {
            border: 1px solid #ddd;
            border-radius: 8px;
            padding: 16px;
        }
        .profile-card__avatar {
            width: 48px;
            height: 48px;
            border-radius: 50%;
        }
    "#;

    let component = html! {
        <div class="profile-card">
            <img class="profile-card__avatar" src="/avatar.png" alt="Avatar" />
            <span>"Username"</span>
            {azumi::inline_css!(css)}
        </div>
    };

    let output = test::render(&component);
    assert!(output.contains(".profile-card"));
    assert!(output.contains(".profile-card__avatar"));
    assert!(output.contains("border-radius"));
}

// ════════════════════════════════════════════════════════════════════════════
// Empty/Edge Case Payloads
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_json_data_renders() {
    let data = serde_json::json!({});
    let component = html! { {azumi::json_data!("EMPTY" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("EMPTY"));
    assert!(output.contains("{}"));
}

#[test]
fn test_empty_inline_css_renders() {
    let css = "";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains("<style>"));
    assert!(output.contains("</style>"));
}

#[test]
fn test_empty_inline_script_renders() {
    let js = "";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(output.contains("<script>"));
    assert!(output.contains("</script>"));
}

#[test]
fn test_json_data_with_unicode_content() {
    let data = serde_json::json!({
        "name": "Müller",
        "city": "São Paulo",
        "notes": "第一第②"
    });

    let component = html! {
        <div>
            {azumi::json_data!("I18N" = &data)}
        </div>
    };

    let output = test::render(&component);
    assert!(output.contains("Müller"));
    assert!(output.contains("São Paulo"));
}

#[test]
fn test_inline_script_with_unicode() {
    let js = "console.log('你好'); console.log('🎉');";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(output.contains("你好"));
    assert!(output.contains("🎉"));
}

#[test]
fn test_inline_css_with_unicode_emoji() {
    let css = r#"
        .emoji-test::before {
            content: "🚀 🇺🇸 💻";
        }
    "#;
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains("🚀"));
}

// ════════════════════════════════════════════════════════════════════════════
// Multiple Injections in Sequence
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_three_json_data_macros_sequential() {
    let d1 = serde_json::json!({"a": 1});
    let d2 = serde_json::json!({"b": 2});
    let d3 = serde_json::json!({"c": 3});

    let component = html! {
        <div>
            {azumi::json_data!("A" = &d1)}
            {azumi::json_data!("B" = &d2)}
            {azumi::json_data!("C" = &d3)}
        </div>
    };

    let output = test::render(&component);
    assert!(output.contains("A = {\"a\":1}"));
    assert!(output.contains("B = {\"b\":2}"));
    assert!(output.contains("C = {\"c\":3}"));
}

#[test]
fn test_css_then_script_then_json() {
    let css = ".a { color: red; }";
    let js = "console.log('test');";
    let data = serde_json::json!({"key": "val"});

    let component = html! {
        <div>
            {azumi::inline_css!(css)}
            {azumi::inline_script!(js)}
            {azumi::json_data!("DATA" = &data)}
        </div>
    };

    let output = test::render(&component);
    let css_pos = output.find(".a { color: red; }");
    let js_pos = output.find("console.log('test')");
    let data_pos = output.find("DATA");

    assert!(css_pos.is_some() && js_pos.is_some() && data_pos.is_some());
}

// ════════════════════════════════════════════════════════════════════════════
// Error Handling / Resilience
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_large_object() {
    let mut data = serde_json::Map::new();
    for i in 0..100 {
        data.insert(format!("key{}", i), serde_json::json!(i));
    }
    let data = serde_json::json!(data);

    let component = html! { {azumi::json_data!("LARGE" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("LARGE"));
    assert!(output.contains("key0"));
    assert!(output.contains("key99"));
}

#[test]
fn test_inline_css_with_long_selector_list() {
    let css = r#"
        .btn, .btn-primary, .btn-secondary, .btn-success,
        .btn-danger, .btn-warning, .btn-info {
            display: inline-block;
            padding: 8px 16px;
        }
    "#;

    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains(".btn"));
    assert!(output.contains(".btn-danger"));
    assert!(output.contains("display: inline-block"));
}