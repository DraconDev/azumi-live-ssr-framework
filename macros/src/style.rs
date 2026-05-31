use crate::css::extract_selectors;
use heck::ToSnakeCase;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::Targets;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse2, token, Ident, LitStr, Token};

pub struct StyleOutput {
    pub bindings: TokenStream,
    pub css: String,
}

// AST for our style! macro
pub(crate) struct StyleInput {
    items: Vec<StyleItem>,
}

enum StyleItem {
    Rule(StyleRule),
    AtRule(AtRule),
}

struct AtRule {
    name: String,
    content: String,
}

struct StyleRule {
    selectors: TokenStream,
    block: StyleBlock,
}

struct StyleBlock {
    properties: Vec<StyleProperty>,
}

struct StyleProperty {
    name: String,
    value: String,
    #[allow(dead_code)]
    span: proc_macro2::Span,
    /// If true, the CSS output should include the quotes around the value.
    /// Used for properties where CSS syntax requires quoted strings (e.g., content).
    preserve_quotes: bool,
}

/// CSS properties whose values REQUIRE quotes in valid CSS output.
/// The `style!` macro strips quotes from all values by default, but these
/// properties produce invalid CSS without them.
const CSS_QUOTE_REQUIRED_PROPERTIES: &[&str] = &[
    "content",
    "font-family",
    "string-set",
];

impl Parse for StyleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            // Check for @ rules
            if input.peek(Token![@]) {
                let at_rule = input.parse::<AtRule>()?;
                items.push(StyleItem::AtRule(at_rule));
            } else {
                let rule = input.parse::<StyleRule>()?;
                items.push(StyleItem::Rule(rule));
            }
        }
        Ok(StyleInput { items })
    }
}

impl Parse for AtRule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![@]>()?;
        let name = input.parse::<Ident>()?.to_string();

        // Collect tokens to form the content
        let mut tokens = TokenStream::new();
        let mut depth = 0;
        let mut _found_opening_brace = false;

        while !input.is_empty() {
            // Peek to check structure without consuming yet
            let fork = input.fork();
            let tt: TokenTree = fork.parse()?;
            let token_str = tt.to_string();

            // Handle braces for nested structures
            if token_str == "{" {
                depth += 1;
                _found_opening_brace = true;
            } else if token_str == "}" {
                depth -= 1;
            }

            // Consume the token
            let tt: TokenTree = input.parse()?;
            tokens.extend(std::iter::once(tt));

            // Check termination conditions AFTER consuming
            if depth == 0 {
                if _found_opening_brace {
                    // We just closed the main block of the AtRule (e.g. @media { ... })
                    // Stop here so we don't swallow subsequent rules!
                    break;
                }

                // If it's a statement rule (like @import), it ends with semicolon
                if input.peek(Token![;]) {
                    // We don't consume the semicolon here (it's handled by caller loop logic usually?
                    // Wait, caller loop in StyleInput::parse invokes AtRule::parse.
                    // If AtRule::parse consumes everything including semicolon, that's fine.
                    // The original code had: "Consume trailing semicolon if present" at end.
                    // So we should break here, and let the end logic handle it?
                    // Original logic: "Stop when we've closed all braces and hit a semicolon"
                    // If we peek semicolon, we break loop.
                    break;
                }
            }
        }

        // Consume trailing semicolon if present (for statement rules)
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }

        // Convert collected tokens to CSS string using the helper to ensure proper spacing
        let content = tokens_to_css_string(&tokens);

        Ok(AtRule { name, content })
    }
}

impl Parse for StyleRule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse selectors until we see a brace
        let mut selector_tokens = TokenStream::new();
        while !input.peek(token::Brace) && !input.is_empty() {
            selector_tokens.extend(std::iter::once(input.parse::<TokenTree>()?));
        }

        // Validate selectors for kebab-case classes
        validate_selectors(&selector_tokens)?;

        if input.is_empty() {
            return Err(input.error("Expected block after selectors"));
        }

        let content;
        braced!(content in input);
        let block = content.parse()?;

        Ok(StyleRule {
            selectors: selector_tokens,
            block,
        })
    }
}

