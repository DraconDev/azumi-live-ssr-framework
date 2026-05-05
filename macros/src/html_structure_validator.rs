use crate::token_parser::Element;
use crate::token_parser::Node;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

/// Validate Raw usage patterns - ERROR on ANY Raw() in html! expressions.
///
/// Raw() bypasses ALL of Azumi's safety guarantees. Use the safe alternatives:
/// - `json_data!("var" = &data)` for JSON data injection
/// - `inline_css!(var)` for CSS content
/// - `inline_script!(var)` for JavaScript content
/// - Standard `{value}` interpolation for text (auto-escaped)
pub fn validate_raw_usage(nodes: &[Node]) -> Vec<TokenStream> {
    let mut errors = vec![];

    fn check_node(node: &Node, errors: &mut Vec<TokenStream>) {
        match node {
            Node::Expression(expr) => {
                let content_str = expr.content.to_string();
                // TokenStream may add spaces, so normalize by removing spaces for detection
                let normalized_str = content_str.replace(' ', "");
                let has_raw = normalized_str.contains("Raw(");

                if has_raw {
                    errors.push(quote_spanned! { expr.span =>
                        compile_error!(
                            "Azumi: Raw() is not allowed inside html!.\n\n\
                            Raw() bypasses ALL of Azumi's safety guarantees — escaping, CSS scoping, \n\
                            compile-time validation, and security checks.\n\
                            \n\
                            ✅ Use Azumi's safe alternatives:\n\
                            \n\
                            // For injected text content — use {value} interpolation:\n\
                            html! { <p>{value}</p> }\n\
                            \n\
                            // For JSON data to JavaScript — use json_data! macro:\n\
                            html! { {azumi::json_data!(\"MY_DATA\" = &data)} }\n\
                            \n\
                            // For CSS content — use inline_css! macro:\n\
                            html! { {azumi::inline_css!(HUB_GLOBAL_CSS)} }\n\
                            \n\
                            // For JavaScript content — use inline_script! macro:\n\
                            html! { {azumi::inline_script!(AI_HUB_COPY_JS)} }\n\
                            \n\
                            // For trusted pre-sanitized HTML — use TrustedHtml:\n\
                            html! { {TrustedHtml::new(pre_sanitized_html)} }\n\
                            \n\
                            ❌ Wrong — Raw() bypasses all safety:\n\
                            html! { @{Raw(\"...\")} }\n\
                            html! { @{Raw(format!(\"...\"))} }\n\
                            \n\
                            See: AI_GUIDE_FOR_WRITING_AZUMI.md section \"Safe Injection Macros\""
                        );
                    });
                }
            }
            Node::Element(elem) => {
                for child in &elem.children {
                    check_node(child, errors);
                }
            }
            Node::Fragment(frag) => {
                for child in &frag.children {
                    check_node(child, errors);
                }
            }
            _ => {}
        }
    }

    for node in nodes {
        check_node(node, &mut errors);
    }

    errors
}

