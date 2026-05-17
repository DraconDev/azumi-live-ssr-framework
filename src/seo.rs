use std::fmt::Write;
use std::sync::RwLock;

use crate::Component;

/// Global site-wide SEO configuration.
///
/// # Limitation
///
/// This is a process-global singleton set once via `init_seo()`. It is NOT
/// per-request — all requests share the same SEO config (title, base URL,
/// OpenGraph defaults).
///
/// For per-request SEO or multi-tenant applications, use `generate_head_with()`
/// which accepts an explicit `SeoConfig` parameter.
static SITE_CONFIG: RwLock<Option<SeoConfig>> = RwLock::new(None);

#[derive(Clone, Default, Debug)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    pub type_: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub struct TwitterCard {
    pub card: String,
    pub site: Option<String>,
    pub creator: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub struct SeoConfig {
    pub title: String,
    pub description: Option<String>,
    pub canonical_url: Option<String>,
    pub base_url: Option<String>,
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<TwitterCard>,
}

impl SeoConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        let img = image.into();
        let mut og = self.open_graph.unwrap_or_default();
        og.image = Some(img.clone());
        self.open_graph = Some(og);

        let mut tw = self.twitter.take().unwrap_or_else(|| TwitterCard {
            card: "summary".to_string(),
            ..Default::default()
        });
        tw.image = Some(img);
        self.twitter = Some(tw);
        self
    }
}

/// Initialize the global SEO configuration.
///
/// Returns `Ok(())` on first call, `Err(config)` on subsequent calls
/// (the returned config is the one that was rejected).
///
/// # Example
///
/// ```rust,ignore
/// let config = SeoConfig::new("My Site").with_description("A cool site");
/// if let Err(rejected) = init_seo(config) {
///     eprintln!("SEO already initialized, rejected: {:?}", rejected);
/// }
/// ```
pub fn init_seo(config: SeoConfig) -> Result<(), Box<SeoConfig>> {
    if let Ok(mut guard) = SITE_CONFIG.write() {
        if guard.is_none() {
            *guard = Some(config);
            Ok(())
        } else {
            Err(Box::new(config))
        }
    } else {
        Err(Box::new(config))
    }
}

#[cfg(test)]
pub fn reset_seo() {
    if let Ok(mut guard) = SITE_CONFIG.write() {
        *guard = None;
    }
}

pub fn generate_head(
    title: &str,
    description: Option<&str>,
    image: Option<&str>,
    url: Option<&str>,
    type_: Option<&str>,
) -> HeadContent {
    let global = SITE_CONFIG.read().ok().and_then(|guard| guard.clone());
    generate_head_with(title, description, image, url, type_, global.as_ref())
}

/// Generate SEO head content with explicit site configuration.
///
/// This is the primary API for per-request or multi-tenant SEO, where different
/// pages or tenants need different site-wide settings (site name, base URL,
/// OpenGraph defaults, Twitter card settings).
///
/// The `site_config` parameter provides site-wide defaults that supplement
/// the per-page parameters. Pass `None` to use only the per-page values.
///
/// # Example
///
/// ```ignore
/// let site = SeoConfig::new("My Site")
///     .with_description("Default site description")
///     .with_image("/default-og.png");
///
/// let head = generate_head_with(
///     "Blog Post Title",
///     Some("Post-specific description"),
///     Some("/post-image.png"),
///     None,
///     Some("article"),
///     Some(&site),
/// );
/// ```
pub fn generate_head_with(
    title: &str,
    description: Option<&str>,
    image: Option<&str>,
    url: Option<&str>,
    type_: Option<&str>,
    site_config: Option<&SeoConfig>,
) -> HeadContent {
    let global = site_config.cloned();
    let context_meta = crate::context::get_page_meta();
    let current_path = crate::context::get_current_path();

    let effective_title = resolve_title(title, &context_meta);
    let full_title = build_full_title(&effective_title, &global);
    let effective_desc = resolve_description(description, &context_meta, &global);
    let effective_image = resolve_image(image, &context_meta, &global);
    let full_url = resolve_url(url, &global, &current_path);
    let effective_type = type_.unwrap_or("website");

    let mut html = String::new();

    let safe_title = crate::escape_html(&full_title);
    let safe_desc = effective_desc.as_deref().map(crate::escape_html);
    let safe_url = full_url.as_deref().map(crate::escape_html);
    let safe_image = effective_image.as_deref().map(crate::escape_html);

    render_basic_meta(&mut html, &safe_title, safe_desc.as_deref(), safe_url.as_deref());
    render_open_graph(&mut html, &global, &safe_title, safe_desc.as_deref(), safe_url.as_deref(), safe_image.as_deref(), effective_type);
    render_twitter_card(&mut html, &global, &safe_title, safe_desc.as_deref(), safe_image.as_deref());

    HeadContent(html)
}