fn validate_selectors(tokens: &TokenStream) -> syn::Result<()> {
    use std::collections::HashSet;

    let mut seen_ids = HashSet::new();

    // Re-implementing with peekable
    let mut iter = tokens.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        if let TokenTree::Punct(p) = &tt {
            // Check for class selectors
            if p.as_char() == '.' {
                // Class start - consume the identifier
                if let Some(TokenTree::Ident(_)) = iter.peek() {
                    let _ = iter.next(); // consume ident
                                         // Check for dashes - explicitly forbidden
                    if let Some(TokenTree::Punct(next_p)) = iter.peek() {
                        if next_p.as_char() == '-' {
                            return Err(syn::Error::new(
                                next_p.span(),
                                "Dashes are not allowed in CSS classes in Azumi. Use snake_case (e.g. .my_class).",
                            ));
                        }
                    }
                }
            }
            // Check for ID selectors
            else if p.as_char() == '#' {
                // ID start
                if let Some(TokenTree::Ident(ident)) = iter.peek() {
                    let ident_span = ident.span();
                    let id_name = ident.to_string();
                    let _ = iter.next(); // consume ident

                    // Check for dashes - explicitly forbidden
                    if let Some(TokenTree::Punct(next_p)) = iter.peek() {
                        if next_p.as_char() == '-' {
                            return Err(syn::Error::new(
                                next_p.span(),
                                "Dashes are not allowed in CSS IDs in Azumi. Use snake_case (e.g. #my_id).",
                            ));
                        }
                    }

                    // Check for duplicate IDs
                    if seen_ids.contains(&id_name) {
                        return Err(syn::Error::new(
                            ident_span,
                            format!(
                                "Duplicate ID '{}' in CSS. IDs must be unique within a component.",
                                id_name
                            ),
                        ));
                    }
                    seen_ids.insert(id_name);
                }
            }
        }
    }
    Ok(())
}

impl Parse for StyleBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while !input.is_empty() {
            properties.push(input.parse()?);
        }
        Ok(StyleBlock { properties })
    }
}

impl Parse for StyleProperty {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse property name (kebab-case identifier)
        // syn::Ident doesn't support dashes, so we might get multiple tokens
        // e.g. background - color
        let mut name = String::new();
        let start_span = input.span();

        loop {
            if input.peek(Token![:]) {
                break;
            }
            if input.is_empty() {
                return Err(input.error("Expected ':' after property name"));
            }
            let tt: TokenTree = input.parse()?;
            name.push_str(&tt.to_string());
        }

        // Validate property name
        if !is_valid_css_property(&name) {
            return Err(syn::Error::new(
                start_span,
                format!("Unknown CSS property: '{}'", name),
            ));
        }

        input.parse::<Token![:]>()?;

        // Parse value - MUST be a double-quoted string literal
        let value_start_span = input.span();

        // Require double-quoted string literals for CSS values
        // This prevents lexer issues with values like "2em", "#e0e0e0", "rgba(...)"
        let lit_str: LitStr = input.parse().map_err(|_| {
            syn::Error::new(
                value_start_span,
                "CSS values must be double-quoted strings.\n\
                 Example: padding: \"1rem\";\n\
                 \n\
                 Unquoted values like `padding: 1rem;` can cause lexer issues\n\
                 with certain CSS values (e.g., #colors, 2em units).",
            )
        })?;
        let value = lit_str.value();

        input.parse::<Token![;]>()?;

        // Validate the CSS value using lightningcss (skip for CSS variables)
        if !value.starts_with("var(") {
            if let Err(err_msg) = validate_css_value(&name, &value) {
                return Err(syn::Error::new(
                    value_start_span,
                    format!("Invalid CSS value for property '{}': {}", name, err_msg),
                ));
            }
        }

        // Determine if quotes should be preserved in CSS output.
        // Properties whose CSS syntax REQUIRES string values (e.g., content: "text")
        // need the quotes in the output. Other properties (padding, color, etc.)
        // should have quotes stripped.
        let preserve_quotes = CSS_QUOTE_REQUIRED_PROPERTIES.contains(&name.as_str());

        Ok(StyleProperty {
            name,
            value,
            span: start_span,
            preserve_quotes,
        })
    }
}

/// Validate CSS property value using custom rules + lightningcss parser
fn validate_css_value(property: &str, value: &str) -> Result<(), String> {
    // Step 1: Strict custom validation for common errors

    // Check for spaces in single-word values (common typo)
    let trimmed = value.trim();

    if !is_multi_word_property(property) {
        // Properties that should be single tokens (no spaces)
        if trimmed.contains(' ') && !is_valid_space_in_value(property, trimmed) {
            return Err(format!(
                "Unexpected space in value '{}'. Did you mean '{}'?",
                value,
                trimmed.replace(' ', "")
            ));
        }
    }

    // Check for invalid units
    if let Some(err) = validate_units(trimmed) {
        return Err(err);
    }

    // Check for malformed hex colors
    if trimmed.starts_with('#') {
        if let Some(err) = validate_hex_color(trimmed) {
            return Err(err);
        }
    }

    // Step 2: Use lightningcss for full syntax validation
    let css = format!(".test {{ {}: {}; }}", property, value);
    let parse_options = ParserOptions::default();

    let result = StyleSheet::parse(&css, parse_options);
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("Unexpected token") || error_msg.contains("UnexpectedToken") {
                Err(format!("Unexpected token in value '{}'", value))
            } else if error_msg.contains("InvalidValue") {
                Err(format!("'{}' is not a valid value", value))
            } else {
                Err(format!("Parse error: {}", error_msg))
            }
        }
    }
}

