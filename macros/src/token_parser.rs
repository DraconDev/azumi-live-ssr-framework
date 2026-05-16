use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Paren},
    Error, Ident, Result, Token,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Node {
    Element(Element),
    Text(Text),
    RawText(Text),
    Expression(Expression),
    Comment(Comment),
    Doctype(Doctype),
    Fragment(Fragment),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
    pub bind_struct: Option<syn::Path>,
    pub span: Span,
    pub full_span: Span,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub content: TokenStream,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    #[allow(dead_code)]
    pub name_span: Span,
    pub value: AttributeValue,
    #[allow(dead_code)]
    pub span: Span,
    #[allow(dead_code)]
    pub value_span: Option<Span>,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Static(String),
    Dynamic(TokenStream),
    StyleDsl(Vec<(String, TokenStream)>), // List of (property, value_expr)
    None,
    /// External CSS class names that bypass validation but are still HTML-escaped.
    /// Used for `class:external="bg-blue-500 px-4"` to allow third-party CSS.
    External(String),
}

#[derive(Debug, Clone)]
pub struct Comment {
    #[allow(dead_code)]
    pub content: String,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Doctype {
    #[allow(dead_code)]
    pub content: String,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub children: Vec<Node>,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Block {
    If(IfBlock),
    For(ForBlock),
    Match(MatchBlock),
    Call(CallBlock),
    Component(ComponentBlock),
    Let(LetBlock),
    Style(StyleBlock),
}

#[derive(Debug, Clone)]
pub struct IfBlock {
    pub condition: TokenStream,
    pub then_branch: Vec<Node>,
    pub else_branch: Option<Vec<Node>>,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForBlock {
    pub pattern: TokenStream,
    pub iterator: TokenStream,
    pub body: Vec<Node>,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchBlock {
    pub expr: TokenStream,
    pub arms: Vec<MatchArm>,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: TokenStream,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct CallBlock {
    pub name: syn::Path,
    pub args: TokenStream, // Named args or positional
    pub children: Vec<Node>,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ComponentBlock {
    #[allow(dead_code)]
    pub name: syn::Path,
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LetBlock {
    pub pattern: TokenStream, // Variable pattern (e.g., `name`, `(x, y)`, etc.)
    pub value: TokenStream,   // The value to assign
    #[allow(dead_code)]
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StyleBlock {
    pub content: TokenStream, // The CSS content inside style! { ... }
    pub is_global: bool,
    #[allow(dead_code)]
    pub span: Span,
}

// Parsing logic

fn parse_style_tag(input: ParseStream) -> Result<Node> {
    let start_span = input.span();
    input.parse::<Token![<]>()?;
    parse_html_name(input, false)?; // "style"

    // Check for 'global' attribute
    let mut is_global = false;
    while !input.peek(Token![>]) && !input.peek(Token![/]) {
        let fork = input.fork();
        if let Ok(ident) = fork.parse::<Ident>() {
            if ident == "global" {
                input.parse::<Ident>()?; // Consume 'global'
                is_global = true;
                continue;
            }
        }
        // Skip other attributes
        input.parse::<TokenTree>()?;
    }

    if input.peek(Token![/]) {
        // Self-closing <style /> or <style global /> -> empty
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        return Ok(Node::Block(Block::Style(StyleBlock {
            content: TokenStream::new(),
            is_global,
            span: start_span,
        })));
    }

    input.parse::<Token![>]>()?;

    // Check if content is a brace expression: <style>{expr}</style>
    // If so, parse as an Element with expression children (auto-escaped)
    if input.peek(Brace) {
        let expr_node = Node::Expression(input.parse()?);
        // Expect closing </style>
        if input.peek(Token![<]) && input.peek2(Token![/]) {
            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            parse_html_name(input, false)?;
            input.parse::<Token![>]>()?;
        }
        return Ok(Node::Element(Element {
            name: "style".to_string(),
            attrs: vec![],
            children: vec![expr_node],
            bind_struct: None,
            span: start_span,
            full_span: start_span,
        }));
    }

    // Parse content until </style> (style! DSL content)
    let mut content = TokenStream::new();
    while !input.is_empty() {
        // Check for </style>
        let fork = input.fork();
        if fork.parse::<Token![<]>().is_ok() && fork.parse::<Token![/]>().is_ok() {
            if let Ok((end_name, _)) = parse_html_name(&fork, false) {
                if end_name == "style" && fork.parse::<Token![>]>().is_ok() {
                    // Found </style>
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![/]>()?;
                    parse_html_name(input, false)?;
                    input.parse::<Token![>]>()?;
                    break;
                }
            }
        }
        content.extend(std::iter::once(input.parse::<TokenTree>()?));
    }

    Ok(Node::Block(Block::Style(StyleBlock {
        content,
        is_global,
        span: start_span,
    })))
}

pub fn parse_nodes(input: ParseStream) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        // Skip whitespace-only tokens
        let fork = input.fork();
        if let Ok(tt) = fork.parse::<TokenTree>() {
            if tt.to_string().trim().is_empty() {
                input.parse::<TokenTree>()?; // Consume the whitespace
                continue;
            }
        }

        if input.peek(Token![<]) {
            // Element, Comment, Doctype, Fragment
            if input.peek2(Token![!]) {
                // Comment or Doctype
                if input.peek3(Token![-]) {
                    nodes.push(Node::Comment(input.parse()?));
                } else {
                    nodes.push(Node::Doctype(input.parse()?));
                }
            } else if input.peek2(Token![/]) {
                // Closing tag - should be handled by parent element parser
                // If we see it here, it's an error (unmatched closing tag)
                // But we might be in a loop parsing children, so we stop.
                break;
            } else if input.peek2(Token![>]) {
                // Fragment < >
                nodes.push(Node::Fragment(input.parse()?));
            } else {
                // Element or Style
                let fork = input.fork();
                fork.parse::<Token![<]>()?;
                if let Ok((name, _)) = parse_html_name(&fork, false) {
                    if name == "style" {
                        nodes.push(parse_style_tag(input)?);
                        continue;
                    }
                }
                nodes.push(Node::Element(input.parse()?));
            }
        } else if input.peek(Token![@]) {
            if input.peek2(Brace) {
                // @{ ... } -> Expression
                input.parse::<Token![@]>()?;
                nodes.push(Node::Expression(input.parse()?));
            } else {
                // Block
                nodes.push(Node::Block(input.parse()?));
            }
        } else if input.peek(Brace) {
            // Expression { ... }
            nodes.push(Node::Expression(input.parse()?));
        } else if input.peek(syn::Lit) {
            // Text content (must be string literal)
            nodes.push(Node::Text(input.parse()?));
        } else {
            // Unexpected token
            return Err(Error::new(
                input.span(),
                "Unexpected token. Expected:\n\
                - HTML element: <tag>...</tag>\n\
                - Expression: {expr} or @{expr}\n\
                - Control flow: @if, @for, @match, @let\n\
                - Text content (must be quoted): \"text\"\n\
                - Component call: @ComponentName(...) or @function(...)\n\
                - Style: <style>.class { ... }</style>",
            ));
        }
    }
    Ok(nodes)
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let start_span = input.span();
        input.parse::<Token![<]>()?;
        let (name, name_span) = parse_html_name(input, false)?; // false = don't allow double dash in tag names

        let mut attrs: Vec<Attribute> = Vec::new();
        let mut bind_struct = None;
        let mut full_span = start_span;

        while !input.is_empty() && !input.peek(Token![>]) && !input.peek(Token![/]) {
            // Check for bind={Struct} on <form> tags
            if name == "form" && input.peek(Ident) {
                let fork = input.fork();
                let key: Ident = fork.parse()?;
                if key == "bind" {
                    input.parse::<Ident>()?; // consume "bind"
                    input.parse::<Token![=]>()?;

                    let content;
                    syn::braced!(content in input);
                    let path: syn::Path = content.parse()?;
                    bind_struct = Some(path);
                    continue;
                }
            }
            attrs.push(input.parse()?);
        }

        // Azumi is strict: duplicate HTML attributes are invalid and lead to silent overrides in browsers.
        // Fail closed at compile time so markup can't "almost work".
        {
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
            for attr in &attrs {
                if !seen.insert(attr.name.clone()) {
                    let guidance = match attr.name.as_str() {
                        "class" => "Use a single `class={...}` and combine classes in one expression, e.g. `class={card active}`.",
                        "style" => "Use a single `style={...}` attribute (prefer the Azumi style DSL / CSS variables).",
                        _ => "Remove the duplicate attribute and compute the final value before rendering.",
                    };
                    return Err(Error::new(
                        attr.name_span,
                        format!(
                            "Duplicate attribute `{}` on <{}> is not allowed.\n\n{}",
                            attr.name, name, guidance
                        ),
                    ));
                }
            }
        }

        // NOTE: Magic <script src="azumi.js"> transformation was REMOVED in v15.14.0
        // Use {azumi_script()} instead - explicit, type-safe, follows Component patterns

        // Azumi: Enforce component-scoped CSS - block <link rel="stylesheet"> for local files
        if name == "link" {
            let has_rel_stylesheet = attrs.iter().any(|attr: &Attribute| {
                attr.name == "rel"
                    && matches!(&attr.value, AttributeValue::Static(v) if v == "stylesheet")
            });

            if has_rel_stylesheet {
                if let Some(href_attr) = attrs.iter().find(|attr| attr.name == "href") {
                    if let AttributeValue::Static(href) = &href_attr.value {
                        // Allow external URLs (http/https), block local paths
                        if !href.starts_with("http://") && !href.starts_with("https://") {
                            return Err(Error::new(
                                start_span,
                                format!(
                                    "Local CSS must use component-scoped <style src> instead of <link>:\n\n\
                                     ✅ <style src=\"{}\" />  (auto-scoped to component)\n\
                                     ❌ <link rel=\"stylesheet\" href=\"{}\" />\n\n\
                                     Why? All local CSS is component-scoped in Azumi.\n\
                                     External CDN stylesheets (https://...) are allowed with <link>.",
                                    href, href
                                ),
                            ));
                        }
                    }
                }
            }
        }

        let mut children = Vec::new();
        if input.peek(Token![/]) {
            // Self-closing tag (e.g., <img />, <br />)
            input.parse::<Token![/]>()?;
            let end_token = input.parse::<Token![>]>()?;
            if let Some(joined) = start_span.join(end_token.span()) {
                full_span = joined;
            } else {
                full_span = name_span;
            }
        } else {
            input.parse::<Token![>]>()?;

            // Parse children
            if is_void_element(&name) {
                // Void element, no children, no closing tag
            } else {
                if name == "script" || name == "style" {
                    children = parse_script_content(input, &name)?;
                } else {
                    children = parse_nodes(input)?;
                }

                // Expect closing tag
                if input.peek(Token![<]) && input.peek2(Token![/]) {
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![/]>()?;
                    let (closing_name, _) = parse_html_name(input, false)?;
                    if closing_name != name {
                        return Err(Error::new(
                            input.span(),
                            format!(
                                "Mismatched closing tag: expected </{}>, found </{}>",
                                name, closing_name
                            ),
                        ));
                    }
                    let end_token = input.parse::<Token![>]>()?;
                    if let Some(joined) = start_span.join(end_token.span()) {
                        full_span = joined;
                    } else {
                        // Fallback to name_span
                        full_span = name_span;
                    }
                } else {
                    return Err(Error::new(
                        start_span,
                        format!("Unclosed element <{}>", name),
                    ));
                }
            }
        }

        // NOTE: <script src="azumi.js" /> transformation was REMOVED in v15.14.0
        // Use {azumi_script()} instead - explicit, type-safe, follows Component patterns

        // Azumi: Block inline <style> and <script> tags
        // Moved to end of parsing to allow checking children for expressions
        if name == "style" || name == "script" {
            let has_src = attrs.iter().any(|attr: &Attribute| attr.name == "src");
            let is_internal = attrs
                .iter()
                .any(|attr: &Attribute| attr.name == "data-azumi-scope");
            let is_json_script = name == "script"
                && attrs.iter().any(|attr: &Attribute| {
                    attr.name == "type"
                        && matches!(&attr.value, AttributeValue::Static(v) if v.contains("json"))
                });

            // Allow scripts and styles with dynamic content (expressions) like json_data! or bare {var}
            let has_expression_child = children
                .iter()
                .any(|node| matches!(node, Node::Expression(_)));

            if !(has_src
                || is_internal
                || (name == "script" && (is_json_script || has_expression_child))
                || (name == "style" && has_expression_child))
            {
                let tag_help = if name == "script" {
                    "JavaScript must be external or JSON data:
  ✅ <script src=\"/static/app.js\" />
  ✅ <script type=\"application/json\">{{ data }}</script>
  ✅ <script>{ azumi::json_data!(\"MY_DATA\" = &data) }</script>
  ✅ <script>{TRACKING_JS}</script>
  ❌ <script>const x = 42;</script>

For data: use json_data! macro or data-* attributes"
                } else {
                    "CSS must be external:
  ✅ <style src=\"components/card.css\" />  (auto-scoped)
  ✅ <style>{GLOBAL_CSS}</style>
  ❌ <style>.card { padding: 2em; }</style>

For dynamic styles: use <style>{variable}</style> or style attribute with expressions"
                };

                return Err(Error::new(
                    if let Some(joined) = start_span.join(name_span) { joined } else { name_span },
                    format!(
                                "Inline <{}> tags not allowed in Azumi\n\n{}\n\nUse external files or {{expression}} instead of Raw(). See: docs/guide.md.",
                        name, tag_help
                    ),
                ));
            }
        }

        Ok(Element {
            name,
            attrs,
            children,
            bind_struct,
            span: start_span,
            full_span,
        })
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let (name, name_span) = parse_html_name(input, true)?; // true = allow double dash in attributes

        // Check if this is a boolean attribute (no value required)
        const BOOLEAN_ATTRS: &[&str] = &[
            "allow",
            "async",
            "autofocus",
            "autoplay",
            "checked",
            "controls",
            "default",
            "defer",
            "disabled",
            "download",
            "formnovalidate",
            "global",
            "hidden",
            "inert",
            "ismap",
            "itemscope",
            "loop",
            "multiple",
            "muted",
            "nomodule",
            "novalidate",
            "open",
            "readonly",
            "required",
            "reversed",
            "selected",
            "truespeed",
        ];

        // Check for class:external (external CSS class names, bypass validation but still escaped)
        if name == "class:external" {
            if !input.peek(Token![=]) {
                return Err(Error::new(
                    name_span,
                    "class:external requires a value. Example: class:external=\"bg-blue-500 px-4\"",
                ));
            }
            input.parse::<Token![=]>()?;
            let tt: proc_macro2::TokenTree = input.parse()?;
            if let proc_macro2::TokenTree::Literal(lit) = &tt {
                if let Ok(syn::Lit::Str(s)) = syn::parse_str(&lit.to_string()) {
                    return Ok(Attribute {
                        name,
                        name_span,
                        value: AttributeValue::External(s.value()),
                        span: name_span,
                        value_span: Some(lit.span()),
                    });
                }
            }
            return Err(Error::new(
                name_span,
                "class:external requires a double-quoted string literal. Example: class:external=\"bg-blue-500\"",
            ));
        }

        let (value, value_span) = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            // Robust parsing: Consume the next token tree and inspect it
            // This works around issues where peek(Brace) might fail for groups
            let tt: proc_macro2::TokenTree = input.parse()?;
            match tt {
                proc_macro2::TokenTree::Group(g) if g.delimiter() == proc_macro2::Delimiter::Brace => {
                     // It's a brace group { ... }
                     let stream = g.stream();

                     // Check if this is a style attribute with DSL syntax
                     if name == "style" {
                         // Check for -- prefix
                         let is_style_dsl = 'check: {
                            let mut fork = stream.clone().into_iter();
                            if !matches!(fork.next(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '-') { break 'check false; }
                            if !matches!(fork.next(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '-') { break 'check false; }
                            true
                         };

                         if is_style_dsl {
                             // Re-implement the style DSL parser using the stream
                             let content_parser = |input: ParseStream| {
                                let mut props = Vec::new();
                                while !input.is_empty() {
                                    // Parse property name: --foo-bar
                                    let mut prop_name = String::new();
                                    input.parse::<Token![-]>()?;
                                    input.parse::<Token![-]>()?;
                                    prop_name.push_str("--");

                                    // Parse rest of identifier parts
                                    let ident = input.parse::<syn::Ident>()?;
                                    prop_name.push_str(&ident.to_string());

                                    while input.peek(Token![-]) {
                                        input.parse::<Token![-]>()?;
                                        prop_name.push('-');
                                        let part = input.parse::<syn::Ident>()?;
                                        prop_name.push_str(&part.to_string());
                                    }

                                    input.parse::<Token![:]>()?;

                                    // Parse value expression until semicolon, comma, or end
                                    let mut value_tokens = TokenStream::new();
                                    while !input.is_empty()
                                        && !input.peek(Token![;])
                                        && !input.peek(Token![,])
                                    {
                                        value_tokens.extend(std::iter::once(input.parse::<TokenTree>()?));
                                    }

                                    props.push((prop_name, value_tokens));

                                    // Consume separator if present
                                    if input.peek(Token![;]) {
                                        input.parse::<Token![;]>()?;
                                    } else if input.peek(Token![,]) {
                                        input.parse::<Token![,]>()?;
                                    }
                                }
                                Ok(props)
                             };

                             // Use Parser trait method parse2
                             let props = syn::parse::Parser::parse2(content_parser, stream)?;
                             (AttributeValue::StyleDsl(props), None)
                         } else {
                             (AttributeValue::Dynamic(stream), None)
                         }
                     } else {
                         (AttributeValue::Dynamic(stream), None)
                     }
                }
                proc_macro2::TokenTree::Literal(lit) => {
                    // It's a literal. Convert to syn::Lit to check if string and properly unescape.
                    if let Ok(syn::Lit::Str(s)) = syn::parse_str(&lit.to_string()) {
                        // syn::LitStr properly handles escape sequences like \" and \\
                        (AttributeValue::Static(s.value()), Some(lit.span()))
                    } else {
                        return Err(Error::new(
                            name_span,
                            format!("Attribute '{}' value must be a double-quoted string literal or dynamic expression {{...}}.", name)
                        ))
                    }
                }
                 _ => {
                        return Err(Error::new(
                            name_span,
                            format!("Attribute '{}' value error. Found unexpected token: {:?}. Expected string literal or {{...}}", name, tt)
                        ))
                 }
            }
        } else {
            // No = sign - must be a boolean attribute
            // CSS variables (starting with --) are NOT boolean attributes, they require a value
            if name.starts_with("--") {
                return Err(Error::new(
                    name_span,
                    format!(
                        "CSS variable '{}' requires a value. Example: {}=\"value\"",
                        name, name
                    ),
                ));
            }

            if !BOOLEAN_ATTRS.contains(&name.as_str()) {
                return Err(Error::new(
                    name_span,
                    format!("Attribute '{}' requires a value. Use {}=\"value\" or {}={{expr}}.\nOnly boolean attributes like 'disabled', 'checked', etc. can omit values.", name, name, name)
                ));
            }
            (AttributeValue::None, None)
        };

        let mut span = name_span;
        if let Some(v_span) = value_span {
            if let Some(joined) = span.join(v_span) {
                span = joined;
            }
        }

        Ok(Attribute {
            name,
            name_span,
            value,
            span,
            value_span,
        })
    }
}

impl Parse for Text {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();

        // Rusti 2.0: All text content must be double-quoted string literals
        // This prevents lexer issues with patterns like "2e5", "88Ester", etc.
        if input.peek(syn::Lit) {
            let lit: syn::Lit = input.parse()?;
            match lit {
                syn::Lit::Str(s) => {
                    return Ok(Text {
                        content: s.value(),
                        span,
                    });
                }
                _ => {
                    return Err(Error::new(
                        span,
                        "Text content must be a double-quoted string literal.\nExample: <h1>\"Hello World\"</h1>"
                    ));
                }
            }
        }

        // If no string literal found, error
        Err(Error::new(
            span,
            "Text content must be a double-quoted string literal to prevent lexer issues.\nExample: <p>\"Your text here\"</p>\nFor dynamic content, use {expression} instead."
        ))
    }
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let span = input.span();
        syn::braced!(content in input);

        // Collect all tokens and reassemble
        let mut all_tokens = TokenStream::new();
        while !content.is_empty() {
            all_tokens.extend(Some(content.parse::<TokenTree>()?));
        }

        Ok(Expression {
            content: all_tokens,
            span,
        })
    }
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![@]>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            Ok(Block::If(input.parse()?))
        } else if lookahead.peek(Token![for]) {
            Ok(Block::For(input.parse()?))
        } else if lookahead.peek(Token![match]) {
            Ok(Block::Match(input.parse()?))
        } else if lookahead.peek(Token![let]) {
            Ok(Block::Let(input.parse()?))
        } else {
            // Component or Call
            // Check if it's a path
            // Use parse_mod_style to avoid parsing generics (which might confuse < with HTML tags)
            let path: syn::Path = syn::Path::parse_mod_style(input)?;
            if input.peek(Paren) {
                // Call @foo(...)
                let content;
                let _ = syn::parenthesized!(content in input);
                let args = content.parse()?;

                // Optional children { ... }
                let children = if input.peek(Brace) {
                    let child_content;
                    syn::braced!(child_content in input);
                    parse_nodes(&child_content)?
                } else {
                    Vec::new()
                };

                let span = path.span();
                Ok(Block::Call(CallBlock {
                    name: path,
                    args,
                    children,
                    span,
                }))
            } else {
                // Component variable @foo or call with children @foo { ... }
                let span = path.span();

                // Optional children { ... }
                if input.peek(Brace) {
                    let child_content;
                    syn::braced!(child_content in input);
                    let children = parse_nodes(&child_content)?;

                    Ok(Block::Call(CallBlock {
                        name: path,
                        args: TokenStream::new(),
                        children,
                        span,
                    }))
                } else {
                    Ok(Block::Component(ComponentBlock { name: path, span }))
                }
            }
        }
    }
}

// Helpers

fn parse_html_name(input: ParseStream, allow_double_dash: bool) -> Result<(String, Span)> {
    let mut name = String::new();
    let mut full_span = input.span();

    // Check for CSS variable prefix --
    if allow_double_dash && input.peek(Token![-]) && input.peek2(Token![-]) {
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        name.push_str("--");

        // After --, we expect an identifier (including keywords like static)
        let fork = input.fork();
        if Ident::parse_any(&fork).is_ok() {
            let ident = Ident::parse_any(input)?;
            name.push_str(&ident.to_string());
            if let Some(joined) = full_span.join(ident.span()) {
                full_span = joined;
            }
        } else {
            return Err(input.error("Expected identifier after --"));
        }
    } else {
        // Check if it starts with an identifier (or keyword)
        let fork = input.fork();
        if Ident::parse_any(&fork).is_ok() {
            let ident = Ident::parse_any(input)?;
            name.push_str(&ident.to_string());
            full_span = ident.span();
        } else {
            return Err(input.error("Expected identifier"));
        }
    }

    // Continue parsing rest of the name (e.g. -foo:bar or .modifier)
    while input.peek(Token![-]) || input.peek(Token![:]) || input.peek(Token![.]) {
        // Double dash in tag names is invalid HTML — return a clear error instead of breaking silently
        if input.peek(Token![-]) && input.peek2(Token![-]) && !allow_double_dash {
            return Err(Error::new(
                input.span(),
                "Double dash (`--`) is not allowed in HTML tag names. \
                 CSS custom properties (`--var`) are only allowed in attribute names.",
            ));
        }

        let punct_span = input.span();
        if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            name.push('-');
        } else if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            name.push(':');
        } else {
            input.parse::<Token![.]>()?;
            name.push('.');
        }
        if let Some(joined) = full_span.join(punct_span) {
            full_span = joined;
        }

        let fork = input.fork();
        if Ident::parse_any(&fork).is_ok() {
            let part = Ident::parse_any(input)?;
            name.push_str(&part.to_string());
            if let Some(joined) = full_span.join(part.span()) {
                full_span = joined;
            }
        } else {
            // Allow numbers?
            if input.peek(syn::Lit) {
                let lit: syn::Lit = input.parse()?;
                name.push_str(&lit.to_token_stream().to_string());
                if let Some(joined) = full_span.join(lit.span()) {
                    full_span = joined;
                }
            }
        }
    }

    Ok((name, full_span))
}

pub(crate) const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link",
    "meta", "param", "source", "track", "wbr",
];

fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.contains(&name)
}

fn is_css_at_rule(input: ParseStream) -> bool {
    let fork = input.fork();
    if fork.parse::<Token![@]>().is_ok() {
        if let Ok(ident) = fork.parse::<Ident>() {
            let s = ident.to_string();
            matches!(
                s.as_str(),
                "keyframes"
                    | "media"
                    | "import"
                    | "font-face"
                    | "supports"
                    | "page"
                    | "layer"
                    | "container"
                    | "charset"
                    | "namespace"
            )
        } else {
            false
        }
    } else {
        false
    }
}

fn parse_script_content(input: ParseStream, tag_name: &str) -> Result<Vec<Node>> {
    let debug = std::env::var("AZUMI_DEBUG").is_ok();
    let mut nodes = Vec::new();
    while !input.is_empty() {
        if input.peek(Token![<]) && input.peek2(Token![/]) {
            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            fork.parse::<Token![/]>()?;
            if let Ok((name, _)) = parse_html_name(&fork, false) {
                if name == tag_name {
                    break;
                }
            }
        }

        if input.peek(Brace) {
            // Bare { ... } -> Expression
            if debug {
                eprintln!("Found {{ ... }} bare expression!");
            }
            nodes.push(Node::Expression(input.parse()?));
            continue;
        }

        if input.peek(Token![@]) {
            let is_css = is_css_at_rule(input);
            if debug {
                eprintln!("Found @ in script, is_css_at_rule: {}", is_css);
            }
            if !is_css {
                if debug {
                    eprintln!("Not CSS, checking if it's a Brace...");
                }
                if input.peek2(Brace) {
                    // @{ ... } -> Expression
                    if debug {
                        eprintln!("Found @{{ ... }} expression!");
                    }
                    input.parse::<Token![@]>()?;
                    nodes.push(Node::Expression(input.parse()?));
                    continue;
                }
                if debug {
                    eprintln!("Found Block (not brace)");
                }
                nodes.push(Node::Block(input.parse()?));
                continue;
            } else if debug {
                eprintln!("IS CSS, treating as text");
            }
        }

        if !input.peek(Token![@]) || is_css_at_rule(input) {
            // Parse as text until @ (if not CSS) or </tag_name>
            let span = input.span();
            let mut tokens = Vec::new();
            if debug {
                eprintln!("Parsing text...");
            }
            while !input.is_empty() {
                if input.peek(Token![@]) {
                    if debug {
                        eprintln!("Stopped text at @");
                    }
                    break;
                }
                if input.peek(Token![<]) && input.peek2(Token![/]) {
                    let fork = input.fork();
                    fork.parse::<Token![<]>()?;
                    fork.parse::<Token![/]>()?;
                    if let Ok((name, _)) = parse_html_name(&fork, false) {
                        if name == tag_name {
                            if debug {
                                eprintln!("Stopped text at closing tag");
                            }
                            break;
                        }
                    }
                }

                let tt: TokenTree = input.parse()?;
                if debug {
                    eprintln!("Consumed token: {:?}", tt);
                }
                tokens.push(tt);
            }

            if !tokens.is_empty() {
                let content = tokens_to_string(&tokens);
                if debug {
                    eprintln!("Created RawText node: {:?}", content);
                }
                nodes.push(Node::RawText(Text { content, span }));
            }
        }
    }
    Ok(nodes)
}

fn tokens_to_string(tokens: &[TokenTree]) -> String {
    let mut output = String::new();
    for (i, tt) in tokens.iter().enumerate() {
        let s = tt.to_string();
        output.push_str(&s);

        if i + 1 < tokens.len() {
            let next = &tokens[i + 1];
            if should_add_space(tt, next) {
                output.push(' ');
            }
        }
    }
    output
}

fn should_add_space(curr: &TokenTree, next: &TokenTree) -> bool {
    use proc_macro2::TokenTree::*;
    match (curr, next) {
        (Ident(_), Ident(_)) => true,     // const app
        (Ident(_), Literal(_)) => true,   // return 0
        (Literal(_), Ident(_)) => true,   // 0 auto
        (Literal(_), Literal(_)) => true, // 1 2
        (Punct(p), Ident(_)) if p.as_char() == ',' || p.as_char() == ';' => true, // , next or ; next
        _ => false,
    }
}

impl Parse for IfBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![if]>()?;
        let mut condition = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            condition.extend(Some(tt));
        }

        let content;
        syn::braced!(content in input);
        let then_branch = parse_nodes(&content)?;

        let else_branch = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            // Support @else if chains by recursively parsing IfBlock
            if input.peek(Token![if]) {
                Some(vec![Node::Block(Block::If(input.parse::<IfBlock>()?))])
            } else {
                let content;
                syn::braced!(content in input);
                Some(parse_nodes(&content)?)
            }
        } else {
            None
        };

        Ok(IfBlock {
            condition,
            then_branch,
            else_branch,
            span,
        })
    }
}

