use std::collections::HashSet;

/// Rendering mode for generating HTML output.
///
/// Controls how expression children are rendered:
/// - `Normal`: HTML-escaped via `RenderWrapper` (Component or Display)
/// - `Script`: Escaped via `azumi::escape_script_content()`
/// - `Style`: Escaped via `azumi::escape_style_content()`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Context {
    Normal,
    Script,
    Style,
}

/// Context for code generation, tracking rendering mode and metadata.
///
/// Passed through the code generation tree to control how expressions
/// are rendered and to track CSS scoping information.
#[derive(Clone, Debug)]
pub struct GenerationContext {
    pub mode: Context,
    pub scope_id: Option<String>,
    pub valid_classes: HashSet<String>,
    pub valid_ids: HashSet<String>,
    pub key_expr: Option<proc_macro2::TokenStream>, // @keyed(expr) for data-key attribute
}

impl GenerationContext {
    pub fn normal() -> Self {
        Self {
            mode: Context::Normal,
            scope_id: None,
            valid_classes: HashSet::new(),
            valid_ids: HashSet::new(),
            key_expr: None,
        }
    }

    pub fn with_scope(
        scope_id: String,
        valid_classes: HashSet<String>,
        valid_ids: HashSet<String>,
    ) -> Self {
        Self {
            mode: Context::Normal,
            scope_id: Some(scope_id),
            valid_classes,
            valid_ids,
            key_expr: None,
        }
    }

    pub fn with_key_expr(&self, key_expr: proc_macro2::TokenStream) -> Self {
        Self {
            key_expr: Some(key_expr),
            ..self.clone()
        }
    }

    pub fn with_mode(&self, mode: Context) -> Self {
        Self {
            mode,
            scope_id: self.scope_id.clone(),
            valid_classes: self.valid_classes.clone(),
            valid_ids: self.valid_ids.clone(),
            key_expr: None, // Only first element in for body gets data-key; children don't inherit
        }
    }
    }
}