/// Rule 10: Component Structure Enforcement
/// Enforces Script -> Content -> Style order
pub fn validate_node_order(nodes: &[Node]) -> Vec<TokenStream> {
    let mut errors = vec![];

    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    enum Phase {
        Script,
        Content,
        Style,
    }

    let mut phase = Phase::Script;

    for node in nodes {
        match node {
            // Script Handling
            Node::Element(elem) if elem.name == "script" => {
                if phase > Phase::Script {
                    let msg = "Order Error: <script> tags must be placed at the top of the component, before any HTML content.";
                    errors.push(quote_spanned! { elem.full_span =>
                        compile_error!(#msg);
                    });
                }
                // Allowed in Script Phase, stays in Script Phase
            }
            // Style Handling
            Node::Element(elem) if elem.name == "style" => {
                // <style> element (e.g., inline or src) - treat as Style phase
                phase = Phase::Style;
            }
            Node::Block(crate::token_parser::Block::Style(_)) => {
                // style! block - treat as Style phase
                phase = Phase::Style;
            }
            // Comments - Ignored, do not change phase
            Node::Comment(_) => {}
            // All other content (HTML elements, text, blocks, etc.)
            _ => {
                if phase == Phase::Style {
                    // Start of content span is tricky without a specific node match,
                    // but we can match individual types if needed or just use a generic span if available.
                    // For now, let's try to get a span from the node if possible
                    let span = match node {
                        Node::Element(e) => e.full_span,
                        Node::Text(t) => t.span,
                        Node::Expression(e) => e.span,
                        Node::Doctype(d) => d.span,
                        Node::Fragment(f) => f.span,
                        Node::Block(b) => match b {
                            crate::token_parser::Block::If(i) => i.span,
                            crate::token_parser::Block::For(f) => f.span,
                            crate::token_parser::Block::Match(m) => m.span,
                            crate::token_parser::Block::Call(c) => c.span,
                            crate::token_parser::Block::Component(c) => c.span,
                            crate::token_parser::Block::Let(l) => l.span,
                            _ => proc_macro2::Span::call_site(), // Should match matches above
                        },
                        _ => proc_macro2::Span::call_site(),
                    };

                    let msg = "Order Error: HTML structure and logic must be placed BEFORE Style blocks. Move this content above the <style> block.";
                    errors.push(quote_spanned! { span =>
                        compile_error!(#msg);
                    });
                } else {
                    // Move to Content phase if we were in Script phase
                    if phase == Phase::Script {
                        phase = Phase::Content;
                    }
                }
            }
        }
    }

    errors
}

/// Rule 1: Tables can only contain specific children
pub fn validate_table_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "table" {
        return vec![];
    }

    let valid_table_children = [
        "caption", "colgroup", "thead", "tbody", "tfoot", "tr", "style", "script", "template",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_table_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <table>. Tables can only contain: caption, colgroup, thead, tbody, tfoot, tr, style, script, template. Browser will hoist this element outside the table.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 2: Lists can only contain <li>, script, or template
pub fn validate_list_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "ul" && elem.name != "ol" {
        return vec![];
    }

    let valid_list_children = ["li", "script", "template"];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_list_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <{}>. Lists can only contain <li>, script, or template. This breaks accessibility - screen readers will report 0 items.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 3: Forms cannot be nested
pub fn validate_nested_forms(elem: &Element, is_inside_form: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    if elem.name == "form" && is_inside_form {
        let msg = "Nested <form> is not allowed. Browsers will delete nested forms, breaking submit logic.";
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 4: Buttons cannot contain interactive elements
pub fn validate_button_interactive(elem: &Element, is_inside_button: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    let interactive_elements = ["a", "button", "input", "select", "textarea", "label"];

    if is_inside_button && interactive_elements.contains(&elem.name.as_str()) {
        let msg = format!(
            "Invalid <{}> inside <button>. Buttons cannot contain interactive elements (a, button, input, select, textarea, label). This creates undefined click behavior.",
            elem.name
        );
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 5: Paragraphs cannot contain block-level elements
pub fn validate_paragraph_content(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "p" {
        return vec![];
    }

    // List of block-level elements that cannot be inside a <p>
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p
    let invalid_p_children = [
        "address",
        "article",
        "aside",
        "blockquote",
        "details",
        "div",
        "dl",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "header",
        "hgroup",
        "hr",
        "main",
        "menu",
        "nav",
        "ol",
        "p",
        "pre",
        "section",
        "table",
        "ul",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if invalid_p_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <p>. Paragraphs cannot contain block-level elements. Browsers will automatically close the <p> tag before this element, breaking your layout.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 6: Anchors cannot contain other anchors or interactive content
pub fn validate_anchor_nesting(elem: &Element, is_inside_anchor: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    if elem.name == "a" && is_inside_anchor {
        let msg = "Nested <a> tags are forbidden. Links cannot contain other links.";
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 7: Headings cannot contain other headings or block-level content
pub fn validate_heading_content(elem: &Element) -> Vec<TokenStream> {
    let headings = ["h1", "h2", "h3", "h4", "h5", "h6"];
    if !headings.contains(&elem.name.as_str()) {
        return vec![];
    }

    let mut errors = vec![];

    // Headings can only contain phrasing content.
    // We'll check for common block-level elements that are definitely wrong.
    let invalid_heading_children = [
        "div",
        "p",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "ul",
        "ol",
        "li",
        "blockquote",
        "form",
        "table",
        "header",
        "footer",
        "main",
        "section",
        "article",
        "aside",
        "nav",
    ];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if invalid_heading_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <{}>. Headings can only contain phrasing content (text, span, em, strong, etc.), not block-level elements.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 8: Tag names must be valid HTML5 tags or custom elements (with dashes)
pub fn validate_tag_name(elem: &Element) -> Option<TokenStream> {
    let name = &elem.name;

    // Allow custom elements (must contain a dash)
    if name.contains('-') {
        return None;
    }

    // Allow standard HTML5 tags
    // Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
    let valid_tags = [
        "a",
        "abbr",
        "address",
        "area",
        "article",
        "aside",
        "audio",
        "b",
        "base",
        "bdi",
        "bdo",
        "blockquote",
        "body",
        "br",
        "button",
        "canvas",
        "caption",
        "cite",
        "code",
        "col",
        "colgroup",
        "data",
        "datalist",
        "dd",
        "del",
        "details",
        "dfn",
        "dialog",
        "div",
        "dl",
        "dt",
        "em",
        "embed",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "head",
        "header",
        "hgroup",
        "hr",
        "html",
        "i",
        "iframe",
        "img",
        "input",
        "ins",
        "kbd",
        "label",
        "legend",
        "li",
        "link",
        "main",
        "map",
        "mark",
        "menu",
        "meta",
        "meter",
        "nav",
        "noscript",
        "object",
        "ol",
        "optgroup",
        "option",
        "output",
        "p",
        "picture",
        "pre",
        "progress",
        "q",
        "rp",
        "rt",
        "ruby",
        "s",
        "samp",
        "script",
        "search",
        "section",
        "select",
        "slot",
        "small",
        "source",
        "span",
        "strong",
        "style",
        "sub",
        "summary",
        "sup",
        "svg",
        "table",
        "tbody",
        "td",
        "template",
        "textarea",
        "tfoot",
        "th",
        "thead",
        "time",
        "title",
        "tr",
        "track",
        "u",
        "ul",
        "var",
        "video",
        "wbr",
        // SVG tags (common ones)
        "path",
        "circle",
        "rect",
        "line",
        "polyline",
        "polygon",
        "text",
        "g",
        "defs",
        "symbol",
        "use",
        "image",
        "clipPath",
        "mask",
        "pattern",
        "linearGradient",
        "radialGradient",
        "stop",
        "animate",
        "animateTransform",
        "mpath",
        "set",
        // MathML (basic)
        "math",
        "mi",
        "mn",
        "mo",
        "ms",
        "mtext",
        "mrow",
        "mfrac",
        "msqrt",
        "mroot",
    ];

    if !valid_tags.contains(&name.as_str()) {
        let msg = format!(
            "Unknown tag <{}>. If this is a custom element, it must contain a dash (e.g., <my-component>). If it's a standard HTML tag, check for typos.",
            name
        );
        return Some(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    None
}

/// Rule 9: Attribute names must be valid HTML attributes, data-*, aria-*, or event handlers
pub fn validate_attribute_name(attr: &crate::token_parser::Attribute) -> Option<TokenStream> {
    let name = &attr.name;

    // 1. Allow ANY attribute containing a hyphen
    // This covers data-*, aria-*, hx-*, x-*, and any future library using hyphenated attributes.
    if name.contains('-') {
        return None;
    }

    // 2. Check for Azumi event DSL (on:event) vs native HTML events (onevent)
    if name.starts_with("on:") {
        // This is Azumi's event DSL like on:click, on:mouseover - allowed
        return None;
    }
    if let Some(stripped) = name.strip_prefix("on") {
        // Native HTML event handler like onclick, onmouseover
        // Suggest using Azumi's on:event syntax instead
        let suggestion = format!("on:{}", stripped.to_lowercase());
        let msg = format!(
            "Native event handler '{}' found. Did you mean to use Azumi's event DSL '{}'?\n\n\
             Azumi uses a different syntax for events:\n\
             ❌ onclick={{handle_click()}}\n\
             ✅ on:click={{handle_click}}\n\n\
             Note: In Azumi, 'on:click' means 'call handle_click when clicked'.",
            name, suggestion
        );
        return Some(quote! {
            compile_error!(#msg);
        });
    }

    // 3. Allow XML namespaces (xmlns) - though usually contains colon or is just xmlns
    if name == "xmlns" || name.contains(':') {
        return None;
    }

    // 4. Standard HTML Global Attributes
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    let global_attributes = [
        "accesskey",
        "autocapitalize",
        "autofocus",
        "class",
        "contenteditable",
        "dir",
        "draggable",
        "enterkeyhint",
        "hidden",
        "id",
        "inert",
        "inputmode",
        "is",
        "itemid",
        "itemprop",
        "itemref",
        "itemscope",
        "itemtype",
        "lang",
        "nonce",
        "part",
        "popover",
        "role",
        "slot",
        "spellcheck",
        "style",
        "tabindex",
        "title",
        "translate",
        "virtualkeyboardpolicy",
    ];

    if global_attributes.contains(&name.as_str()) {
        return None;
    }

    // 9. Specific Element Attributes (Common ones)
    // This is a large list, but essential for strictness.
    // We'll include common attributes for all standard tags.
    let common_attributes = [
        "accept",
        "accept-charset",
        "action",
        "align",
        "allow",
        "alt",
        "async",
        "autocomplete",
        "autoplay",
        "background",
        "bgcolor",
        "border",
        "capture",
        "charset",
        "checked",
        "cite",
        "cols",
        "colspan",
        "content",
        "controls",
        "coords",
        "crossorigin",
        "datetime",
        "decoding",
        "default",
        "defer",
        "dirname",
        "disabled",
        "download",
        "enctype",
        "for",
        "form",
        "formaction",
        "formenctype",
        "formmethod",
        "formnovalidate",
        "formtarget",
        "headers",
        "height",
        "high",
        "href",
        "hreflang",
        "http-equiv",
        "integrity",
        "kind",
        "label",
        "list",
        "loading",
        "loop",
        "low",
        "max",
        "maxlength",
        "media",
        "method",
        "min",
        "minlength",
        "multiple",
        "muted",
        "name",
        "novalidate",
        "open",
        "optimum",
        "pattern",
        "placeholder",
        "playsinline",
        "poster",
        "preload",
        "property",
        "readonly",
        "referrerpolicy",
        "rel",
        "required",
        "reversed",
        "rows",
        "rowspan",
        "sandbox",
        "scope",
        "selected",
        "shape",
        "size",
        "sizes",
        "span",
        "src",
        "srcdoc",
        "srclang",
        "srcset",
        "start",
        "step",
        "target",
        "type",
        "usemap",
        "value",
        "width",
        "wrap",
        // SVG specific (common)
        "viewBox",
        "d",
        "fill",
        "stroke",
        "stroke-width",
        "stroke-linecap",
        "stroke-linejoin",
        "transform",
        "opacity",
        "points",
        "cx",
        "cy",
        "r",
        "rx",
        "ry",
        "x",
        "y",
        "x1",
        "y1",
        "x2",
        "y2",
        "preserveAspectRatio",
    ];

    if common_attributes.contains(&name.as_str()) {
        return None;
    }

    // If we get here, it's an unknown attribute
    let msg = format!(
        "Unknown attribute '{}'. Check for typos. If this is a custom attribute, prefix it with 'data-' (e.g., data-{}).",
        name, name
    );

    Some(quote_spanned! { attr.span =>
        compile_error!(#msg);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_parser::Expression;
    use proc_macro2::TokenStream;

    fn create_expression_node(content: &str) -> Node {
        let tokens: TokenStream = content.parse().unwrap();
        Node::Expression(Expression {
            content: tokens,
            span: proc_macro2::Span::call_site(),
        })
    }

    #[test]
    fn test_css_in_raw_detected_style_tag() {
        let node = create_expression_node(r#"Raw("<style>.foo { color: red; }</style>")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Should detect CSS in Raw with <style> tag"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Error should mention Raw is not allowed, got: {}",
            error_str
        );
    }

    #[test]
    fn test_css_in_raw_detected_dot_class_syntax() {
        let node = create_expression_node(r#"Raw(".main { color: blue; }")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Should detect CSS in Raw with .class syntax"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Error should mention Raw is not allowed, got: {}",
            error_str
        );
    }

    #[test]
    fn test_css_in_raw_detected_property_syntax() {
        let node = create_expression_node(r#"Raw(".container { padding: 1rem; }")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Should detect CSS in Raw with property syntax"
        );
    }

    #[test]
    fn test_css_in_raw_detected_color_property() {
        let node = create_expression_node(r#"Raw("{ color: #fff; }")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Should detect CSS color property in Raw"
        );
    }

    #[test]
    fn test_azumi_script_in_raw_now_blocked() {
        let node = create_expression_node(r#"Raw(azumi_script())"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Raw(azumi_script()) should now be blocked - use {{azumi_script()}} instead"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Should mention Raw is not allowed, got: {}",
            error_str
        )
    }

    #[test]
    fn test_window_location_in_raw_now_blocked() {
        let node = create_expression_node(r#"Raw("window.location.hash")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "Raw with window.location should now be blocked"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Should mention Raw is not allowed, got: {}",
            error_str
        );
    }

    #[test]
    fn test_suspicious_format_not_allowed() {
        let node = create_expression_node(r#"Raw(format!("<div>{}</div>", user_input))"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "format! with user input should be blocked"
        );
    }

    #[test]
    fn test_format_in_raw_always_blocked() {
        let node = create_expression_node(r#"Raw(format!("<div>{}</div>", some_var))"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "format! inside Raw() should always be blocked"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Error should mention Raw is not allowed, got: {}",
            error_str
        );
    }

    #[test]
    fn test_js_in_raw_detected() {
        let node = create_expression_node(r#"Raw("<script>alert('hi')</script>")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "JS inside Raw should be blocked"
        );
        let error_str = errors[0].to_string();
        assert!(
            error_str.contains("Raw() is not allowed"),
            "Error should mention Raw is not allowed, got: {}",
            error_str
        );
    }

    #[test]
    fn test_js_event_listener_in_raw_blocked() {
        let node = create_expression_node(r#"Raw("element.addEventListener('click', fn)")"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            !errors.is_empty(),
            "addEventListener inside Raw should be blocked"
        );
    }

    #[test]
    fn test_safe_expression_no_error() {
        let node = create_expression_node(r#"some_variable"#);
        let errors = validate_raw_usage(&[node]);

        assert!(
            errors.is_empty(),
            "Non-Raw expression should have no errors, got: {:?}",
            errors
        );
    }

    // =========================================================================
    // Structural Validator Tests
    // =========================================================================

    // Helper to create a simple element node
    fn element_node(name: &str) -> Node {
        Node::Element(Element {
            name: name.to_string(),
            attrs: vec![],
            children: vec![],
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        })
    }

    fn element_with_children(name: &str, children: Vec<Node>) -> Node {
        Node::Element(Element {
            name: name.to_string(),
            attrs: vec![],
            children,
            bind_struct: None,
            span: proc_macro2::Span::call_site(),
            full_span: proc_macro2::Span::call_site(),
        })
    }

    fn text_node(content: &str) -> Node {
        Node::Text(crate::token_parser::Text {
            content: content.to_string(),
            span: proc_macro2::Span::call_site(),
        })
    }

    fn element_with_attrs(name: &str, attrs: Vec<(&str, &str)>) -> Node {
        let mut elem = Element {
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

    // validate_node_order
    #[test]
    fn test_valid_order_content_then_style() {
        let nodes = vec![
            element_node("div"),
            Node::Block(crate::token_parser::Block::Style(crate::token_parser::StyleBlock { content: proc_macro2::TokenStream::new(), is_global: false, span: proc_macro2::Span::call_site() })),
        ];
        let errors = validate_node_order(&nodes);
        assert!(errors.is_empty(), "Content before style should be valid");
    }

    #[test]
    fn test_invalid_order_style_then_content() {
        let nodes = vec![
            Node::Block(crate::token_parser::Block::Style(crate::token_parser::StyleBlock { content: proc_macro2::TokenStream::new(), is_global: false, span: proc_macro2::Span::call_site() })),
            element_node("div"),
        ];
        let errors = validate_node_order(&nodes);
        assert!(!errors.is_empty(), "Content after style should be invalid");
    }

    #[test]
    fn test_valid_order_script_then_content_then_style() {
        let nodes = vec![
            element_node("script"),
            element_node("div"),
            Node::Block(crate::token_parser::Block::Style(crate::token_parser::StyleBlock { content: proc_macro2::TokenStream::new(), is_global: false, span: proc_macro2::Span::call_site() })),
        ];
        let errors = validate_node_order(&nodes);
        assert!(errors.is_empty(), "Script -> Content -> Style should be valid");
    }

    #[test]
    fn test_invalid_order_script_after_content() {
        let nodes = vec![
            element_node("div"),
            element_node("script"),
        ];
        let errors = validate_node_order(&nodes);
        assert!(!errors.is_empty(), "Script after content should be invalid");
    }

    // validate_table_children
    #[test]
    fn test_table_with_valid_children() {
        let table = element_with_children("table", vec![
            element_node("tr"),
            element_node("thead"),
            element_node("tbody"),
        ]);
        if let Node::Element(ref elem) = table {
            let errors = validate_table_children(elem);
            assert!(errors.is_empty(), "table with valid children should pass");
        }
    }

    #[test]
    fn test_table_with_invalid_child() {
        let table = element_with_children("table", vec![
            element_node("div"),
        ]);
        if let Node::Element(ref elem) = table {
            let errors = validate_table_children(elem);
            assert!(!errors.is_empty(), "div inside table should fail");
        }
    }

    #[test]
    fn test_non_table_ignored() {
        let div = element_with_children("div", vec![element_node("span")]);
        if let Node::Element(ref elem) = div {
            let errors = validate_table_children(elem);
            assert!(errors.is_empty(), "non-table should be ignored");
        }
    }

    // validate_list_children
    #[test]
    fn test_ul_with_li_children() {
        let ul = element_with_children("ul", vec![
            element_node("li"),
            element_node("li"),
        ]);
        if let Node::Element(ref elem) = ul {
            let errors = validate_list_children(elem);
            assert!(errors.is_empty(), "ul with li children should pass");
        }
    }

    #[test]
    fn test_ul_with_invalid_child() {
        let ul = element_with_children("ul", vec![
            element_node("div"),
        ]);
        if let Node::Element(ref elem) = ul {
            let errors = validate_list_children(elem);
            assert!(!errors.is_empty(), "div inside ul should fail");
        }
    }

    #[test]
    fn test_non_list_ignored() {
        let div = element_with_children("div", vec![element_node("span")]);
        if let Node::Element(ref elem) = div {
            let errors = validate_list_children(elem);
            assert!(errors.is_empty(), "non-list should be ignored");
        }
    }

    // validate_nested_forms
    #[test]
    fn test_form_inside_form_fails() {
        let form = element_node("form");
        if let Node::Element(ref elem) = form {
            let errors = validate_nested_forms(elem, true);
            assert!(!errors.is_empty(), "nested form should fail");
        }
    }

    #[test]
    fn test_form_not_inside_form_passes() {
        let form = element_node("form");
        if let Node::Element(ref elem) = form {
            let errors = validate_nested_forms(elem, false);
            assert!(errors.is_empty(), "form not nested should pass");
        }
    }

    #[test]
    fn test_non_form_ignored() {
        let div = element_node("div");
        if let Node::Element(ref elem) = div {
            let errors = validate_nested_forms(elem, true);
            assert!(errors.is_empty(), "non-form should be ignored even inside form");
        }
    }

    // validate_button_interactive
    #[test]
    fn test_button_inside_button_fails() {
        let button = element_node("button");
        if let Node::Element(ref elem) = button {
            let errors = validate_button_interactive(elem, true);
            assert!(!errors.is_empty(), "button inside button should fail");
        }
    }

    #[test]
    fn test_input_inside_button_fails() {
        let input = element_node("input");
        if let Node::Element(ref elem) = input {
            let errors = validate_button_interactive(elem, true);
            assert!(!errors.is_empty(), "input inside button should fail");
        }
    }

    #[test]
    fn test_span_inside_button_passes() {
        let span = element_node("span");
        if let Node::Element(ref elem) = span {
            let errors = validate_button_interactive(elem, true);
            assert!(errors.is_empty(), "span inside button should pass");
        }
    }

    #[test]
    fn test_not_inside_button_passes() {
        let button = element_node("button");
        if let Node::Element(ref elem) = button {
            let errors = validate_button_interactive(elem, false);
            assert!(errors.is_empty(), "not inside button should pass");
        }
    }

    // validate_paragraph_content
    #[test]
    fn test_div_inside_p_fails() {
        let p = element_with_children("p", vec![element_node("div")]);
        if let Node::Element(ref elem) = p {
            let errors = validate_paragraph_content(elem);
            assert!(!errors.is_empty(), "div inside p should fail");
        }
    }

    #[test]
    fn test_span_inside_p_passes() {
        let p = element_with_children("p", vec![element_node("span")]);
        if let Node::Element(ref elem) = p {
            let errors = validate_paragraph_content(elem);
            assert!(errors.is_empty(), "span inside p should pass");
        }
    }

    #[test]
    fn test_non_p_ignored() {
        let div = element_with_children("div", vec![element_node("div")]);
        if let Node::Element(ref elem) = div {
            let errors = validate_paragraph_content(elem);
            assert!(errors.is_empty(), "non-p should be ignored");
        }
    }

    // validate_anchor_nesting
    #[test]
    fn test_anchor_inside_anchor_fails() {
        let a = element_node("a");
        if let Node::Element(ref elem) = a {
            let errors = validate_anchor_nesting(elem, true);
            assert!(!errors.is_empty(), "nested anchor should fail");
        }
    }

    #[test]
    fn test_anchor_not_nested_passes() {
        let a = element_node("a");
        if let Node::Element(ref elem) = a {
            let errors = validate_anchor_nesting(elem, false);
            assert!(errors.is_empty(), "anchor not nested should pass");
        }
    }

    // validate_heading_content
    #[test]
    fn test_div_inside_h1_fails() {
        let h1 = element_with_children("h1", vec![element_node("div")]);
        if let Node::Element(ref elem) = h1 {
            let errors = validate_heading_content(elem);
            assert!(!errors.is_empty(), "div inside h1 should fail");
        }
    }

    #[test]
    fn test_span_inside_h1_passes() {
        let h1 = element_with_children("h1", vec![element_node("span")]);
        if let Node::Element(ref elem) = h1 {
            let errors = validate_heading_content(elem);
            assert!(errors.is_empty(), "span inside h1 should pass");
        }
    }

    #[test]
    fn test_non_heading_ignored() {
        let div = element_with_children("div", vec![element_node("div")]);
        if let Node::Element(ref elem) = div {
            let errors = validate_heading_content(elem);
            assert!(errors.is_empty(), "non-heading should be ignored");
        }
    }

    // validate_tag_name
    #[test]
    fn test_valid_tag_div() {
        let div = element_node("div");
        if let Node::Element(ref elem) = div {
            let result = validate_tag_name(elem);
            assert!(result.is_none(), "div should be a valid tag");
        }
    }

    #[test]
    fn test_custom_element_with_dash_passes() {
        let custom = element_node("my-component");
        if let Node::Element(ref elem) = custom {
            let result = validate_tag_name(elem);
            assert!(result.is_none(), "custom element with dash should pass");
        }
    }

    #[test]
    fn test_invalid_tag_fails() {
        let unknown = element_node("notatag");
        if let Node::Element(ref elem) = unknown {
            let result = validate_tag_name(elem);
            assert!(result.is_some(), "unknown tag should fail");
        }
    }

    // validate_attribute_name
    #[test]
    fn test_valid_attr_class() {
        let attr = crate::token_parser::Attribute {
            name: "class".to_string(),
            name_span: proc_macro2::Span::call_site(),
            value: crate::token_parser::AttributeValue::Static("foo".to_string()),
            span: proc_macro2::Span::call_site(),
            value_span: None,
        };
        let result = validate_attribute_name(&attr);
        assert!(result.is_none(), "class should be a valid attribute");
    }

    #[test]
    fn test_data_attr_passes() {
        let attr = crate::token_parser::Attribute {
            name: "data-value".to_string(),
            name_span: proc_macro2::Span::call_site(),
            value: crate::token_parser::AttributeValue::Static("123".to_string()),
            span: proc_macro2::Span::call_site(),
            value_span: None,
        };
        let result = validate_attribute_name(&attr);
        assert!(result.is_none(), "data-* should be a valid attribute");
    }

    #[test]
    fn test_on_event_passes() {
        let attr = crate::token_parser::Attribute {
            name: "on:click".to_string(),
            name_span: proc_macro2::Span::call_site(),
            value: crate::token_parser::AttributeValue::Static("handler".to_string()),
            span: proc_macro2::Span::call_site(),
            value_span: None,
        };
        let result = validate_attribute_name(&attr);
        assert!(result.is_none(), "on:event should be a valid attribute");
    }

    #[test]
    fn test_native_onclick_fails() {
        let attr = crate::token_parser::Attribute {
            name: "onclick".to_string(),
            name_span: proc_macro2::Span::call_site(),
            value: crate::token_parser::AttributeValue::Static("handler".to_string()),
            span: proc_macro2::Span::call_site(),
            value_span: None,
        };
        let result = validate_attribute_name(&attr);
        assert!(result.is_some(), "native onclick should fail with suggestion");
    }

    #[test]
    fn test_unknown_attr_fails() {
        let attr = crate::token_parser::Attribute {
            name: "notanattr".to_string(),
            name_span: proc_macro2::Span::call_site(),
            value: crate::token_parser::AttributeValue::Static("value".to_string()),
            span: proc_macro2::Span::call_site(),
            value_span: None,
        };
        let result = validate_attribute_name(&attr);
        assert!(result.is_some(), "unknown attribute should fail");
    }
}