/// Properties that accept multiple space-separated values
fn is_multi_word_property(property: &str) -> bool {
    // Check for prefixes
    if property.starts_with("border-")
        || property.starts_with("background-")
        || property.starts_with("margin-")
        || property.starts_with("padding-")
        || property.starts_with("font-")
        || property.starts_with("text-")
        || property.starts_with("grid-")
        || property.starts_with("flex-")
        || property.starts_with("animation-")
        || property.starts_with("transition-")
        || property.starts_with("transform-")
        || property.starts_with("list-style-")
        || property.starts_with("outline-")
    {
        return true;
    }

    matches!(
        property,
        "margin"
            | "padding"
            | "border"
            | "border-radius"
            | "background"
            | "box-shadow"
            | "transform"
            | "transition"
            | "animation"
            | "font"
            | "font-family"
            | "text-shadow"
            | "flex"
            | "grid-template-columns"
            | "grid-template-rows"
            | "grid-gap"
            | "gap"
            | "content"
            | "cursor" // cursor can be "pointer", but also "url(...) x y, auto"
            | "filter"
            | "backdrop-filter"
            | "clip-path"
    )
}

/// Check if spaces are valid in this specific value context
fn is_valid_space_in_value(property: &str, value: &str) -> bool {
    // Allow spaces in certain contexts:
    // - Multiple values: "10px 20px"
    // - Functions: "rgb(255, 0, 0)", "calc(100% - 20px)"
    // - Keywords with spaces: "ease-in-out"

    value.contains('(') || // Function call
    value.split_whitespace().count() > 1 && is_multi_word_property(property)
}

/// Validate unit suffixes
fn validate_units(value: &str) -> Option<String> {
    // Extract potential unit from value like "10px", "2em", etc.
    let value_lower = value.to_lowercase();

    // Common typos in units - use word boundary matching to avoid false positives
    // e.g., "pz" should not match "spaz" or "pizza"
    let typo_map = [
        ("pz", "px", "p z"),    // Also check for "p z" (space)
        ("pxs", "px", "p x s"), // Also check for spaced version
        ("e m", "em", "e  m"),  // space between e and m
        ("r em", "rem", "r  em"),
        ("p t", "pt", "p  t"),
        ("p c", "pc", "p  c"),
    ];

    for (typo, correct, spaced_typo) in &typo_map {
        // Check for word boundary before and after the typo
        if value_lower == *typo
            || value_lower.ends_with(typo)
                && value_lower.len() > typo.len()
                && !value_lower
                    .chars()
                    .nth(value_lower.len() - typo.len() - 1)
                    .is_some_and(|c| c.is_alphanumeric())
            || value_lower.starts_with(typo)
                && value_lower.len() > typo.len()
                && !value_lower
                    .chars()
                    .nth(typo.len())
                    .is_some_and(|c| c.is_alphanumeric())
            || value_lower.contains(spaced_typo)
        {
            return Some(format!(
                "Invalid unit '{}'. Did you mean '{}'?",
                typo, correct
            ));
        }
    }

    None
}

/// Validate hex color format
fn validate_hex_color(value: &str) -> Option<String> {
    if !value.starts_with('#') {
        return None;
    }

    let hex_part = &value[1..];

    // Valid lengths: 3, 4, 6, 8
    if !matches!(hex_part.len(), 3 | 4 | 6 | 8) {
        return Some(format!(
            "Invalid hex color length: '{}'. Expected 3, 4, 6, or 8 characters after #",
            value
        ));
    }

    // Check for non-hex characters
    for ch in hex_part.chars() {
        if !ch.is_ascii_hexdigit() {
            return Some(format!(
                "Invalid hex color: '{}' contains non-hex character '{}'",
                value, ch
            ));
        }
    }

    None
}

