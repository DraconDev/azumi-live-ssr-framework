/// CSS Validator - Enforces Azumi's CSS rules at compile time
/// Note: Inline <style> tag blocking is handled in token_parser.rs during parsing.
/// This module focuses on external CSS file validation.
use crate::token_parser::{AttributeValue, Block, Node};

/// Parse all CSS files referenced in the component and validate classes
/// Returns a TokenStream of compile errors if validation fails
pub fn validate_component_css(nodes: &[Node]) -> proc_macro2::TokenStream {
    use quote::quote;

    let mut css_files = Vec::new();
    collect_css_files(nodes, &mut css_files);

    if !css_files.is_empty() {
        return quote! {
            compile_error!("External CSS files are banned in Azumi. Use the style! macro instead.");
        };
    }

    quote! {}
}

/// Collect all CSS file paths from <style src="..."> tags
fn collect_css_files(nodes: &[Node], css_files: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                if elem.name.as_str() == "style" {
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let AttributeValue::Static(path) = &src_attr.value {
                            // Skip global.css files - they are opt-out of validation
                            if !path.ends_with("global.css") {
                                let css_file_path = resolve_css_file_path(path);
                                css_files.push(css_file_path);
                            }
                        }
                    }
                }
                collect_css_files(&elem.children, css_files);
            }
            Node::Fragment(frag) => {
                collect_css_files(&frag.children, css_files);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    collect_css_files(&if_block.then_branch, css_files);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_css_files(else_branch, css_files);
                    }
                }
                Block::For(for_block) => {
                    collect_css_files(&for_block.body, css_files);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_css_files(&arm.body, css_files);
                    }
                }
                Block::Call(call_block) => {
                    collect_css_files(&call_block.children, css_files);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Resolve CSS file path from CARGO_MANIFEST_DIR
pub fn resolve_css_file_path(css_path: &str) -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = std::path::Path::new(&manifest_dir);
    let clean_path = css_path.trim_start_matches('/');

    let possible_paths = vec![
        manifest_path.join(clean_path).to_string_lossy().to_string(),
        manifest_path
            .join("static")
            .join(clean_path)
            .to_string_lossy()
            .to_string(),
        manifest_path
            .join("src")
            .join(clean_path)
            .to_string_lossy()
            .to_string(),
    ];

    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            return path.clone();
        }
    }

    manifest_path.join(clean_path).to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element_with_attrs(name: &str, attrs: Vec<(&str, &str)>) -> Node {
        let mut elem = crate::token_parser::Element {
            name: name.to_string(),
            attrs: vec![],
            children: vec![],
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        };
        for (name, value) in attrs {
            elem.attrs.push(crate::token_parser::Attribute {
                name: name.to_string(),
                name_span: proc_macro2::Span::call_site(),
                value: crate::token_parser::AttributeValue::Static(value.to_string()),
                span: proc_macro2::Span::call_site(),
                value_span: None,
            });
        }
        Node::Element(elem)
    }

    // =========================================================================
    // validate_component_css
    // =========================================================================

    #[test]
    fn test_no_external_css_returns_empty() {
        let nodes = vec![];
        let result = validate_component_css(&nodes);
        assert!(result.to_string().is_empty(), "No nodes should return empty");
    }

    #[test]
    fn test_style_without_src_returns_empty() {
        let style = element_with_attrs("style", vec![]);
        let result = validate_component_css(&[style]);
        assert!(result.to_string().is_empty(), "Style without src should be allowed");
    }

    #[test]
    fn test_style_with_src_fails() {
        let style = element_with_attrs("style", vec![("src", "/styles.css")]);
        let result = validate_component_css(&[style]);
        let s = result.to_string();
        assert!(s.contains("compile_error"), "External CSS should produce compile_error");
        assert!(s.contains("External CSS files are banned"), "Error should mention ban");
    }

    #[test]
    fn test_global_css_allowed() {
        let style = element_with_attrs("style", vec![("src", "global.css")]);
        let result = validate_component_css(&[style]);
        assert!(result.to_string().is_empty(), "global.css should be allowed");
    }

    // =========================================================================
    // collect_css_files
    // =========================================================================

    #[test]
    fn test_collect_no_css_files() {
        let nodes = vec![];
        let mut files = vec![];
        collect_css_files(&nodes, &mut files);
        assert!(files.is_empty(), "No nodes should collect no files");
    }

    #[test]
    fn test_collect_style_with_src() {
        let style = element_with_attrs("style", vec![("src", "/styles.css")]);
        let mut files = vec![];
        collect_css_files(&[style], &mut files);
        assert_eq!(files.len(), 1, "Should collect one CSS file");
        assert!(files[0].ends_with("styles.css"), "Should end with styles.css");
    }

    #[test]
    fn test_collect_skips_global_css() {
        let style = element_with_attrs("style", vec![("src", "global.css")]);
        let mut files = vec![];
        collect_css_files(&[style], &mut files);
        assert!(files.is_empty(), "global.css should be skipped");
    }

    #[test]
    fn test_collect_recurses_into_children() {
        let inner_style = element_with_attrs("style", vec![("src", "/inner.css")]);
        let div = crate::token_parser::Element {
            name: "div".to_string(),
            attrs: vec![],
            children: vec![inner_style],
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        };
        let mut files = vec![];
        collect_css_files(&[Node::Element(div)], &mut files);
        assert_eq!(files.len(), 1, "Should recurse into children");
    }

    // =========================================================================
    // resolve_css_file_path
    // =========================================================================

    #[test]
    fn test_resolve_strips_leading_slash() {
        let result = resolve_css_file_path("/styles.css");
        assert!(!result.starts_with("/styles.css"), "Should strip leading slash");
        assert!(result.ends_with("styles.css"), "Should end with styles.css");
    }

    #[test]
    fn test_resolve_relative_path() {
        let result = resolve_css_file_path("styles.css");
        assert!(result.ends_with("styles.css"), "Should end with styles.css");
    }
}