/// Resolve the effective title from local, context, or global config.
fn resolve_title(title: &str, context_meta: &crate::context::PageMeta) -> String {
    if !title.is_empty() {
        title.to_string()
    } else {
        context_meta.title.clone().unwrap_or_default()
    }
}

/// Build the full title, appending site name if available.
fn build_full_title(effective_title: &str, global: &Option<SeoConfig>) -> String {
    if let Some(ref g) = global {
        if let Some(ref og) = g.open_graph {
            if let Some(ref site_name) = og.site_name {
                if !effective_title.is_empty() {
                    return format!("{} | {}", effective_title, site_name);
                } else {
                    return site_name.clone();
                }
            }
        }
    }
    effective_title.to_string()
}

/// Resolve description from local, context, or global config.
fn resolve_description(
    description: Option<&str>,
    context_meta: &crate::context::PageMeta,
    global: &Option<SeoConfig>,
) -> Option<String> {
    description
        .map(|s| s.to_string())
        .or_else(|| context_meta.description.clone())
        .or_else(|| global.as_ref().and_then(|g| g.description.clone()))
}

/// Resolve image from local, context, or global config.
fn resolve_image(
    image: Option<&str>,
    context_meta: &crate::context::PageMeta,
    global: &Option<SeoConfig>,
) -> Option<String> {
    image
        .map(|s| s.to_string())
        .or_else(|| context_meta.image.clone())
        .or_else(|| {
            global
                .as_ref()
                .and_then(|g| g.open_graph.as_ref().and_then(|og| og.image.clone()))
        })
}

/// Resolve the canonical URL from explicit, base+path, or global config.
fn resolve_url(
    url: Option<&str>,
    global: &Option<SeoConfig>,
    current_path: &Option<String>,
) -> Option<String> {
    if let Some(u) = url {
        return Some(u.to_string());
    }

    let base_url = global.as_ref().and_then(|g| g.base_url.clone());
    match (base_url, current_path) {
        (Some(base), Some(path)) => {
            let base_clean = base.trim_end_matches('/');
            let path_clean = path.strip_prefix('/').unwrap_or(path);
            Some(format!("{}/{}", base_clean, path_clean))
        }
        (Some(base), None) => Some(base),
        _ => None,
    }
}

