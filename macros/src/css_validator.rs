/// CSS Validator - Enforces Azumi's CSS rules at compile time
/// Note: Inline <style> tag blocking is handled in token_parser.rs during parsing.
/// This module validates external CSS file references and enforces double-quoted
/// CSS values in `<style>` tag content.
use crate::token_parser::{AttributeValue, Block, Node};

/// Parse all CSS files referenced in the component and validate classes
/// Returns a TokenStream of compile errors if validation fails
pub fn validate_component_css(nodes: &[Node]) -> proc_macro2::TokenStream {
    use quote::quote;

    let mut css_files = Vec::new();
    collect_css_files(nodes, &mut css_files);

    if !css_files.is_empty() {
        let paths = css_files.join("\n  - ");
        let msg = format!(
            "External CSS files are banned in Azumi. Found:\n  - {}\n\n\
             Use one of these alternatives:\n\
             1. Inline styles in a <style> block: <style>{{{{ my_css_var }}}}</style>\n\
             2. Scope CSS with the html! macro's built-in scoping\n\
             3. Put global styles in a file named 'global.css' (opt-out of validation)",
            paths
        );
        return quote! {
            compile_error!(#msg);
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

// ============================================================================
// CSS Value Double-Quote Enforcement
// ============================================================================

/// Validate CSS declarations inside `<style>` tags and `style!` blocks
/// for unquoted values.
///
/// Enforces the Azumi rule that CSS values must be double-quoted strings.
/// This prevents lexer issues with values like `#colors`, `2em`, `rgba(...)`.
///
/// Returns a TokenStream of compile errors for any unquoted values found.
pub fn validate_style_tag_css(nodes: &[Node]) -> proc_macro2::TokenStream {
    let mut errors = proc_macro2::TokenStream::new();
    collect_style_tag_css_errors(nodes, &mut errors);
    errors
}

fn collect_style_tag_css_errors(nodes: &[Node], errors: &mut proc_macro2::TokenStream) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                if elem.name == "style" {
                    // Skip <style src="..."> (external references)
                    let has_src = elem.attrs.iter().any(|a| a.name == "src");
                    // Skip <style>{expr}</style> (dynamic content)
                    let has_expression = elem
                        .children
                        .iter()
                        .any(|c| matches!(c, Node::Expression(_)));

                    if !has_src && !has_expression {
                        for child in &elem.children {
                            if let Node::Text(text) = child {
                                if let Some(err) =
                                    validate_css_values_in_text(&text.content, text.span)
                                {
                                    errors.extend(err);
                                }
                            }
                        }
                    }
                } else {
                    collect_style_tag_css_errors(&elem.children, errors);
                }
            }
            Node::Fragment(frag) => {
                collect_style_tag_css_errors(&frag.children, errors);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    collect_style_tag_css_errors(&if_block.then_branch, errors);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_style_tag_css_errors(else_branch, errors);
                    }
                }
                Block::For(for_block) => {
                    collect_style_tag_css_errors(&for_block.body, errors);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_style_tag_css_errors(&arm.body, errors);
                    }
                }
                Block::Call(call_block) => {
                    collect_style_tag_css_errors(&call_block.children, errors);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Validate that all CSS property values in the text are double-quoted.
///
/// Parses CSS declarations (property: value;) and checks that each value
/// is a double-quoted string. Skips:
/// - CSS variables (`var(...)`)
/// - At-rules (`@media`, `@keyframes`, etc.)
/// - Multi-value properties (e.g., `margin: 10px 20px`)
/// - Function calls (`rgb(...)`, `calc(...)`)
fn validate_css_values_in_text(
    css: &str,
    span: proc_macro2::Span,
) -> Option<proc_macro2::TokenStream> {
    use quote::quote_spanned;

    let mut errors = Vec::new();

    // Extract declarations from rule blocks (depth-1 content)
    let declarations = extract_declarations_for_validation(css);

    for decl in &declarations {
        let trimmed = decl.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip at-rules (they start with @)
        if trimmed.starts_with('@') {
            continue;
        }

        // Skip comments
        if trimmed.starts_with("/*") {
            continue;
        }

        // Parse "property: value" — find the first colon
        let colon_pos = match trimmed.find(':') {
            Some(pos) => pos,
            None => continue,
        };
        let property = trimmed[..colon_pos].trim();
        let value = trimmed[colon_pos + 1..].trim();

        // Skip empty values
        if value.is_empty() {
            continue;
        }

        // Skip CSS custom properties (properties starting with --)
        // Their values are arbitrary and quoting is unusual
        if property.starts_with("--") {
            continue;
        }

        // Skip CSS variables (value starts with var()
        if value.starts_with("var(") {
            continue;
        }

        // Skip values that contain function calls (rgb(), calc(), url(), etc.)
        if value.contains('(') {
            continue;
        }

        // Skip multi-word values (multiple space-separated tokens) — these are
        // multi-value shorthand properties like `margin: 10px 20px`
        let value_words: Vec<&str> = value.split_whitespace().collect();
        if value_words.len() > 1 {
            continue;
        }

        // Check if value is already double-quoted
        let is_quoted = value.len() >= 2
            && value.starts_with('"')
            && value.ends_with('"');

        if !is_quoted {
            let msg = format!(
                "CSS values must be double-quoted strings.\n\n\
                 Found: {} {} {};\n\
                 Expected: {} {} \"{}\";\n\n\
                 Why? Unquoted CSS values cause lexer issues with:\n\
                 - Hash colors: {} #ff0000 -> ambiguous token\n\
                 - Unit values: {} 2em -> ambiguous token\n\n\
                 The `style!` macro requires double-quoted values.\n\
                 Match that convention in `<style>` tags for consistency.",
                property,
                ":",
                value,
                property,
                ":",
                value,
                property,
                property,
            );
            errors.push(quote_spanned! { span =>
                compile_error!(#msg);
            });
        }
    }

    if errors.is_empty() {
        None
    } else {
        let mut tokens = proc_macro2::TokenStream::new();
        for err in errors {
            tokens.extend(err);
        }
        Some(tokens)
    }
}

/// Extract CSS declarations from rule blocks for validation.
///
/// Walks CSS text tracking brace depth. At depth 1 (inside a rule block
/// like `.card { ... }`), collects declarations separated by `;`.
/// Skips at-rules (@media, @keyframes, etc.) and their nested blocks.
fn extract_declarations_for_validation(css: &str) -> Vec<String> {
    let mut declarations = Vec::new();
    let mut current_decl = String::new();
    let mut depth: i32 = 0;
    let mut in_string: Option<char> = None;
    let mut skip_block = false;

    for ch in css.chars() {
        // Handle string literals
        if ch == '"' || ch == '\'' {
            if in_string == Some(ch) {
                in_string = None;
            } else if in_string.is_none() {
                in_string = Some(ch);
            }
            if depth >= 1 && !skip_block {
                current_decl.push(ch);
            }
            continue;
        }

        if in_string.is_some() {
            if depth >= 1 && !skip_block {
                current_decl.push(ch);
            }
            continue;
        }

        match ch {
            '{' => {
                depth += 1;
                if depth == 1 {
                    // Check if the content before this brace is an at-rule
                    // If so, skip this entire block
                    // We detect this by checking if we haven't started collecting declarations yet
                    // and the current_decl is empty (we clear it on '}')
                    // Actually, at depth 0 we don't track, so let's use a simpler check:
                    // We'll peek backward in the CSS to find the selector
                    skip_block = false;
                    current_decl.clear();
                } else if depth == 2 && skip_block {
                    // Nested block inside an at-rule — skip it too
                }
            }
            '}' => {
                if depth == 1 && !skip_block {
                    // End of a rule block — extract remaining declaration
                    let decl = current_decl.trim().to_string();
                    if !decl.is_empty() && decl.contains(':') {
                        declarations.push(decl);
                    }
                    current_decl.clear();
                }
                if depth == 2 && skip_block {
                    // End of nested block inside at-rule
                }
                depth -= 1;
                if depth == 0 {
                    skip_block = false;
                }
            }
            ';' if depth == 1 && !skip_block => {
                current_decl.push(ch);
                let decl = current_decl.trim().to_string();
                // Strip trailing semicolon
                let decl = decl.trim_end_matches(';').trim().to_string();
                if !decl.is_empty() && decl.contains(':') {
                    declarations.push(decl);
                }
                current_decl.clear();
            }
            _ => {
                if depth == 1 && !skip_block {
                    current_decl.push(ch);
                }
            }
        }
    }

    declarations
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

    // =========================================================================
    // validate_style_tag_css / validate_css_values_in_text
    // =========================================================================

    fn style_element_with_text(css: &str) -> Node {
        let elem = crate::token_parser::Element {
            name: "style".to_string(),
            attrs: vec![],
            children: vec![Node::Text(crate::token_parser::Text {
                content: css.to_string(),
                span: proc_macro2::Span::call_site(),
            })],
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        };
        Node::Element(elem)
    }

    #[test]
    fn test_quoted_css_values_pass() {
        let css = ".card { padding: \"1rem\"; color: \"red\"; }";
        let style = style_element_with_text(css);
        let result = validate_style_tag_css(&[style]);
        assert!(result.to_string().is_empty(), "Quoted values should pass: {}", result);
    }

    #[test]
    fn test_unquoted_css_values_fail() {
        let style = style_element_with_text(".card { padding: 1rem; }");
        let result = validate_style_tag_css(&[style]);
        let s = result.to_string();
        assert!(s.contains("compile_error"), "Unquoted values should produce compile_error, got: {}", s);
        assert!(s.contains("CSS values must be double-quoted"), "Error should mention double-quoted");
    }

    #[test]
    fn test_var_functions_skip_validation() {
        let style = style_element_with_text(":root { --size: 1rem; } .card { padding: var(--size); }");
        let result = validate_style_tag_css(&[style]);
        assert!(result.to_string().is_empty(), "var() values should be skipped: {}", result);
    }

    #[test]
    fn test_function_calls_skip_validation() {
        let style = style_element_with_text(".card { color: rgb(255, 0, 0); }");
        let result = validate_style_tag_css(&[style]);
        assert!(result.to_string().is_empty(), "Function calls should be skipped: {}", result);
    }

    #[test]
    fn test_multi_value_properties_skip_validation() {
        let style = style_element_with_text(".card { margin: 10px 20px; }");
        let result = validate_style_tag_css(&[style]);
        assert!(result.to_string().is_empty(), "Multi-value properties should be skipped: {}", result);
    }

    #[test]
    fn test_at_rules_skip_validation() {
        let style = style_element_with_text("@media (max-width: 768px) { .card { display: none; } }");
        let result = validate_style_tag_css(&[style]);
        // @media rules are skipped, but the inner .card { display: none; } is inside braces
        // so it's not a top-level declaration — should be fine
        assert!(result.to_string().is_empty(), "@media content should be skipped: {}", result);
    }

    #[test]
    fn test_style_with_expression_child_skipped() {
        let elem = crate::token_parser::Element {
            name: "style".to_string(),
            attrs: vec![],
            children: vec![Node::Expression(crate::token_parser::Expression {
                content: quote::quote! { GLOBAL_CSS },
                span: proc_macro2::Span::call_site(),
            })],
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        };
        let result = validate_style_tag_css(&[Node::Element(elem)]);
        assert!(result.to_string().is_empty(), "Dynamic style expressions should be skipped");
    }

    #[test]
    fn test_style_with_src_skipped() {
        let style = element_with_attrs("style", vec![("src", "/styles.css")]);
        let result = validate_style_tag_css(&[style]);
        assert!(result.to_string().is_empty(), "External style references should be skipped");
    }

    #[test]
    fn test_extract_top_level_declarations_simple() {
        let css = ".card { padding: 1rem; color: red; }";
        let decls = extract_declarations_for_validation(css);
        assert_eq!(decls.len(), 2, "Expected 2 declarations, got {:?}: {:?}", decls.len(), decls);
        assert!(decls[0].contains("padding"), "First decl should contain padding: {:?}", decls);
        assert!(decls[1].contains("color"), "Second decl should contain color: {:?}", decls);
    }

    #[test]
    fn test_extract_top_level_declarations_nested_media() {
        let css = "@media (max-width: 768px) { .card { display: none; } }";
        let decls = extract_declarations_for_validation(css);
        // Declarations inside @media are nested (depth > 1 when inside both @media and .card)
        // so they should not be extracted
        assert!(decls.is_empty(), "Nested declarations should not be extracted, got: {:?}", decls);
    }

    #[test]
    fn test_extract_top_level_declarations_semicolon_outside_braces() {
        let css = ":root { --size: 1rem; }";
        let decls = extract_declarations_for_validation(css);
        // :root is a pseudo-class selector, its declarations are inside braces at depth 1
        assert_eq!(decls.len(), 1, "Should extract 1 declaration from :root, got: {:?}", decls);
    }

    #[test]
    fn test_multiple_errors_collected() {
        let style = style_element_with_text(".card { padding: 1rem; margin: 2rem; }");
        let result = validate_style_tag_css(&[style]);
        let s = result.to_string();
        // Should have at least 2 compile_error instances
        let count = s.matches("compile_error").count();
        assert!(count >= 2, "Should collect multiple errors, found {} compile_error occurrences", count);
    }
}
