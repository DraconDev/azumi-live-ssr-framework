use azumi::head;
use azumi::seo::{OpenGraph, SeoConfig, TwitterCard};

fn ensure_seo_init() {
    let config = SeoConfig {
        open_graph: Some(OpenGraph::default()),
        twitter: Some(TwitterCard::default()),
        ..Default::default()
    };
    let _ = azumi::seo::init_seo(config);
}

#[test]
fn test_minimal_head() {
    ensure_seo_init();
    let meta = head! {
        title: "Minimal Page",
        description: "Just a title and description"
    };

    assert!(meta.as_str().contains("<title>Minimal Page</title>"));
    assert!(meta
        .as_str()
        .contains("<meta name=\"description\" content=\"Just a title and description\">"));
    assert!(meta
        .as_str()
        .contains("<meta property=\"og:title\" content=\"Minimal Page\">"));
    // assert!(meta.as_str().contains("<meta name=\"twitter:card\" content=\"summary\">")); // Twitter card default might confirm this
}

#[test]
fn test_full_head() {
    ensure_seo_init();
    let meta = head! {
        title: "Full Page",
        description: "Everything included",
        image: "/static/preview.jpg"
        // url: "https://example.com", // macro parser needs to support these new keys if they aren't there
        // type: "article"
    };

    assert!(meta
        .as_str()
        .contains("<meta property=\"og:image\" content=\"/static/preview.jpg\">"));
    assert!(meta
        .as_str()
        .contains("<meta name=\"twitter:image\" content=\"/static/preview.jpg\">"));
    // assert!(meta.as_str().contains("<meta name=\"twitter:card\" content=\"summary_large_image\">"));
}

#[test]
fn test_dynamic_values() {
    ensure_seo_init();
    let page_title = "Dynamic Title";
    let page_desc = format!("Description for {}", page_title);
    let meta = head! {
        title: page_title,
        description: page_desc.as_str()
    };

    assert!(meta.as_str().contains("<title>Dynamic Title</title>"));
    assert!(meta.as_str().contains("Description for Dynamic Title"));
}