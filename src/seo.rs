use std::fmt::Write;
use std::sync::OnceLock;

/// Escape a string for safe inclusion in an HTML attribute value (double-quoted).
/// Prevents XSS by escaping `"`, `<`, `>`, `&`, and `'`.
fn html_attr_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape a string for safe inclusion in XML text content or attribute values.
/// Prevents malformed XML from special characters.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape a string for safe inclusion as HTML text content.
/// Escapes `<`, `>`, and `&`.
fn html_text_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    out
}

static SITE_CONFIG: OnceLock<SeoConfig> = OnceLock::new();

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

        let mut tw = self.twitter.unwrap_or_default();
        tw.image = Some(img);
        self.twitter = Some(tw);
        self
    }
}

pub fn init_seo(config: SeoConfig) {
    if SITE_CONFIG.set(config).is_err() {
        eprintln!("WARNING: init_seo() called multiple times - first initialization preserved");
    }
}

pub fn generate_head(
    title: &str,
    description: Option<&str>,
    image: Option<&str>,
    url: Option<&str>,
    type_: Option<&str>,
) -> crate::Raw<String> {
    let global = SITE_CONFIG.get();

    let context_meta = crate::context::get_page_meta();

    let effective_title = if !title.is_empty() {
        title.to_string()
    } else {
        context_meta.title.unwrap_or_default()
    };

    let effective_desc = description
        .map(|s| s.to_string())
        .or(context_meta.description)
        .or(global.and_then(|g| g.description.clone()));

    let effective_image = image
        .map(|s| s.to_string())
        .or(context_meta.image)
        .or(global.and_then(|g| g.open_graph.as_ref().and_then(|og| og.image.clone())));

    let full_title = if let Some(g) = global {
        if let Some(og) = &g.open_graph {
            if let Some(site_name) = &og.site_name {
                if !effective_title.is_empty() {
                    format!("{} | {}", effective_title, site_name)
                } else {
                    site_name.clone()
                }
            } else {
                effective_title.clone()
            }
        } else {
            effective_title.clone()
        }
    } else {
        effective_title.clone()
    };

    let current_path = crate::context::get_current_path();
    let base_url = global.and_then(|g| g.base_url.as_deref());

    let full_url = if let Some(u) = url {
        Some(u.to_string())
    } else {
        match (base_url, &current_path) {
            (Some(base), Some(path)) => {
                let base_clean = base.trim_end_matches('/');
                let path_clean = if let Some(stripped) = path.strip_prefix('/') {
                    stripped
                } else {
                    path
                };
                Some(format!("{}/{}", base_clean, path_clean))
            }
            (Some(base), None) => Some(base.to_string()),
            _ => None,
        }
    };

    let effective_type = type_.unwrap_or("website");

    let mut html = String::new();

    let safe_title = html_text_escape(&full_title);
    let safe_desc = effective_desc.as_deref().map(html_attr_escape);
    let safe_url = full_url.as_deref().map(html_attr_escape);
    let safe_image = effective_image.as_deref().map(html_attr_escape);

    let _ = write!(html, "<title>{}</title>", safe_title);
    if let Some(d) = &safe_desc {
        let _ = write!(html, r#"<meta name="description" content="{}">"#, d);
    }
    if let Some(url) = &safe_url {
        let _ = write!(html, r#"<link rel="canonical" href="{}">"#, url);
    }

    if let Some(g) = global {
        if let Some(og) = &g.open_graph {
            let _ = write!(
                html,
                r#"<meta property="og:title" content="{}">"#,
                safe_title
            );

            if let Some(d) = &safe_desc {
                let _ = write!(html, r#"<meta property="og:description" content="{}">"#, d);
            }

            if let Some(u) = &safe_url {
                let _ = write!(html, r#"<meta property="og:url" content="{}">"#, u);
            }

            if let Some(i) = &safe_image {
                let _ = write!(html, r#"<meta property="og:image" content="{}">"#, i);
            }

            if let Some(s) = &og.site_name {
                let safe_s = html_attr_escape(s);
                let _ = write!(
                    html,
                    r#"<meta property="og:site_name" content="{}">"#,
                    safe_s
                );
            }

            let safe_type = html_attr_escape(effective_type);
            let _ = write!(html, r#"<meta property="og:type" content="{}">"#, safe_type);
        }
    }

    if let Some(g) = global {
        if let Some(tw) = &g.twitter {
            let safe_card = html_attr_escape(&tw.card);
            let _ = write!(
                html,
                r#"<meta name="twitter:card" content="{}">"#,
                safe_card
            );
            if let Some(s) = &tw.site {
                let safe_s = html_attr_escape(s);
                let _ = write!(html, r#"<meta name="twitter:site" content="{}">"#, safe_s);
            }
            if let Some(c) = &tw.creator {
                let safe_c = html_attr_escape(c);
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
            if let Some(d) = &safe_desc {
                let _ = write!(html, r#"<meta name="twitter:description" content="{}">"#, d);
            }
            if let Some(i) = &safe_image {
                let _ = write!(html, r#"<meta name="twitter:image" content="{}">"#, i);
            }
        }
    }

    crate::Raw(html)
}

pub fn render_automatic_seo() -> crate::Raw<String> {
    generate_head("", None, None, None, None)
}

pub struct SitemapBuilder {
    base_url: String,
    urls: Vec<String>,
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
                let mut candidate = format!(
                    "{}{}{}",
                    base,
                    if path.starts_with('/') { "" } else { "/" },
                    path
                );
                while let Some(pos) = candidate.find("/../") {
                    let mut seg_start = pos;
                    while seg_start > 0 && candidate.as_bytes()[seg_start - 1] != b'/' {
                        seg_start -= 1;
                    }
                    if seg_start > 0 {
                        seg_start -= 1;
                        candidate = format!("{}{}", &candidate[..seg_start], &candidate[pos + 3..]);
                    } else {
                        candidate = candidate[pos + 3..].to_string();
                    }
                }
                if !candidate.starts_with(base_origin) {
                    eprintln!(
                        "SEO Warning: Path '{}' resolves outside base URL, skipping",
                        path
                    );
                    continue;
                }
                candidate
            };

            let escaped_url = xml_escape(&url);
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
            Some("https://example.com?q=<img src=x onerror=alert(1)>"),
            None,
        );
        let html = crate::render_to_string(&result);
        assert!(!html.contains("onerror"));
        assert!(html.contains("&lt;img"));
    }

    #[test]
    fn test_generate_head_with_image() {
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
    fn test_generate_head_twitter_card() {
        init_seo(SeoConfig {
            title: "Site".to_string(),
            twitter: Some(TwitterCard {
                card: "summary".to_string(),
                site: Some("@handle".to_string()),
                creator: None,
                title: None,
                description: None,
                image: None,
            }),
            ..Default::default()
        });
        let result = generate_head("Page", None, None, None, None);
        let html = crate::render_to_string(&result);
        assert!(html.contains("twitter:card"));
        assert!(html.contains("summary"));
        assert!(html.contains("twitter:site"));
        assert!(html.contains("@handle"));
    }

    #[test]
    fn test_generate_head_open_graph() {
        init_seo(SeoConfig {
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
    fn test_sitemap_builder_xml_escapes_content() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/page?q=test&v=1")
            .build();
        assert!(sitemap.contains("&amp;"));
        assert!(sitemap.contains("&lt;"));
    }

    #[test]
    fn test_sitemap_builder_empty_urls() {
        let sitemap = SitemapBuilder::new("https://example.com").build();
        assert!(sitemap.contains("<?xml"));
        assert!(sitemap.contains("</urlset>"));
        assert!(!sitemap.contains("<loc>"));
    }

    #[test]
    fn test_html_attr_escape_double_quote() {
        assert_eq!(html_attr_escape("say \"hello\""), "say &quot;hello&quot;");
    }

    #[test]
    fn test_html_attr_escape_single_quote() {
        assert_eq!(html_attr_escape("it's"), "it&#x27;s");
    }

    #[test]
    fn test_html_attr_escape_xss_payload() {
        let input = "<img src=x onerror=alert(1)>";
        let escaped = html_attr_escape(input);
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("onerror"));
    }

    #[test]
    fn test_xml_escape_escapes_ampersand() {
        assert_eq!(xml_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_sitemap_builder_xml_escapes_ampersand() {
        let sitemap = SitemapBuilder::new("https://example.com")
            .add_url("/page?q=test&v=1")
            .build();
        assert!(sitemap.contains("&amp;"));
    }

    #[test]
    fn test_xml_escape_all_special_chars() {
        assert_eq!(xml_escape("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
    }

    #[test]
    fn test_html_text_escape() {
        assert_eq!(html_text_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_text_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_html_text_escape_preserves_newlines() {
        let input = "line1\nline2";
        let escaped = html_text_escape(input);
        assert!(escaped.contains('\n'));
    }

    #[test]
    fn test_render_automatic_seo_empty_context() {
        let result = render_automatic_seo();
        let html = crate::render_to_string(&result);
        assert!(html.starts_with("<title>"));
    }
}