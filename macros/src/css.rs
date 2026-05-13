use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

/// Transform CSS selectors to include scope attribute
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut iter = css.chars().peekable();
    scope_css_recursive(&mut iter, &scope_attr)
}

fn scope_css_recursive(iter: &mut Peekable<Chars>, scope_attr: &str) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        // Skip strings (single and double quoted) - needed to handle CSS like content: "}"
        if ch == '"' || ch == '\'' {
            buffer.push(ch);
            let quote = ch;
            for c in iter.by_ref() {
                buffer.push(c);
                if c == quote {
                    break;
                }
            }
            continue;
        }
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();
                buffer.clear();

                // Check if this is a grouping rule (recurse) or style rule (scope)
                if is_grouping_rule(&selector_raw) {
                    result.push_str(&selector_raw);
                    result.push_str(" {");
                    // Recurse into the block
                    // We need to pass the iterator which is now inside the block
                    // We need to call scope_css_recursive until we hit '}' matched to this level?
                    // No, scope_css_recursive consumes until end of stream.
                    // But we want to consume only ONE block.
                    // Actually, we can just recurse. The recursive call will return when it finds a closing brace?
                    // We need to architect this so the recursive function processes a sequence of rules.
                    // It stops when it hits `}` (if it was called for a block) or EOF.

                    let inner_content = scope_css_level(iter, scope_attr, true); // true = stop at '}'
                    result.push_str(&inner_content);
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&selector_raw);
                    result.push_str(" {");
                    // Keyframes content (0% { ... }) should NOT be scoped
                    // Just copy balanced block
                    let content = extract_balanced_block(iter);
                    result.push_str(&content);
                    result.push('}');
                } else {
                    // Style Rule - Scope the selector
                    // But skip @font-face etc which are not grouping rules but also not style rules with selectors?
                    // @font-face { src: ... }
                    // scope_selector handles @ check.

                    // Split by comma for multiple selectors
                    let selectors: Vec<&str> = split_selector_list(&selector_raw);
                    let scoped: Vec<String> = selectors
                        .iter()
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| scope_selector(s.trim(), scope_attr))
                        .collect();

                    if !scoped.is_empty() {
                        result.push_str(&scoped.join(", "));
                    } else {
                        // e.g. @font-face
                        result.push_str(&selector_raw);
                    }

                    result.push_str(" {");
                    // Content is properties, just copy balanced block
                    let content = extract_balanced_block(iter);
                    result.push_str(&content);
                    result.push('}');
                }
            }
            '}' => {
                // Stray closing brace — preserve it in output to avoid corrupting CSS structure
                buffer.push('}');
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    // Append remaining buffer (whitespace etc)
    result.push_str(&buffer);
    result
}

// Helper that processes rules until it sees a closing brace (if finding_close=true) or EOF
fn scope_css_level(iter: &mut Peekable<Chars>, scope_attr: &str, finding_close: bool) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        // Skip strings (single and double quoted)
        if ch == '"' || ch == '\'' {
            buffer.push(ch);
            let quote = ch;
            for c in iter.by_ref() {
                buffer.push(c);
                if c == quote {
                    break;
                }
            }
            continue;
        }
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();

                if is_grouping_rule(&selector_raw) {
                    result.push_str(&buffer); // Keep original whitespace/selector
                    result.push('{');
                    buffer.clear();
                    // Recurse
                    result.push_str(&scope_css_level(iter, scope_attr, true));
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                } else {
                    // Scope selectors
                    // We need to preserve the whitespace in buffer before the selector?
                    // buffer contains the selector.
                    let scoped_selector_str = if selector_raw.starts_with('@') {
                        selector_raw.to_string()
                    } else {
                        let selectors: Vec<&str> = split_selector_list(&selector_raw);
                        selectors
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(|s| scope_selector(s.trim(), scope_attr))
                            .collect::<Vec<_>>()
                            .join(", ")
                    };

                    // We replace buffer content with scoped selector
                    // But try to keep formatting? Naive replacement is fine for minified CSS.
                    result.push_str(&scoped_selector_str);
                    result.push('{');
                    buffer.clear();

                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                }
            }
            '}' => {
                if finding_close {
                    // We found the closing brace for this level
                    // Return everything accumulated so far (excluding the })
                    // The caller will append '}'
                    result.push_str(&buffer);
                    return result;
                }
                // Stray } or logic error, just append
                buffer.push(ch);
            }
            ';' => {
                buffer.push(ch);
                result.push_str(&buffer);
                buffer.clear();
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    result.push_str(&buffer);
    result
}

fn is_grouping_rule(s: &str) -> bool {
    s.starts_with("@media")
        || s.starts_with("@supports")
        || s.starts_with("@layer")
        || s.starts_with("@container")
        || s.starts_with("@property")
        || s.starts_with("@font-face")
        || s.starts_with("@counter-style")
        || s.starts_with("@font-feature-values")
}

fn is_keyframes(s: &str) -> bool {
    s.starts_with("@keyframes") || s.starts_with("@-webkit-keyframes")
}