/// Generate bindings for classes and IDs extracted from CSS.
/// When `skip_dashed` is true, dashed class/ID names are skipped (for process_style_macro).
/// When `skip_dashed` is false, all names are included (for process_global_style_macro).
fn generate_bindings(classes: Vec<String>, ids: Vec<String>, skip_dashed: bool) -> TokenStream {
    let mut bindings = TokenStream::new();
    let mut skipped_dashed_classes: Vec<String> = Vec::new();

    for class in classes {
        if class.contains('-') && skip_dashed {
            skipped_dashed_classes.push(class);
            continue;
        }
        let snake_name = class.to_snake_case();
        let ident = format_ident!("{}", snake_name);

        bindings.extend(quote! {
            let #ident = #class;
        });
    }

    if skip_dashed && !skipped_dashed_classes.is_empty() {
        let class_list: Vec<String> = skipped_dashed_classes
            .iter()
            .map(|s| format!("'.{}'", s))
            .collect();
        eprintln!(
            "WARNING: Dashed CSS classes cannot be used as Rust bindings: {} \
             These must use class={{\"class-name\"}} syntax, not class={{dashed_name}}.",
            class_list.join(", ")
        );
    }

    for id in ids {
        if id.contains('-') && skip_dashed {
            continue;
        }
        let ident = format_ident!("{}", id);

        bindings.extend(quote! {
            let #ident = #id;
        });
    }

    bindings
}

/// Process global style macro - validates but doesn't scope or generate bindings
pub fn process_global_style_macro(input: TokenStream) -> StyleOutput {
    let style_input: StyleInput = match parse2(input.clone()) {
        Ok(input) => input,
        Err(_) => {
            let raw_css = tokens_to_css_string(&input);
            let (classes, ids) = extract_selectors(&raw_css);
            let bindings = generate_bindings(classes.into_iter().collect(), ids.into_iter().collect(), false);
            return StyleOutput {
                bindings,
                css: minify_css(&raw_css),
            };
        }
    };

    let raw_css = reconstruct_css_from_parsed(&style_input);
    let (classes, ids) = extract_selectors(&raw_css);
    let bindings = generate_bindings(classes.into_iter().collect(), ids.into_iter().collect(), false);

    StyleOutput {
        bindings,
        css: minify_css(&raw_css),
    }
}

pub fn process_style_macro(input: TokenStream) -> StyleOutput {
    let style_input: StyleInput = match parse2(input.clone()) {
        Ok(input) => input,
        Err(_) => {
            let raw_css = tokens_to_css_string(&input);
            let (classes, ids) = extract_selectors(&raw_css);
            let bindings = generate_bindings(classes.into_iter().collect(), ids.into_iter().collect(), true);
            return StyleOutput {
                bindings,
                css: minify_css(&raw_css),
            };
        }
    };

    let raw_css = reconstruct_css_from_parsed(&style_input);
    let (classes, ids) = extract_selectors(&raw_css);
    let bindings = generate_bindings(classes.into_iter().collect(), ids.into_iter().collect(), true);

    StyleOutput {
        bindings,
        css: minify_css(&raw_css),
    }
}

/// Reconstruct CSS string from already-parsed AST (no re-parsing)
pub fn reconstruct_css_from_parsed(style_input: &StyleInput) -> String {
    let mut raw_css = String::new();

    for item in &style_input.items {
        match item {
            StyleItem::AtRule(at_rule) => {
                raw_css.push('@');
                raw_css.push_str(&at_rule.name);
                raw_css.push(' ');
                raw_css.push_str(&at_rule.content);
                if !at_rule.content.trim().ends_with('}') {
                    raw_css.push(';');
                }
                raw_css.push(' ');
            }
            StyleItem::Rule(rule) => {
                let selector_str = tokens_to_css_string(&rule.selectors);

                raw_css.push_str(&selector_str);
                raw_css.push_str(" { ");
                for prop in &rule.block.properties {
                    if prop.preserve_quotes {
                        // Re-add quotes for properties that require them in CSS
                        raw_css.push_str(&format!("{}: \"{}\"; ", prop.name, prop.value));
                    } else {
                        raw_css.push_str(&format!("{}: {}; ", prop.name, prop.value));
                    }
                }
                raw_css.push_str("} ");
            }
        }
    }

    raw_css
}

