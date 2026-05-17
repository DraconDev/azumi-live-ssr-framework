#[cfg(feature = "schema")]
use azumi::Schema;
use azumi::{html, seo, test};

// Initialize SEO once for all tests in this file
fn init_test_seo() {
    let config = seo::SeoConfig::new("Test Site")
        .with_description("Test Description")
        .with_image("/default-og.jpg");
    let _ = seo::init_seo(config);
}

// ════════════════════════════════════════════════════════════════════════════
// Layout that renders SEO tags (the correct pattern)
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn SeoLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <html>
            <head>
                {seo::render_automatic_seo()}
            </head>
            <body>
                {children}
            </body>
        </html>
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SEO Tests Matrix
// ════════════════════════════════════════════════════════════════════════════

#[azumi::page]
fn seo_page_simple() -> impl azumi::Component {
    html! {
        @SeoLayout {
            <h1>"Simple"</h1>
        }
    }
}

#[test]
fn test_seo_inference_simple() {
    init_test_seo();
    let comp = seo_page_simple();
    let html = test::render(&comp);
    assert!(
        html.contains("<title>Seo Page Simple"),
        "Expected title 'Seo Page Simple' in:\n{}",
        html
    );
}

/// My Page Description
#[azumi::page]
fn seo_page_with_desc() -> impl azumi::Component {
    html! {
        @SeoLayout {
            <h1>"Desc"</h1>
        }
    }
}

#[test]
fn test_seo_inference_desc() {
    init_test_seo();
    let comp = seo_page_with_desc();
    let html = test::render(&comp);
    assert!(
        html.contains("content=\"My Page Description\""),
        "Expected description in:\n{}",
        html
    );
}

#[test]
fn test_manual_head_macro() {
    init_test_seo();
    let head = azumi::head! {
        title: "Manual Title",
        description: "Manual Desc",
        image: "/img.jpg",
        url: "https://ex.com",
        type: "website"
    };
    let html = test::render(&head);
    assert!(html.contains("<title>Manual Title"));
    assert!(html.contains("content=\"Manual Desc\""));
    assert!(html.contains("property=\"og:url\" content=\"https://ex.com\""));
    assert!(html.contains("property=\"og:type\" content=\"website\""));
}

// ════════════════════════════════════════════════════════════════════════════
// Schema.org Matrix
// ════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "BlogPosting")]
struct Post {
    headline: String,
    date_published: String,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_blog_posting() {
    let post = Post {
        headline: "News".into(),
        date_published: "2024-01-01".into(),
    };
    let script = post.to_schema_script();
    assert!(script.contains("BlogPosting"));
    assert!(script.contains("News"));
}

#[cfg(feature = "schema")]
#[derive(Schema)]
#[schema(type = "Product")]
struct Product {
    name: String,
    sku: String,
    price: f64,
}

#[cfg(feature = "schema")]
#[test]
fn test_schema_product() {
    let p = Product {
        name: "Gear".into(),
        sku: "G1".into(),
        price: 99.0,
    };
    let script = p.to_schema_script();
    assert!(script.contains("Product"));
    assert!(script.contains("G1"));
}

// ════════════════════════════════════════════════════════════════════════════
// Layout Interactions
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn ComplexLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div id={"layout-root"}>
            <header>
                {seo::render_automatic_seo()}
            </header>
            <main>{children}</main>
        </div>
    }
}

/// Nested SEO Page
#[azumi::page]
fn nested_page() -> impl azumi::Component {
    html! {
        @ComplexLayout {
            <h1>"Nested Content"</h1>
        }
    }
}

#[test]
fn test_layout_seo_propagation() {
    init_test_seo();
    let comp = nested_page();
    let html = test::render(&comp);
    assert!(
        html.contains("<title>Nested Page"),
        "Expected title 'Nested Page' in:\n{}",
        html
    );
    assert!(
        html.contains("id=\"layout-root\""),
        "Expected layout-root in:\n{}",
        html
    );
}