fn extract_balanced_block(iter: &mut Peekable<Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1; // We already passed the opening '{'

    for ch in iter.by_ref() {
        match ch {
            '{' => {
                depth += 1;
                content.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return content;
                }
                content.push(ch);
            }
            _ => content.push(ch),
        }
    }
    content
}

fn split_selector_list(selector_raw: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut paren_depth: usize = 0;
    let mut bracket_depth: usize = 0;
    let mut last_start = 0;
    for (byte_idx, ch) in selector_raw.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let sel = selector_raw[last_start..byte_idx].trim();
                if !sel.is_empty() {
                    result.push(sel);
                }
                last_start = byte_idx + ch.len_utf8();
            }
            _ => {}
        }
    }
    let last = selector_raw[last_start..].trim();
    if !last.is_empty() {
        result.push(last);
    }
    result
}

fn scope_selector(selector: &str, scope_attr: &str) -> String {
    if selector.starts_with('@') || selector.starts_with("/*") {
        return selector.to_string();
    }
    // Web Components shadow DOM selectors - do NOT scope these
    if selector.starts_with(":host")
        || selector.starts_with("::slotted")
        || selector.starts_with("::part")
    {
        return selector.to_string();
    }
    // Document-level pseudo-classes - do NOT scope these
    // These refer to the document root or document state, not individual components
    if selector == ":root" || selector == ":fullscreen" {
        return selector.to_string();
    }
    // Handle functional pseudo-classes like :is(), :where(), :not(), :has()
    // These may contain nested selectors that shouldn't be scoped (like :root inside)
    if let Some(paren_pos) = selector.find('(') {
        let before_paren = &selector[..paren_pos];
        let paren_content = extract_balanced_paren(selector, paren_pos);
        let after_paren_start = paren_pos + 1 + paren_content.len() + 1; // Skip "(", content, and ")"
        let after_paren = if after_paren_start < selector.len() {
            &selector[after_paren_start..]
        } else {
            ""
        };

        // Check if this is a functional pseudo-class (starts with :)
        if before_paren.starts_with(':') {
            // For :is(), :where(), :not(), :has() - process content but preserve document selectors
            let scoped_content = scope_selector_list_preserve_docs(&paren_content, scope_attr);
            return format!("{}({}){}", before_paren, scoped_content, after_paren);
        }
    }
    // Handle pseudo-elements (::before, ::after, etc.)
    // They must come AFTER pseudo-classes in the selector
    if let Some(pseudo_pos) = selector.find("::") {
        let base_and_pseudos = &selector[..pseudo_pos];
        let pseudo_element = &selector[pseudo_pos..];
        // Check if base has pseudo-classes (e.g., div:hover::before)
        if let Some(class_pos) = base_and_pseudos.rfind(':') {
            let base = &base_and_pseudos[..class_pos];
            let pseudo_classes = &base_and_pseudos[class_pos..];
            return format!("{}{}{}{}", base, pseudo_classes, scope_attr, pseudo_element);
        }
        // No pseudo-classes, just base + pseudo-element
        return format!("{}{}{}", base_and_pseudos, scope_attr, pseudo_element);
    }
    // Handle pseudo-classes only (no :: pseudo-element)
    // Use rfind to find the LAST colon (the pseudo-class colon)
    if let Some(pseudo_pos) = selector.rfind(':') {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }
    format!("{}{}", selector, scope_attr)
}

/// Extract balanced parentheses content starting after the opening paren
fn extract_balanced_paren(s: &str, start: usize) -> String {
    let mut result = String::new();
    let mut depth = 1; // Start at 1 since we're already inside the paren
    for ch in s[start + 1..].chars() {
        match ch {
            '(' => {
                depth += 1;
                result.push(ch);
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return result;
                }
                result.push(ch);
            }
            _ => result.push(ch),
        }
    }
    result
}

/// Scope a comma-separated selector list while preserving document-level selectors
fn scope_selector_list_preserve_docs(content: &str, scope_attr: &str) -> String {
    let selectors = split_selector_list(content);
    let scoped: Vec<String> = selectors
        .iter()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let trimmed = s.trim();
            // If this selector is or contains a document-level selector, preserve it unchanged
            if trimmed == ":root"
                || trimmed == ":fullscreen"
                || trimmed.starts_with(":host")
                || trimmed.starts_with("::slotted")
                || trimmed.starts_with("::part")
            {
                trimmed.to_string()
            } else {
                scope_selector(trimmed, scope_attr)
            }
        })
        .collect();
    scoped.join(", ")
}

/// Extract all defined class names and IDs from CSS content
pub fn extract_selectors(css: &str) -> (HashSet<String>, HashSet<String>) {
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    // Use recursive extractor
    let mut iter = css.chars().peekable();
    extract_selectors_recursive(&mut iter, &mut classes, &mut ids, false);
    (classes, ids)
}