pub(crate) fn tokens_to_css_string(tokens: &TokenStream) -> String {
    let mut css = String::new();
    let mut last_char_was_hyphen = false;
    let mut last_char_was_dot_or_hash_or_colon = false;
    let mut last_was_open_paren = false;
    let mut last_was_at_sign = false;

    for tt in tokens.clone() {
        match tt {
            TokenTree::Ident(ident) => {
                // Add space if previous wasn't a special char that expects attachment
                if !css.is_empty()
                    && !last_char_was_hyphen
                    && !last_char_was_dot_or_hash_or_colon
                    && !last_was_open_paren
                    && !last_was_at_sign
                {
                    css.push(' ');
                }
                css.push_str(&ident.to_string());
                last_char_was_hyphen = false;
                last_char_was_dot_or_hash_or_colon = false;
                last_was_open_paren = false;
                last_was_at_sign = false;
            }
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                // Reset at_sign unless this is it (unlikely to have @@)
                last_was_at_sign = false;

                if ch == '-' {
                    css.push(ch);
                    last_char_was_hyphen = true;
                    last_char_was_dot_or_hash_or_colon = false;
                    last_was_open_paren = false;
                } else if ch == '.' || ch == '#' || ch == ':' {
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash_or_colon = true;
                    last_was_open_paren = false;
                } else if ch == '(' {
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash_or_colon = false;
                    last_was_open_paren = true;
                } else if ch == '@' {
                    if !css.is_empty() {
                        css.push(' ');
                    }
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash_or_colon = false;
                    last_was_open_paren = false;
                    last_was_at_sign = true;
                } else {
                    // Other puncts (>, +, ;, etc)
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash_or_colon = false;
                    last_was_open_paren = false;
                }
            }
            TokenTree::Literal(lit) => {
                if !css.is_empty()
                    && !last_char_was_dot_or_hash_or_colon
                    && !last_was_open_paren
                    && !last_was_at_sign
                {
                    css.push(' ');
                }

                let s = lit.to_string();
                // Strip outer quotes logic
                let trimmed = s.trim();
                let is_quoted = trimmed.len() >= 2
                    && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
                        || (trimmed.starts_with('\'') && trimmed.ends_with('\'')));

                if is_quoted {
                    css.push_str(&trimmed[1..trimmed.len() - 1]);
                } else {
                    css.push_str(trimmed);
                }

                last_char_was_hyphen = false;
                last_char_was_dot_or_hash_or_colon = false;
                last_was_open_paren = false;
                last_was_at_sign = false;
            }
            TokenTree::Group(group) => {
                if !css.is_empty()
                    && !last_char_was_dot_or_hash_or_colon
                    && !last_was_open_paren
                    && !last_was_at_sign
                {
                    css.push(' ');
                }
                // Group logic
                let delimiter_pair = match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => ("(", ")"),
                    proc_macro2::Delimiter::Brace => ("{", "}"),
                    proc_macro2::Delimiter::Bracket => ("[", "]"),
                    proc_macro2::Delimiter::None => ("", ""),
                };

                css.push_str(delimiter_pair.0);
                css.push_str(&tokens_to_css_string(&group.stream()));
                css.push_str(delimiter_pair.1);

                last_char_was_hyphen = false;
                last_char_was_dot_or_hash_or_colon = false;
                last_was_open_paren = false;
                last_was_at_sign = false;
            }
        }
    }
    css
}