impl Parse for ForBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![for]>()?;

        // pattern in iterator
        // We need to find "in"
        let mut pre_in = TokenStream::new();
        while !input.peek(Token![in]) && !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            pre_in.extend(Some(tt));
        }

        if input.peek(Token![in]) {
            input.parse::<Token![in]>()?;
        } else {
            return Err(Error::new(input.span(), "Expected 'in' in for loop"));
        }

        let mut iterator = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            iterator.extend(Some(tt));
        }

        let content;
        syn::braced!(content in input);
        let body = parse_nodes(&content)?;

        Ok(ForBlock {
            pattern: pre_in,
            iterator,
            body,
            span,
        })
    }
}

impl Parse for LetBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![let]>()?;

        // Parse pattern (variable name or destructuring pattern) until =
        let mut pattern = TokenStream::new();
        while !input.peek(Token![=]) && !input.peek(Token![;]) {
            let tt: TokenTree = input.parse()?;
            pattern.extend(Some(tt));
        }

        // Parse = token
        input.parse::<Token![=]>()?;

        // Parse value until semicolon
        let mut value = TokenStream::new();
        while !input.peek(Token![;]) {
            if input.is_empty() {
                return Err(Error::new(span, "Expected ';' after @let value"));
            }
            let tt: TokenTree = input.parse()?;
            value.extend(Some(tt));
        }

        // Parse semicolon
        input.parse::<Token![;]>()?;

        Ok(LetBlock {
            pattern,
            value,
            span,
        })
    }
}