fn extract_selectors_recursive(
    iter: &mut Peekable<Chars>,
    classes: &mut HashSet<String>,
    ids: &mut HashSet<String>,
    finding_close: bool,
) {
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();
                if is_grouping_rule(&selector_raw) {
                    buffer.clear();
                    extract_selectors_recursive(iter, classes, ids, true);
                } else if is_keyframes(&selector_raw) {
                    buffer.clear();
                    // Consume balanced block without extracting
                    let _ = extract_balanced_block(iter);
                } else {
                    // Extract from selectors
                    process_selectors(&selector_raw, classes, ids);
                    buffer.clear();
                    // Consume balanced block (properties)
                    let _ = extract_balanced_block(iter);
                }
            }
            '}' => {
                if finding_close {
                    return;
                }
                // Ignore stray
            }
            '/' => {
                // Skip comments
                if let Some(&'*') = iter.peek() {
                    iter.next();
                    while let Some(c) = iter.next() {
                        if c == '*' {
                            if let Some(&'/') = iter.peek() {
                                iter.next();
                                break;
                            }
                        }
                    }
                } else {
                    buffer.push(ch);
                }
            }
            '"' | '\'' => {
                // Skip strings
                let quote = ch;
                for c in iter.by_ref() {
                    if c == quote {
                        break;
                    }
                }
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
}

fn process_selectors(buffer: &str, classes: &mut HashSet<String>, ids: &mut HashSet<String>) {
    for selector in buffer.split(',') {
        let selector = selector.trim();
        if selector.is_empty() || selector.starts_with('@') || selector.starts_with("/*") {
            continue;
        }

        let mut chars = selector.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !name.is_empty() {
                    classes.insert(name);
                }
            } else if ch == '#' {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !name.is_empty() {
                    ids.insert(name);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_css_nested_media() {
        let css = "@media (max-width: 768px) { .center_zone { display: none !important; } }";
        let scope_id = "s123";
        let scoped = scope_css(css, scope_id);

        // Expected: @media (max-width: 768px) { .center_zone[data-s123] { display: none !important; } }
        assert!(
            scoped.contains(".center_zone[data-s123]"),
            "Actual: {}",
            scoped
        );
        assert!(scoped.contains("@media (max-width: 768px)"));
    }

    #[test]
    fn test_scope_css_nested_media_complex() {
        let css = "@media (min-width: 1024px) { .foo { color: red; } .bar { color: blue; } }";
        let scope_id = "xyz";
        let scoped = scope_css(css, scope_id);

        assert!(scoped.contains(".foo[data-xyz]"));
        assert!(scoped.contains(".bar[data-xyz]"));
    }

    #[test]
    fn test_scope_css_root_not_scoped() {
        let css = ":root { --color: red; } .foo { color: var(--color); }";
        let scope_id = "abc";
        let scoped = scope_css(css, scope_id);

        // :root should NOT be scoped - it always refers to document root
        assert!(
            scoped.contains(":root {"),
            ":root should remain unscoped, Actual: {}",
            scoped
        );
        // .foo should still be scoped
        assert!(
            scoped.contains(".foo[data-abc]"),
            ".foo should be scoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_scope_css_fullscreen_not_scoped() {
        let css = ":fullscreen { background: black; }";
        let scope_id = "xyz";
        let scoped = scope_css(css, scope_id);

        // :fullscreen should NOT be scoped
        assert!(
            scoped.contains(":fullscreen"),
            ":fullscreen should remain unscoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_scope_css_is_with_root() {
        let css = ":is(.foo, :root) { color: red; }";
        let scope_id = "abc";
        let scoped = scope_css(css, scope_id);

        // :root inside :is() should NOT be scoped
        assert!(
            scoped.contains(":is(.foo[data-abc], :root)"),
            ":root in :is() should remain unscoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_scope_css_where_with_root() {
        let css = ":where(.foo, :root) { color: blue; }";
        let scope_id = "xyz";
        let scoped = scope_css(css, scope_id);

        // :root inside :where() should NOT be scoped
        assert!(
            scoped.contains(":where(.foo[data-xyz], :root)"),
            ":root in :where() should remain unscoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_scope_css_not_with_root() {
        let css = ":not(:root) { color: green; }";
        let scope_id = "def";
        let scoped = scope_css(css, scope_id);

        // :root inside :not() should NOT be scoped
        assert!(
            scoped.contains(":not(:root)"),
            ":root in :not() should remain unscoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_scope_css_has_with_root() {
        let css = ":has(.child, :root) { color: yellow; }";
        let scope_id = "ghi";
        let scoped = scope_css(css, scope_id);

        // :root inside :has() should NOT be scoped
        assert!(
            scoped.contains(":has(.child[data-ghi], :root)"),
            ":root in :has() should remain unscoped, Actual: {}",
            scoped
        );
    }

    #[test]
    fn test_extract_selectors_nested() {
        let css = "@media screen { .foo { color: red; } } .bar { color: blue; }";
        let (classes, _ids) = extract_selectors(css);

        assert!(classes.contains("foo"));
        assert!(classes.contains("bar"));
    }
}
