use azumi::prelude::*;

/// Comprehensive Syntax Reference
/// This component demonstrates valid and invalid syntax patterns in Azumi.
/// Uncommenting the BANNED lines should trigger compile-time errors.
#[azumi::component]
pub fn SyntaxReference() -> impl Component {
    let is_active = true;

    html! {


        <div class={valid_class}>
            <h1>"Azumi Syntax Reference"</h1>

            // ==========================================
            // SECTION 1: CSS CLASSES (class={...})
            // ==========================================
            <div class={second_class}>
                "1. CSS Classes"
            </div>

            // ✅ ALLOWED: Single auto-scoped variable
            <div class={valid_class}>"Single Variable"</div>

            // ✅ ALLOWED: Multiple variables (Space separated)
            <div class={valid_class second_class}>"Multiple Variables"</div>

            // ✅ ALLOWED: Conditional logic (Expressions)
            <div class={if is_active { valid_class } else { error_state }}>
                "Conditional Logic"
            </div>

            // ❌ BANNED: Static string attributes
            // <div class="valid_class">"Static String"</div>

            // ❌ BANNED: String literals in brackets
            // <div class:external="valid_class">"Brackets String"</div>

            // ❌ BANNED: Mixed string literals
            // <div class={valid_class "some_string"}>"Mixed"</div>

            // ==========================================
            // SECTION 2: IDs (id={...})
            // ==========================================
            <div class={second_class}>
                "2. IDs"
            </div>

            // ✅ ALLOWED: Variable
            <div id={valid_id}>"Valid ID"</div>

            // ❌ BANNED: Static string
            // <div id="valid_id">"Static ID"</div>

            // ❌ BANNED: String literal
            // <div id={"valid_id"}>"String ID"</div>

            // ==========================================
            // SECTION 3: INLINE STYLES (style={...})
            // ==========================================
            <div class={second_class}>
                "3. Inline Styles"
            </div>

            // ✅ ALLOWED: Style DSL (Typed CSS variables)
            <div style={ --text-color: "blue"; --gap: "1rem" }>
                "Style DSL"
            </div>

            // ❌ BANNED: Static style string
            // <div style="color: blue">"Static Style"</div>

            // ❌ BANNED: Dynamic style string
            // <div style={"color: blue"}>"Dynamic String Style"</div>
        </div>
        <style>
            .valid_class { color: "green"; }
            .second_class { font-weight: "bold"; }
            .error_state { color: "red"; }
            #valid_id { border: "1px solid black"; }
        </style>
    }
}