impl Parse for MatchBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![match]>()?;

        let mut expr = TokenStream::new();
        while !input.peek(Brace) {
            let tt: TokenTree = input.parse()?;
            expr.extend(Some(tt));
        }

        if !input.peek(Brace) {
            return Err(Error::new(
                input.span(),
                "Expected block { ... } in match expression",
            ));
        }
        let content;
        syn::braced!(content in input);

        // Parse arms
        let mut arms = Vec::new();
        while !content.is_empty() {
            // pattern => { body } or pattern => single_node
            let mut pattern = TokenStream::new();
            while !content.peek(Token![=>]) && !content.is_empty() {
                let tt: TokenTree = content.parse()?;
                pattern.extend(Some(tt));
            }

            if content.peek(Token![=>]) {
                content.parse::<Token![=>]>()?;
            } else {
                return Err(Error::new(content.span(), "Expected =>"));
            }

            // Check if body is braced or single expression
            let body = if content.peek(Brace) {
                // Braced body: { ... }
                let body_content;
                syn::braced!(body_content in content);
                parse_nodes(&body_content)?
            } else {
                // Single node without braces
                let mut single_node = Vec::new();

                // Parse until we hit a comma or end of arms
                // We need to be careful here - parse one node
                if content.peek(Token![<]) {
                    // HTML element
                    single_node.push(Node::Element(content.parse()?));
                } else if content.peek(Token![@]) {
                    // Block (if, for, match, let, component, call)
                    single_node.push(Node::Block(content.parse()?));
                } else {
                    return Err(Error::new(
                        content.span(),
                        "Expected HTML element, block (@if, @for, @match, @let, component call), or braced body { ... }",
                    ));
                }

                single_node
            };

            // Optional comma
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }

            arms.push(MatchArm { pattern, body });
        }

        Ok(MatchBlock { expr, arms, span })
    }
}