fn is_valid_css_property(name: &str) -> bool {
    if name.starts_with("--") {
        return true;
    }
    static VALID_PROPERTIES: std::sync::LazyLock<Vec<&'static str>> =
        std::sync::LazyLock::new(|| {
            let mut props = vec![
                "align-content",
                "align-items",
                "align-self",
                "all",
                "appearance",
                "animation",
                "animation-delay",
                "animation-direction",
                "animation-duration",
                "animation-fill-mode",
                "animation-iteration-count",
                "animation-name",
                "animation-play-state",
                "animation-timing-function",
                "backdrop-filter",
                "backface-visibility",
                "background",
                "background-attachment",
                "background-blend-mode",
                "background-clip",
                "background-color",
                "background-image",
                "background-origin",
                "background-position",
                "background-repeat",
                "background-size",
                "border",
                "border-bottom",
                "border-bottom-color",
                "border-bottom-left-radius",
                "border-bottom-right-radius",
                "border-bottom-style",
                "border-bottom-width",
                "border-collapse",
                "border-color",
                "border-image",
                "border-image-outset",
                "border-image-repeat",
                "border-image-slice",
                "border-image-source",
                "border-image-width",
                "border-left",
                "border-left-color",
                "border-left-style",
                "border-left-width",
                "border-radius",
                "border-right",
                "border-right-color",
                "border-right-style",
                "border-right-width",
                "border-spacing",
                "border-style",
                "border-top",
                "border-top-color",
                "border-top-left-radius",
                "border-top-right-radius",
                "border-top-style",
                "border-top-width",
                "border-width",
                "bottom",
                "box-decoration-break",
                "box-shadow",
                "box-sizing",
                "break-after",
                "break-before",
                "break-inside",
                "caption-side",
                "caret-color",
                "clear",
                "clip",
                "clip-path",
                "color",
                "column-count",
                "column-fill",
                "column-gap",
                "column-rule",
                "column-rule-color",
                "column-rule-style",
                "column-rule-width",
                "column-span",
                "column-width",
                "columns",
                "content",
                "counter-increment",
                "counter-reset",
                "cursor",
                "direction",
                "display",
                "empty-cells",
                "filter",
                "flex",
                "flex-basis",
                "flex-direction",
                "flex-flow",
                "flex-grow",
                "flex-shrink",
                "flex-wrap",
                "float",
                "font",
                "font-family",
                "font-feature-settings",
                "font-kerning",
                "font-language-override",
                "font-size",
                "font-size-adjust",
                "font-stretch",
                "font-style",
                "font-synthesis",
                "font-variant",
                "font-variant-alternates",
                "font-variant-caps",
                "font-variant-east-asian",
                "font-variant-ligatures",
                "font-variant-numeric",
                "font-variant-position",
                "font-weight",
                "gap",
                "grid",
                "grid-area",
                "grid-auto-columns",
                "grid-auto-flow",
                "grid-auto-rows",
                "grid-column",
                "grid-column-end",
                "grid-column-gap",
                "grid-column-start",
                "grid-gap",
                "grid-row",
                "grid-row-end",
                "grid-row-gap",
                "grid-row-start",
                "grid-template",
                "grid-template-areas",
                "grid-template-columns",
                "grid-template-rows",
                "hanging-punctuation",
                "height",
                "hyphens",
                "image-rendering",
                "isolation",
                "justify-content",
                "justify-items",
                "justify-self",
                "left",
                "letter-spacing",
                "line-break",
                "line-height",
                "list-style",
                "list-style-image",
                "list-style-position",
                "list-style-type",
                "margin",
                "margin-bottom",
                "margin-left",
                "margin-right",
                "margin-top",
                "max-height",
                "max-width",
                "min-height",
                "min-width",
                "mix-blend-mode",
                "object-fit",
                "object-position",
                "opacity",
                "order",
                "orphans",
                "outline",
                "outline-color",
                "outline-offset",
                "outline-style",
                "outline-width",
                "overflow",
                "overflow-wrap",
                "overflow-x",
                "overflow-y",
                "padding",
                "padding-bottom",
                "padding-left",
                "padding-right",
                "padding-top",
                "page-break-after",
                "page-break-before",
                "page-break-inside",
                "perspective",
                "perspective-origin",
                "pointer-events",
                "position",
                "quotes",
                "resize",
                "right",
                "row-gap",
                "scroll-behavior",
                "tab-size",
                "table-layout",
                "text-align",
                "text-align-last",
                "text-combine-upright",
                "text-decoration",
                "text-decoration-color",
                "text-decoration-line",
                "text-decoration-style",
                "text-indent",
                "text-justify",
                "text-orientation",
                "text-overflow",
                "text-shadow",
                "text-transform",
                "text-underline-position",
                "top",
                "transform",
                "transform-origin",
                "transform-style",
                "transition",
                "transition-delay",
                "transition-duration",
                "transition-property",
                "transition-timing-function",
                "unicode-bidi",
                "user-select",
                "vertical-align",
                "visibility",
                "white-space",
                "widows",
                "width",
                "word-break",
                "word-spacing",
                "word-wrap",
                "writing-mode",
                "z-index",
                "-webkit-backdrop-filter",
                "-webkit-appearance",
                "-webkit-background-clip",
                "-webkit-font-smoothing",
                "-webkit-overflow-scrolling",
                "-webkit-text-fill-color",
                "-moz-osx-font-smoothing",
            ];
            props.sort();
            props
        });

    VALID_PROPERTIES.binary_search(&name).is_ok()
}

/// Minify CSS using lightningcss
fn minify_css(css: &str) -> String {
    let parse_options = ParserOptions::default();
    if let Ok(stylesheet) = StyleSheet::parse(css, parse_options) {
        let print_options = PrinterOptions {
            minify: true,
            targets: Targets::default(),
            ..PrinterOptions::default()
        };
        if let Ok(minified) = stylesheet.to_css(print_options) {
            let result = restore_pseudo_elements(&minified.code);
            return requote_property_values(&result);
        }
    }

    css.trim().to_string()
}

