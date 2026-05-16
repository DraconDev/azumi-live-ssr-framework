/// Keywords after which a `/` starts a regex (not division).
const REGEX_PRECEDING_KEYWORDS: &[&str] = &[
    "return", "typeof", "void", "delete", "throw", "new", "in", "case", "yield", "await",
];

/// Basic JS minification: strip comments and collapse whitespace.
///
/// Uses a state-machine approach that respects string literals and regex
/// literals, so it won't accidentally strip content inside quotes or regex.
/// Correctly handles multi-byte UTF-8 and JS keywords that precede regex.
/// Falls back to the original source if minification would produce empty output.
fn minify_js(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut prev_was_regex_possible = true;
    let mut ident_start = None;

    while i < len {
        let ch = chars[i];

        // String literals — pass through verbatim
        if ch == '\'' || ch == '"' || ch == '`' {
            let quote = ch;
            out.push(ch);
            i += 1;
            while i < len {
                let c = chars[i];
                out.push(c);
                if c == '\\' && i + 1 < len {
                    i += 1;
                    out.push(chars[i]);
                } else if c == quote {
                    break;
                }
                i += 1;
            }
            i += 1;
            prev_was_regex_possible = false;
            ident_start = None;
            continue;
        }

        // Regex literal: /pattern/flags
        // A '/' is a regex start when preceded by an operator, keyword, or
        // punctuation (not by an identifier, number, or closing bracket/paren).
        if ch == '/' && prev_was_regex_possible && i + 1 < len && chars[i + 1] != '/' && chars[i + 1] != '*' {
            out.push(ch);
            i += 1;
            while i < len {
                let c = chars[i];
                out.push(c);
                if c == '\\' && i + 1 < len {
                    i += 1;
                    out.push(chars[i]);
                } else if c == '/' {
                    i += 1;
                    break;
                } else if c == '[' {
                    out.push(c);
                    i += 1;
                    while i < len {
                        let cc = chars[i];
                        out.push(cc);
                        if cc == '\\' && i + 1 < len {
                            i += 1;
                            out.push(chars[i]);
                        } else if cc == ']' {
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }
                i += 1;
            }
            prev_was_regex_possible = false;
            ident_start = None;
            continue;
        }

        // Line comment
        if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Block comment
        if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            } else {
                i = len;
            }
            continue;
        }

        // Collapse runs of whitespace to a single space (or newline if original has one)
        if ch.is_whitespace() {
            if let Some(start) = ident_start.take() {
                prev_was_regex_possible = REGEX_PRECEDING_KEYWORDS.contains(&&out[start..]);
            }
            let has_newline = chars[i..].iter().take_while(|c| c.is_whitespace()).any(|c| *c == '\n');
            out.push(if has_newline { '\n' } else { ' ' });
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            continue;
        }

        // Check if we just finished an identifier — detect regex-preceding keywords
        let is_ident_char = ch.is_ascii_alphanumeric() || ch == '_' || ch == '$';
        if is_ident_char {
            if ident_start.is_none() {
                ident_start = Some(out.len());
            }
        } else {
            if let Some(start) = ident_start.take() {
                prev_was_regex_possible = REGEX_PRECEDING_KEYWORDS.contains(&&out[start..]);
            }
        }

        out.push(ch);

        if is_ident_char {
            // Don't update prev_was_regex_possible mid-identifier;
            // it's resolved above when the identifier ends.
        } else {
            // After these tokens, a '/' could start a regex; after others, it can't
            // Division '/' itself means the next '/' can't start a regex
            // Identifiers, numbers, ), ], ++, -- also prevent regex
            prev_was_regex_possible = matches!(ch,
                '=' | '(' | '[' | '{' | ',' | ';' | '!' | '&' | '|' | '^' | '~'
                | '<' | '>' | '+' | '-' | '*' | '%' | '?' | ':' | '@'
            );
        }

        i += 1;
    }

    // Trim leading/trailing whitespace per line, then remove blank lines
    let result: String = out
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if result.is_empty() { src.to_string() } else { result }
}
