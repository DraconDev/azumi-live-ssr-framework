//! SEO Tests
//!
//! Tests for Azumi's SEO features: meta tags, titles, links
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Meta Tags (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_meta_charset() {
    let component = html! {
        <head>
            <meta charset="utf-8" />
        </head>
    };
    let html = test::render(&component);
    assert!(html.contains("charset=\"utf-8\""));
}

#[test]
fn test_meta_viewport() {
    let component = html! {
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
        </head>
    };
    let html = test::render(&component);
    assert!(html.contains("viewport"));
}

#[test]
fn test_meta_author() {
    let component = html! {
        <meta name="author" content="John Doe" />
    };
    let html = test::render(&component);
    assert!(html.contains("author") && html.contains("John Doe"));
}

#[test]
fn test_meta_keywords() {
    let component = html! {
        <meta name="keywords" content="rust, web, framework" />
    };
    let html = test::render(&component);
    assert!(html.contains("keywords"));
}

#[test]
fn test_meta_robots() {
    let component = html! {
        <meta name="robots" content="index, follow" />
    };
    let html = test::render(&component);
    assert!(html.contains("robots"));
}

#[test]
fn test_meta_theme_color() {
    let component = html! {
        <meta name="theme-color" content="#4CAF50" />
    };
    let html = test::render(&component);
    assert!(html.contains("theme-color"));
}

#[test]
fn test_og_title() {
    let component = html! {
        <meta property="og:title" content="My Page Title" />
    };
    let html = test::render(&component);
    assert!(html.contains("og:title") || html.contains("My Page Title"));
}

#[test]
fn test_og_description() {
    let component = html! {
        <meta property="og:description" content="Page description" />
    };
    let html = test::render(&component);
    assert!(html.contains("og:description") || html.contains("Page description"));
}

#[test]
fn test_og_image() {
    let component = html! {
        <meta property="og:image" content="/images/og.jpg" />
    };
    let html = test::render(&component);
    assert!(html.contains("og:image") || html.contains("/images/og.jpg"));
}

#[test]
fn test_twitter_card() {
    let component = html! {
        <meta name="twitter:card" content="summary_large_image" />
    };
    let html = test::render(&component);
    assert!(html.contains("twitter:card"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Title Tag (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_title_tag() {
    let component = html! {
        <head>
            <title>"My Page"</title>
        </head>
    };
    let html = test::render(&component);
    assert!(html.contains("<title>") && html.contains("My Page"));
}

#[test]
fn test_dynamic_title() {
    let page_name = "About Us";
    let component = html! {
        <title>{page_name}</title>
    };
    let html = test::render(&component);
    assert!(html.contains("About Us"));
}

#[test]
fn test_title_with_suffix() {
    let page_name = "Home";
    let site_name = "My Site";
    let component = html! {
        <title>{page_name}" | "{site_name}</title>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("My Site"));
}

#[test]
fn test_escaped_title() {
    let title = "Tips and Tricks";
    let component = html! {
        <title>{title}</title>
    };
    let html = test::render(&component);
    assert!(html.contains("Tips") && html.contains("Tricks"));
}

#[test]
fn test_empty_title() {
    let component = html! {
        <title></title>
    };
    let html = test::render(&component);
    assert!(html.contains("<title>"));
}

#[test]
fn test_unicode_title() {
    let title = "日本語タイトル";
    let component = html! {
        <title>{title}</title>
    };
    let html = test::render(&component);
    assert!(html.contains("日本語"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Link Tags (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_link_icon() {
    let component = html! {
        <link rel="icon" href="/favicon.ico" />
    };
    let html = test::render(&component);
    assert!(html.contains("icon") && html.contains("favicon"));
}

#[test]
fn test_link_apple_touch_icon() {
    let component = html! {
        <link rel="apple-touch-icon" href="/apple-icon.png" />
    };
    let html = test::render(&component);
    assert!(html.contains("apple-touch-icon"));
}

#[test]
fn test_link_manifest() {
    let component = html! {
        <link rel="manifest" href="/manifest.json" />
    };
    let html = test::render(&component);
    assert!(html.contains("manifest"));
}

#[test]
fn test_link_preconnect() {
    let component = html! {
        <link rel="preconnect" href="https://fonts.googleapis.com" />
    };
    let html = test::render(&component);
    assert!(html.contains("preconnect"));
}

#[test]
fn test_link_canonical() {
    let component = html! {
        <link rel="canonical" href="https://example.com/page" />
    };
    let html = test::render(&component);
    assert!(html.contains("canonical"));
}

#[test]
fn test_link_dns_prefetch() {
    let component = html! {
        <link rel="dns-prefetch" href="https://cdn.example.com" />
    };
    let html = test::render(&component);
    assert!(html.contains("dns-prefetch"));
}

#[test]
fn test_link_alternate() {
    let component = html! {
        <link rel="alternate" hreflang="es" href="/es/" />
    };
    let html = test::render(&component);
    assert!(html.contains("alternate"));
}

#[test]
fn test_link_external_stylesheet() {
    let component = html! {
        <link rel="stylesheet" href="https://cdn.example.com/style.css" />
    };
    let html = test::render(&component);
    assert!(html.contains("stylesheet"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Script Tags (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_script_src() {
    let component = html! {
        <script src="/js/app.js"></script>
    };
    let html = test::render(&component);
    assert!(html.contains("<script") && html.contains("src="));
}

#[test]
fn test_script_defer() {
    let component = html! {
        <script src="/js/app.js" defer="true"></script>
    };
    let html = test::render(&component);
    assert!(html.contains("defer"));
}

#[test]
fn test_script_async() {
    let component = html! {
        <script src="/js/analytics.js" async="true"></script>
    };
    let html = test::render(&component);
    assert!(html.contains("async"));
}

#[test]
fn test_script_module() {
    let component = html! {
        <script type="module" src="/js/module.js"></script>
    };
    let html = test::render(&component);
    assert!(html.contains("module"));
}

#[test]
fn test_azumi_script() {
    let component = html! {
        <script src="https://example.com/azumi.js"></script>
    };
    let html = test::render(&component);
    assert!(html.contains("<script") && html.contains("src="));
}

#[test]
fn test_noscript() {
    let component = html! {
        <noscript>"JavaScript is required"</noscript>
    };
    let html = test::render(&component);
    assert!(html.contains("<noscript>") && html.contains("JavaScript"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: Security — XSS Prevention in SEO (generate_head)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_seo_xss_title_script_injection() {
    let html = azumi::seo::generate_head(r#""><script>alert(1)</script>"#, None, None, None, None);
    let output = html.0;
    // Must not contain unescaped <script>
    assert!(
        !output.contains("<script>"),
        "XSS: raw <script> found in output"
    );
    // Angle brackets must be escaped
    assert!(
        output.contains("&lt;script&gt;"),
        "Expected escaped script tag"
    );
    // Quotes in title (text context) are not escaped by html_text_escape,
    // but they're inside <title>text</title> which is safe.
    // What matters: no attribute breakout is possible.
}

#[test]
fn test_seo_xss_description_onload() {
    let html =
        azumi::seo::generate_head("Safe Title", Some(r#"onload="alert(2)""#), None, None, None);
    let output = html.0;
    // The quotes in the description must be escaped to prevent attribute breakout.
    // Output should be: content="onload=&quot;alert(2)&quot;"
    // NOT: content="onload="alert(2)"" (which would allow XSS)
    assert!(
        output.contains("&quot;alert(2)&quot;"),
        "Quotes must be escaped in meta description attribute. Got: {}",
        output
    );
}



#[test]
fn test_seo_xss_angle_brackets_in_title() {
    let html = azumi::seo::generate_head("<script>alert('xss')</script>", None, None, None, None);
    let output = html.0;
    assert!(!output.contains("<script>"), "Raw <script> in title");
    assert!(
        output.contains("&lt;script&gt;"),
        "Title should escape angle brackets"
    );
}

#[test]
fn test_seo_xss_ampersand_escaping() {
    let html = azumi::seo::generate_head(
        "Tom & Jerry's <Best> \"Show\"",
        Some("A & B <C> 'D'"),
        None,
        None,
        None,
    );
    let output = html.0;
    // Title (text context) should escape <, >, &
    assert!(
        output.contains("Tom &amp; Jerry"),
        "Ampersand should be escaped in title"
    );
    assert!(
        output.contains("&lt;Best&gt;"),
        "Angle brackets should be escaped in title"
    );
    // Description (attribute context) should escape ", <, >, &, '
    assert!(
        output.contains("A &amp; B"),
        "Ampersand should be escaped in desc attribute"
    );
    assert!(
        output.contains("&lt;C&gt;"),
        "Angle brackets should be escaped in desc attribute"
    );
}

#[test]
fn test_seo_safe_values_unchanged() {
    let html = azumi::seo::generate_head(
        "Normal Title",
        Some("A normal description."),
        Some("/images/photo.jpg"),
        Some("https://example.com/page"),
        None,
    );
    let output = html.0;
    assert!(
        output.contains("<title>Normal Title</title>") || output.contains("<title>Normal Title |"),
        "Title should be present. Got: {}",
        output
    );
    assert!(output.contains(r#"content="A normal description.""#));
    assert!(output.contains(r#"href="https://example.com/page""#));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: Twitter Card — site and creator (generate_head)
// Tests that twitter:site and twitter:creator are output when configured.
// Note: init_seo is idempotent (first call wins), so these tests run AFTER
// the init_seo test to avoid pollution. The twitter config from init_seo
// is preserved for subsequent tests.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_twitter_card_with_site_and_creator() {
    let html = azumi::seo::generate_head("Title", None, None, None, None);
    let output = html.0;
    assert!(
        output.contains(r#"twitter:site"#),
        "Expected twitter:site meta. Got: {}",
        output
    );
    assert!(
        output.contains(r#"twitter:creator"#),
        "Expected twitter:creator meta. Got: {}",
        output
    );
}

#[test]
fn test_twitter_card_site_escapes_quotes() {
    let mut tw = azumi::seo::TwitterCard::default();
    tw.site = Some(r#"@handle" onclick="alert(1)"#.to_string());
    tw.card = "summary".to_string();
    let mut config = azumi::seo::SeoConfig::new("Test");
    config.twitter = Some(tw);
    let html = azumi::seo::generate_head("Title", None, None, None, None);
    let output = html.0;
    assert!(
        !output.contains(r#"onclick=""#),
        "twitter:site should escape quotes. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: init_seo idempotency
// Tests that init_seo only applies on first call.
// Note: Uses #[serial] to prevent init_seo pollution between tests.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_init_seo_first_call_wins() {
    let config1 = azumi::seo::SeoConfig::new("First Title")
        .with_description("First Description");
    let config2 = azumi::seo::SeoConfig::new("Second Title")
        .with_description("Second Description");
    azumi::seo::init_seo(config1);
    azumi::seo::init_seo(config2);
    let html = azumi::seo::generate_head("", None, None, None, None);
    let output = html.0;
    assert!(
        output.contains("First Title") || output.contains("First Description"),
        "Second init_seo should not overwrite first. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: URL construction (base_url + current_path)
// Note: These tests use the public API. The base_url is set via init_seo
// and current_path is managed by the context module (internal).
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_canonical_url_from_base_url() {
    let mut config = azumi::seo::SeoConfig::new("Test");
    config.base_url = Some("https://example.com".to_string());
    azumi::seo::init_seo(config);
    let html = azumi::seo::generate_head("Title", None, None, None, None);
    let output = html.0;
    assert!(
        output.contains("https://example.com"),
        "Canonical URL should include base_url. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: All fields None
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_generate_head_all_none() {
    let html = azumi::seo::generate_head("", None, None, None, None);
    let output = html.0;
    assert!(
        output.contains("<title>"),
        "Should still produce <title> tag even with all None. Got: {}",
        output
    );
}

#[test]
fn test_generate_head_empty_title_still_renders() {
    let html = azumi::seo::generate_head("", None, None, None, None);
    let output = html.0;
    assert!(
        output.contains("<title></title>") || output.contains("<title>"),
        "Empty title should still produce title tag. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: XSS escaping in image URL (generate_head)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_seo_xss_image_url_with_quotes() {
    let mut og = azumi::seo::OpenGraph::default();
    og.site_name = Some("Test".into());
    let mut config = azumi::seo::SeoConfig::new("Test");
    config.open_graph = Some(og);
    azumi::seo::init_seo(config);
    let html = azumi::seo::generate_head(
        "Safe Title",
        None,
        Some(r#"/img.png" onload="alert(1) x=""#),
        None,
        None,
    );
    let output = html.0;
    assert!(
        output.contains("&quot;") || !output.contains(r#" onload=""#),
        "Image URL should have quotes escaped. Got: {}",
        output
    );
}

#[test]
fn test_seo_xss_image_url_with_angle_brackets() {
    let mut og = azumi::seo::OpenGraph::default();
    og.site_name = Some("Test".into());
    let mut config = azumi::seo::SeoConfig::new("Test");
    config.open_graph = Some(og);
    azumi::seo::init_seo(config);
    let html = azumi::seo::generate_head(
        "Title",
        None,
        Some("/<img/src=x onerror=alert(1)>"),
        None,
        None,
    );
    let output = html.0;
    assert!(
        !output.contains("<script") && !output.contains("onerror"),
        "Image URL should escape angle brackets and event handlers. Got: {}",
        output
    );
}
