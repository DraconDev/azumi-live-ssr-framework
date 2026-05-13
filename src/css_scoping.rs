use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

/// Compute a deterministic scope ID from source position (line, column).
/// Used by both the proc-macro and the hot reload watcher to guarantee
/// that scope IDs match at compile time and runtime.
#[must_use]
pub fn compute_scope_id(line: usize, col: usize) -> String {
    let mut hasher = FnvHasher::default();
    line.hash(&mut hasher);
    col.hash(&mut hasher);
    format!("s{:x}", hasher.finish())
}

/// Transform CSS selectors to include scope attribute
/// All CSS is automatically scoped - no escape hatches!
#[must_use]
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut iter = css.chars().peekable();
    scope_css_level(&mut iter, &scope_attr, false)
}

fn scope_css_level(
    iter: &mut std::iter::Peekable<std::str::Chars>,
    scope_attr: &str,
    finding_close: bool,
) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();

                if is_grouping_rule(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&scope_css_level(iter, scope_attr, true));
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                } else {
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

                    result.push_str(&scoped_selector_str);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                }
            }
            '}' => {
                if finding_close {
                    result.push_str(&buffer);
                    return result;
                }
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
}

fn is_keyframes(s: &str) -> bool {
    s.starts_with("@keyframes") || s.starts_with("@-webkit-keyframes")
}

fn extract_balanced_block(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1;
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
    if selector.starts_with(":host") || selector.starts_with("::slotted") || selector.starts_with("::part") {
        return selector.to_string();
    }
    
    fn find_last_real_colon(s: &str) -> Option<usize> {
        let mut bracket_depth = 0usize;
        let mut last_colon = None;
        
        for (i, ch) in s.char_indices() {
            match ch {
                '[' => bracket_depth = bracket_depth.saturating_add(1),
                ']' => bracket_depth = bracket_depth.saturating_sub(1),
                ':' if bracket_depth == 0 => { last_colon = Some(i); }
                _ => {}
            }
        }
        last_colon
    }
    
    if let Some(pseudo_pos) = selector.find("::") {
        let base_and_pseudos = &selector[..pseudo_pos];
        let pseudo_element = &selector[pseudo_pos..];
        if let Some(class_pos) = find_last_real_colon(base_and_pseudos) {
            let base = &base_and_pseudos[..class_pos];
            let pseudo_classes = &base_and_pseudos[class_pos..];
            return format!("{}{}{}{}", base, pseudo_classes, scope_attr, pseudo_element);
        }
        return format!("{}{}{}", base_and_pseudos, scope_attr, pseudo_element);
    }
    if let Some(pseudo_pos) = find_last_real_colon(selector) {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }
    format!("{}{}", selector, scope_attr)
}

#[cfg(test)]
mod tests;