/// Re-add quotes to CSS property values that require them in valid CSS.
///
/// lightningcss strips quotes during minification, but properties like
/// `content`, `font-family`, and `string-set` require quoted string values.
/// This post-processes the minified CSS to re-add quotes where needed.
fn requote_property_values(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut remaining = css;

    for &prop in CSS_QUOTE_REQUIRED_PROPERTIES {
        while let Some(pos) = remaining.find(&format!("{}:", prop)) {
            // Copy everything before this property
            result.push_str(&remaining[..pos]);
            remaining = &remaining[pos..];

            // Find the property name end (the colon)
            let colon_pos = remaining.find(':').unwrap_or(0);
            let after_colon = &remaining[colon_pos + 1..];

            // Check if the value is already quoted
            let trimmed = after_colon.trim_start();
            if trimmed.starts_with('"') || trimmed.starts_with('\'') {
                // Already quoted, skip
                result.push_str(&remaining[..colon_pos + 1]);
                remaining = &remaining[colon_pos + 1..];
                continue;
            }

            // Find the value end (semicolon or closing brace)
            let value_end = after_colon.find(';')
                .or_else(|| after_colon.find('}'))
                .unwrap_or(after_colon.len());
            let value = after_colon[..value_end].trim();

            // Skip empty values, keywords, and function calls
            if value.is_empty()
                || value.starts_with("var(")
                || value.contains('(')
                || value == "none"
                || value == "normal"
                || value.starts_with("attr(")
                || value.starts_with("counter(")
            {
                result.push_str(&remaining[..colon_pos + 1]);
                remaining = &remaining[colon_pos + 1..];
                continue;
            }

            // Re-add quotes
            let prop_end = colon_pos + 1;
            result.push_str(&remaining[..prop_end]);
            result.push_str(&format!(" \"{}\"", value));
            // Skip past the old unquoted value and the delimiter
            let skip = prop_end + after_colon[..value_end].len();
            remaining = &remaining[skip..];
        }
    }

    result.push_str(remaining);
    result
}