impl Parse for Comment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;

        // Consume until -->
        let mut content = String::new();
        loop {
            if input.peek(Token![-]) && input.peek2(Token![-]) && input.peek3(Token![>]) {
                input.parse::<Token![-]>()?;
                input.parse::<Token![-]>()?;
                input.parse::<Token![>]>()?;
                break;
            }
            // Check for -- appearing in content (invalid HTML comment)
            if input.peek(Token![-]) && input.peek2(Token![-]) {
                let _dash1: Token![-] = input.parse()?;
                let _dash2: Token![-] = input.parse()?;
                content.push_str("--");
                // Don't allow -- before > (must be -- > for end)
                if !input.peek(Token![>]) {
                    return Err(syn::Error::new(span, "Invalid comment: '--' not allowed inside comment (must be '--' followed by '>')"));
                }
                continue;
            }
            if input.is_empty() {
                return Err(Error::new(span, "Unclosed comment"));
            }
            let tt: TokenTree = input.parse()?;
            content.push_str(&tt.to_string());
        }

        Ok(Comment { content, span })
    }
}

impl Parse for Doctype {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        // DOCTYPE html
        let mut content = String::new();
        while !input.peek(Token![>]) {
            let tt: TokenTree = input.parse()?;
            content.push_str(&tt.to_string());
            content.push(' ');
        }
        input.parse::<Token![>]>()?;
        Ok(Doctype { content, span })
    }
}

impl Parse for Fragment {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        input.parse::<Token![<]>()?;
        input.parse::<Token![>]>()?;

        let children = parse_nodes(input)?;

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let (closing_name, _) = parse_html_name(input, false)?;
        if closing_name != "fragment" {
            return Err(Error::new(
                input.span(),
                format!(
                    "Invalid closing tag for fragment: expected </fragment>, found </{}>",
                    closing_name
                ),
            ));
        }
        input.parse::<Token![>]>()?;

        Ok(Fragment { children, span })
    }
}

pub struct HtmlInput {
    pub nodes: Vec<Node>,
}

impl Parse for HtmlInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let nodes = parse_nodes(input)?;
        Ok(HtmlInput { nodes })
    }
}
