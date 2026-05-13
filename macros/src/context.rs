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
}

impl GenerationContext {
    pub fn normal() -> Self {
        Self {
            mode: Context::Normal,
            scope_id: None,
            valid_classes: HashSet::new(),
            valid_ids: HashSet::new(),
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
        }
    }

    pub fn with_mode(&self, mode: Context) -> Self {
        Self {
            mode,
            scope_id: self.scope_id.clone(),
            valid_classes: self.valid_classes.clone(),
            valid_ids: self.valid_ids.clone(),
        }
    }
}