fn restore_pseudo_elements(css: &str) -> String {
    const CSS2_PSEUDO_ELEMENTS: &[(&str, &str)] = &[
        (":before", "::before"),
        (":after", "::after"),
        (":first-line", "::first-line"),
        (":first-letter", "::first-letter"),
    ];
    let mut result = css.to_string();
    for (single, double) in CSS2_PSEUDO_ELEMENTS {
        let mut start = 0;
        while let Some(pos) = result[start..].find(single) {
            let abs_pos = start + pos;
            let preceded_by_colon = abs_pos > 0 && result.as_bytes()[abs_pos - 1] == b':';
            if !preceded_by_colon {
                result = format!("{}{}{}", &result[..abs_pos], double, &result[abs_pos + single.len()..]);
                start = abs_pos + double.len();
            } else {
                start = abs_pos + 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_css_property_valid() {
        let valid = vec![
            "display",
            "color",
            "background-color",
            "border-radius",
            "font-size",
            "margin-top",
            "padding-left",
            "z-index",
            "flex-grow",
        ];
        for prop in valid {
            assert!(
                is_valid_css_property(prop),
                "Expected '{}' to be valid",
                prop
            );
        }
    }

    #[test]
    fn test_is_valid_css_property_custom() {
        assert!(is_valid_css_property("--custom-prop"));
        assert!(is_valid_css_property("--my-var"));
        assert!(is_valid_css_property("--theme-color"));
    }

    #[test]
    fn test_is_valid_css_property_invalid() {
        let invalid = vec![
            "notaproperty",
            "fancy-style",
            "my-property",
            "foo-bar-baz",
        ];
        for prop in invalid {
            assert!(
                !is_valid_css_property(prop),
                "Expected '{}' to be invalid",
                prop
            );
        }
    }

    #[test]
    fn test_is_valid_css_property_vendor_prefix() {
        assert!(is_valid_css_property("-webkit-appearance"));
        assert!(is_valid_css_property("-moz-osx-font-smoothing"));
        assert!(is_valid_css_property("-webkit-backdrop-filter"));
    }

    #[test]
    fn test_tokens_to_css_string_simple() {
        use quote::quote;
        let input = quote! { display: none };
        let output = tokens_to_css_string(&input);
        assert!(output.contains("display"));
        assert!(output.contains("none"));
    }

    #[test]
    fn test_tokens_to_css_string_with_hex_color() {
        use quote::quote;
        let input = quote! { color: #123456 };
        let output = tokens_to_css_string(&input);
        assert!(output.contains("color"));
        assert!(output.contains("#123456"));
    }

    #[test]
    fn test_tokens_to_css_string_with_function() {
        use quote::quote;
        let input = quote! { transform: rotate(45deg) };
        let output = tokens_to_css_string(&input);
        assert!(output.contains("transform"));
        assert!(output.contains("rotate"));
    }

    #[test]
    fn test_tokens_to_css_string_multiple_declarations() {
        use quote::quote;
        let input = quote! { color: red; font-size: 14px };
        let output = tokens_to_css_string(&input);
        assert!(output.contains("color"));
        assert!(output.contains("red"));
        assert!(output.contains("font-size"));
        assert!(output.contains("14px"));
    }

    #[test]
    fn test_media_query_property_stripping() {
        let input = quote! {
            @media (max-width: 768px) {
                .center_zone {
                    display: "none !important";
                }
            }
        };

        let output = process_global_style_macro(input);

        // We expect: display: none !important;
        // Current bug produces: display: "none !important";
        println!("Generated CSS: {}", output.css);
        assert!(
            !output.css.contains(r#""none !important""#),
            "CSS contains quoted value: {}",
            output.css
        );
    }
    #[test]
    fn test_media_query_spacing() {
        let input = quote! {
            @media (max-width: 768px) {}
        };
        let output = process_global_style_macro(input);
        // Should be "@media (max-width: 768px) {}" NOT "@ media ..."
        println!("CSS: {}", output.css);
        assert!(
            output.css.contains("@media"),
            "CSS contains broken @ media: {}",
            output.css
        );
        assert!(
            !output.css.contains("@ media"),
            "CSS contains broken @ media: {}",
            output.css
        );
    }

    #[test]
    fn test_tokens_to_css_string_pseudo_element() {
        use quote::quote;
        let input = quote! { .tooltip::before { content: "→" } };
        let output = tokens_to_css_string(&input);
        assert!(
            output.contains("::before"),
            "Pseudo-element ::before should be preserved, got: {}",
            output
        );
    }

    #[test]
    fn test_minify_css_preserves_double_colon() {
        let css = ".tooltip::before { content: \"→\" }";
        let minified = minify_css(css);
        assert!(
            minified.contains("::before"),
            "minify_css should preserve ::before pseudo-element, got: {}",
            minified
        );
    }

    #[test]
    fn test_restore_pseudo_elements_basic() {
        let css = ".card:before{color:red}.card:after{color:blue}";
        let restored = restore_pseudo_elements(css);
        assert_eq!(restored, ".card::before{color:red}.card::after{color:blue}");
    }

    #[test]
    fn test_restore_pseudo_elements_preserves_already_double() {
        let css = ".card::before{color:red}";
        let restored = restore_pseudo_elements(css);
        assert_eq!(restored, ".card::before{color:red}");
    }

    #[test]
    fn test_process_style_macro_pseudo_element() {
        let input: TokenStream = ".tooltip::before { content: \"→\" }".parse().unwrap();
        let output = process_style_macro(input);
        assert!(
            output.css.contains("::before"),
            "Pseudo-element ::before should survive process_style_macro, got: {}",
            output.css
        );
    }

    #[test]
    fn test_tokens_to_css_string_pseudo_class() {
        use quote::quote;
        let input = quote! { .card:hover { color: blue } };
        let output = tokens_to_css_string(&input);
        assert!(
            output.contains(":hover"),
            "Pseudo-class :hover should be preserved, got: {}",
            output
        );
    }

    #[test]
    fn test_content_property_preserves_quotes() {
        let input: TokenStream = ".tooltip::before { content: \"→\" }".parse().unwrap();
        let output = process_style_macro(input);
        // content property requires quotes in valid CSS
        assert!(
            output.css.contains("content: \"→\""),
            "content property should preserve quotes in CSS output, got: {}",
            output.css
        );
    }

    #[test]
    fn test_regular_property_strips_quotes() {
        let input: TokenStream = ".card { padding: \"1rem\" }".parse().unwrap();
        let output = process_style_macro(input);
        // Regular properties should NOT have quotes in CSS output
        assert!(
            output.css.contains("padding:1rem") || output.css.contains("padding: 1rem"),
            "Regular properties should have quotes stripped, got: {}",
            output.css
        );
        assert!(
            !output.css.contains("padding:\"1rem\"") && !output.css.contains("padding: \"1rem\""),
            "Regular properties should not retain quotes, got: {}",
            output.css
        );
    }

    #[test]
    fn test_font_family_preserves_quotes() {
        let input: TokenStream = ".text { font-family: \"Arial\" }".parse().unwrap();
        let output = process_style_macro(input);
        assert!(
            output.css.contains("font-family: \"Arial\""),
            "font-family should preserve quotes, got: {}",
            output.css
        );
    }
}