/// Render basic meta tags: title, description, canonical.
fn render_basic_meta(
    html: &mut String,
    safe_title: &str,
    safe_desc: Option<&str>,
    safe_url: Option<&str>,
) {
    let _ = write!(html, "<title>{}</title>", safe_title);
    if let Some(d) = safe_desc {
        let _ = write!(html, r#"<meta name="description" content="{}">"#, d);
    }
    if let Some(url) = safe_url {
        let _ = write!(html, r#"<link rel="canonical" href="{}">"#, url);
    }
}

/// Render Open Graph meta tags.
fn render_open_graph(
    html: &mut String,
    global: &Option<SeoConfig>,
    safe_title: &str,
    safe_desc: Option<&str>,
    safe_url: Option<&str>,
    safe_image: Option<&str>,
    effective_type: &str,
) {
    if let Some(ref g) = global {
        if let Some(ref og) = g.open_graph {
            let _ = write!(
                html,
                r#"<meta property="og:title" content="{}">"#,
                safe_title
            );
            if let Some(d) = safe_desc {
                let _ = write!(html, r#"<meta property="og:description" content="{}">"#, d);
            }
            if let Some(u) = safe_url {
                let _ = write!(html, r#"<meta property="og:url" content="{}">"#, u);
            }
            if let Some(i) = safe_image {
                let _ = write!(html, r#"<meta property="og:image" content="{}">"#, i);
            }
            if let Some(s) = &og.site_name {
                let safe_s = crate::escape_html(s);
                let _ = write!(
                    html,
                    r#"<meta property="og:site_name" content="{}">"#,
                    safe_s
                );
            }
            let safe_type = crate::escape_html(effective_type);
            let _ = write!(html, r#"<meta property="og:type" content="{}">"#, safe_type);
        }
    }
}

/// Render Twitter Card meta tags.
fn render_twitter_card(
    html: &mut String,
    global: &Option<SeoConfig>,
    safe_title: &str,
    safe_desc: Option<&str>,
    safe_image: Option<&str>,
) {
    if let Some(ref g) = global {
        if let Some(ref tw) = g.twitter {
            let safe_card = crate::escape_html(&tw.card);
            let _ = write!(
                html,
                r#"<meta name="twitter:card" content="{}">"#,
                safe_card
            );
            if let Some(s) = &tw.site {
                let safe_s = crate::escape_html(s);
                let _ = write!(html, r#"<meta name="twitter:site" content="{}">"#, safe_s);
            }
            if let Some(c) = &tw.creator {
                let safe_c = crate::escape_html(c);
                let _ = write!(
                    html,
                    r#"<meta name="twitter:creator" content="{}">"#,
                    safe_c
                );
            }
            let _ = write!(
                html,
                r#"<meta name="twitter:title" content="{}">"#,
                safe_title
            );
            if let Some(d) = safe_desc {
                let _ = write!(html, r#"<meta name="twitter:description" content="{}">"#, d);
            }
            if let Some(i) = safe_image {
                let _ = write!(html, r#"<meta name="twitter:image" content="{}">"#, i);
            }
        }
    }
}

/// Public wrapper for SEO head content. Returned by `generate_head()`.
///
/// The inner string is private to prevent construction of unescaped head content
/// from outside the module. Use `generate_head()` or `render_automatic_seo()`
/// to create instances, and `as_str()` to read the rendered HTML.
pub struct HeadContent(String);

impl HeadContent {
    /// Access the rendered HTML string.
    ///
    /// The string contains properly escaped HTML meta tags ready for
    /// inclusion in a `<head>` element.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Component for HeadContent {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for HeadContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn render_automatic_seo() -> HeadContent {
    generate_head("", None, None, None, None)
}

pub struct SitemapBuilder {
    base_url: String,
    urls: Vec<String>,
}

/// Normalize a URL path by resolving `.` and `..` segments.
/// Returns None if the path attempts to escape above the root.
/// Preserves the original structure including trailing slashes.
fn normalize_path(path: &str) -> Option<String> {
    // Don't normalize empty paths or simple valid paths without dots
    if !path.contains('.') {
        return Some(path.to_string());
    }

    let mut segments: Vec<&str> = Vec::new();
    let parts: Vec<&str> = path.split('/').collect();

    for segment in &parts {
        match *segment {
            "" | "." => {} // Ignore empty and current-dir segments
            ".." => {
                segments.pop()?; // Path traversal attempt: .. above root
            }
            _ => segments.push(segment),
        }
    }

    // Reconstruct the path, preserving leading slash if original had it
    let mut result = String::new();
    if path.starts_with('/') {
        result.push('/');
    }
    result.push_str(&segments.join("/"));

    // Preserve trailing slash if original ended with /
    if path.ends_with('/') && !result.ends_with('/') {
        result.push('/');
    }

    Some(result)
}

/// Check if a path contains URL-encoded traversal sequences.
fn contains_encoded_traversal(path: &str) -> bool {
    path.contains("%2e%2e")
        || path.contains("%2E%2E")
        || path.contains("%2e%2E")
        || path.contains("%2E%2e")
        || path.contains("%252e%252e")
        || path.contains("..%2f")
        || path.contains("..%2F")
        || path.contains("%2f..%2f")
        || path.contains("%2F..%2F")
}

impl SitemapBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            urls: Vec::new(),
        }
    }

    pub fn add_url(mut self, path: impl Into<String>) -> Self {
        self.urls.push(path.into());
        self
    }

    pub fn build(self) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
        );

        let base = self.base_url.trim_end_matches('/');

        let base_origin = if let Some(scheme_pos) = base.find("://") {
            let scheme_end = scheme_pos + 3;
            if scheme_end < base.len() {
                let after_scheme = &base[scheme_end..];
                if let Some(path_pos) = after_scheme.find('/') {
                    &base[..scheme_end + path_pos]
                } else {
                    base
                }
            } else {
                base
            }
        } else {
            base
        };

        for path in self.urls {
            // Reject URL-encoded path traversal attempts
            if contains_encoded_traversal(&path) {
                eprintln!(
                    "SEO Warning: Path '{}' contains encoded traversal sequences, skipping",
                    path
                );
                continue;
            }

            let url = if path.starts_with("http") {
                let scheme_pos = path.find("://").map(|p| p + 3).unwrap_or(0);
                let after_scheme = &path[scheme_pos..];
                let path_slash = after_scheme.find('/').unwrap_or(after_scheme.len());
                let url_origin = &path[..scheme_pos + path_slash];

                let base_scheme_pos = base_origin.find("://").map(|p| p + 3).unwrap_or(0);
                let base_after_scheme = &base_origin[base_scheme_pos..];
                let base_path_slash = base_after_scheme
                    .find('/')
                    .unwrap_or(base_after_scheme.len());
                let base_origin_exact = &base_origin[..base_scheme_pos + base_path_slash];

                if url_origin != base_origin_exact {
                    eprintln!(
                        "SEO Warning: Absolute URL '{}' doesn't match base origin '{}', skipping",
                        path, base_origin_exact
                    );
                    continue;
                }
                path.to_string()
            } else {
                // Normalize the path to resolve . and .. segments
                let normalized = match normalize_path(&path) {
                    Some(n) => n,
                    None => {
                        eprintln!(
                            "SEO Warning: Path '{}' attempts directory traversal above root, skipping",
                            path
                        );
                        continue;
                    }
                };

                let candidate = format!(
                    "{}{}",
                    base,
                    normalized
                );

                // Verify the resolved path stays under the base URL
                if !candidate.starts_with(base_origin) {
                    eprintln!(
                        "SEO Warning: Path '{}' resolves outside base URL, skipping",
                        path
                    );
                    continue;
                }
                candidate
            };

            let escaped_url = crate::escape_xml(&url);
            let _ = write!(
                xml,
                "  <url>\n    <loc>{}</loc>\n    <changefreq>weekly</changefreq>\n  </url>\n",
                escaped_url
            );
        }

        xml.push_str("</urlset>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_head_basic() {
        reset_seo();
        let result = generate_head("Test Title", Some("Test desc"), None, None, None);
        let html = crate::render_to_string(&result);
        assert!(html.contains("<title>Test Title</title>"));
        assert!(html.contains(r#"name="description""#));
        assert!(html.contains("Test desc"));
    }

    #[test]
    fn test_generate_head_empty_title_fallback() {
        let result = generate_head("", None, None, None, None);
        let html = crate::render_to_string(&result);
        assert!(html.contains("<title>"));
    }

    #[test]
    fn test_generate_head_xss_prevention_in_title() {
        let result = generate_head(
            "<script>alert('xss')</script>",
            None,
            None,
            None,
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_generate_head_xss_prevention_in_description() {
        let result = generate_head(
            "Title",
            Some("<script>alert('xss')</script>"),
            None,
            None,
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn test_generate_head_xss_prevention_in_url() {
        let result = generate_head(
            "Title",
            None,
            None,
            Some("https://example.com?q=<script>"),
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_generate_head_with_image() {
        reset_seo();
        let _ = init_seo(SeoConfig {
            title: "Site".to_string(),
            open_graph: Some(OpenGraph {
                title: None,
                description: None,
                url: None,
                image: Some("https://example.com/default.png".to_string()),
                site_name: Some("Site".to_string()),
                locale: None,
                type_: None,
            }),
            ..Default::default()
        });
        let result = generate_head(
            "Title",
            None,
            Some("https://example.com/image.png"),
            None,
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(html.contains("og:image"));
        assert!(html.contains("example.com/image.png"));
    }

    #[test]
    fn test_generate_head_open_graph() {
        reset_seo();
        let _ = init_seo(SeoConfig {
            title: "Site".to_string(),
            open_graph: Some(OpenGraph {
                title: None,
                description: None,
                url: None,
                image: None,
                site_name: Some("My Site".to_string()),
                locale: None,
                type_: None,
            }),
            ..Default::default()
        });
        let result = generate_head("Page Title", None, None, None, None);
        let html = crate::render_to_string(&result);
        assert!(html.contains("og:title"));
        assert!(html.contains("og:site_name"));
        assert!(html.contains("My Site"));
    }

    #[test]
    fn test_generate_head_canonical_url() {
        let result = generate_head(
            "Title",
            None,
            None,
            Some("https://example.com/page"),
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(html.contains("canonical"));
        assert!(html.contains("example.com/page"));
    }

    #[test]
    fn test_generate_head_with_type() {
        reset_seo();
        let _ = init_seo(SeoConfig {
            title: "Site".to_string(),
            open_graph: Some(OpenGraph {
                site_name: Some("TestSite".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        });
        let result = generate_head("Title", None, None, None, Some("article"));
        let html = crate::render_to_string(&result);
        assert!(html.contains("og:type"));
    }

    #[test]
    fn test_sitemap_builder_basic() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/page1")
            .add_url("/page2")
            .build();
        assert!(sitemap.contains("<?xml"));
        assert!(sitemap.contains("https://example.com/page1"));
        assert!(sitemap.contains("https://example.com/page2"));
        assert!(sitemap.contains("</urlset>"));
    }

    #[test]
    fn test_sitemap_builder_absolute_url_matching_base() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("https://example.com/absolute")
            .build();
        assert!(sitemap.contains("https://example.com/absolute"));
    }

    #[test]
    fn test_sitemap_builder_rejects_cross_origin() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("https://evil.com/malicious")
            .build();
        assert!(!sitemap.contains("evil.com"));
    }

    #[test]
    fn test_sitemap_builder_normalizes_path_trailing_slash() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/page/")
            .build();
        assert!(sitemap.contains("example.com/page/"));
    }

    #[test]
    fn test_sitemap_builder_resolves_dot_dot() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/foo/bar/../baz")
            .build();
        assert!(sitemap.contains("example.com/foo/baz"));
        assert!(!sitemap.contains("/foo/bar/../"));
    }

    #[test]
    fn test_sitemap_builder_xml_escapes_ampersand() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/page?q=test&v=1")
            .build();
        assert!(sitemap.contains("&amp;"));
        assert!(!sitemap.contains("&lt;"));
    }

    #[test]
    fn test_sitemap_builder_empty_urls() {
        let sitemap = SitemapBuilder::new("https://example.com").build();
        assert!(sitemap.contains("<?xml"));
        assert!(sitemap.contains("</urlset>"));
        assert!(!sitemap.contains("<loc>"));
    }

    #[test]
    fn test_escape_html_double_quote() {
        assert_eq!(crate::escape_html("say \"hello\""), "say &quot;hello&quot;");
    }

    #[test]
    fn test_escape_html_single_quote() {
        assert_eq!(crate::escape_html("it's"), "it&#x27;s");
    }

    #[test]
    fn test_escape_html_xss_payload() {
        let input = "<img src=x onerror=alert(1)>";
        let escaped = crate::escape_html(input);
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("onerror"));
    }

    #[test]
    fn test_escape_xml_escapes_ampersand() {
        assert_eq!(crate::escape_xml("a & b"), "a &amp; b");
    }

    #[test]
    fn test_escape_xml_all_special_chars() {
        assert_eq!(crate::escape_xml("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
    }

    #[test]
    fn test_render_automatic_seo_empty_context() {
        let result = render_automatic_seo();
        let html = crate::render_to_string(&result);
        assert!(html.starts_with("<title>"));
    }

    #[test]
    fn test_head_content_as_str_matches_render() {
        let result = generate_head("Test", Some("Desc"), None, None, None);
        assert_eq!(result.as_str(), crate::render_to_string(&result));
    }

    #[test]
    fn test_head_content_as_str_returns_html() {
        let result = generate_head("My Page", None, None, None, None);
        let s = result.as_str();
        assert!(s.contains("<title>"), "as_str() should return rendered HTML");
        assert!(s.contains("My Page"));
    }

    #[test]
    fn test_generate_head_with_explicit_config() {
        // Test generate_head_with with explicit SeoConfig (no global needed)
        reset_seo(); // Clear global config
        let site = SeoConfig {
            title: "Explicit Site".to_string(),
            open_graph: Some(OpenGraph {
                site_name: Some("Explicit Site".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let result = generate_head_with(
            "Page Title",
            Some("Page-specific description"),
            None,
            None,
            Some("article"),
            Some(&site),
        );
        let html = crate::render_to_string(&result);

        // Site name is only appended via OpenGraph site_name, not title tag
        // Title tag shows just the page title; site name goes in og:site_name
        assert!(html.contains("Page Title"));
        assert!(html.contains("og:site_name"));
        assert!(html.contains("Explicit Site"));
        // Description should use page-specific
        assert!(html.contains("Page-specific description"));
    }

    #[test]
    fn test_generate_head_with_no_config() {
        // Test generate_head_with with None config (works without any global)
        reset_seo();
        let result = generate_head_with(
            "Standalone Title",
            Some("Standalone description"),
            None,
            None,
            None,
            None,
        );
        let html = crate::render_to_string(&result);

        assert!(html.contains("<title>Standalone Title</title>"));
        assert!(html.contains("Standalone description"));
    }
}