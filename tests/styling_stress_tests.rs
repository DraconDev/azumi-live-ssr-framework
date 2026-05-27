use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Scoping & Isolation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn ChildComponent() -> impl Component {
    html! {
        <div class={inner_box}>"Child"</div>
        <style>
            .inner_box { color: "red"; }
        </style>
    }
}

#[azumi::component]
fn ParentComponent() -> impl Component {
    html! {
        <div class={container}>
            <div class={outer_box}>"Parent"</div>
            @ChildComponent()
        </div>
        <style>
            .container { padding: "1rem"; }
            .outer_box { color: "blue"; }
        </style>
    }
}

#[test]
fn test_style_scoping_isolation() {
    let comp = html! { @ParentComponent() };
    let output = test::render(&comp);

    // Azumi uses attribute-based scoping: data-sHASH
    let count = output.matches("data-s").count();
    // 2 in <style> tags, 3 in <div> tags = 5 total
    assert!(
        count >= 5,
        "Expected at least 5 data-s attributes, found {}\nOutput: {}",
        count,
        output
    );

    // Verify we have at least two DIFFERENT scope IDs
    // Scope attributes are in format data-{scope_id} (e.g., data-s1a2b3c)
    let mut scopes = std::collections::HashSet::new();
    // Extract from style tags which contain [data-{hash}] in selectors
    for part in output.split("[data-") {
        if let Some(end) = part.find(']') {
            let scope_id = &part[..end];
            if !scope_id.is_empty() {
                scopes.insert(scope_id.to_string());
            }
        }
    }
    assert!(
        scopes.len() >= 2,
        "Expected at least 2 unique scope IDs, found {:?}",
        scopes
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Global Style Propagation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn GlobalStyled() -> impl Component {
    html! {
        <div class:external="global_box">"Global"</div>
        <style global>
            .global_box { background: "green"; }
        </style>
    }
}

#[test]
fn test_global_style_unmangled() {
    let comp = html! { @GlobalStyled() };
    let output = test::render(&comp);
    assert!(
        output.contains("class=\"global_box\""),
        "Global class should not be mangled"
    );
    // Flexible match for CSS content (ignoring exact whitespace/semicolons)
    assert!(
        output.contains(".global_box"),
        "Global selector should be present"
    );
    assert!(
        output.contains("background: green") || output.contains("background:green"),
        "Global style value should be present"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Dynamic Variable Propagation
// ════════════════════════════════════════════════════════════════════════════

#[azumi::component]
fn Progress(width: f64) -> impl Component {
    html! {
        <div class={track_id}>
            <div class={fill_id} style={--w: format!("{}%", width)}></div>
        </div>
        <style>
            .track_id { width: "100px"; height: "10px"; background: "#eee"; }
            .fill_id { width: "var(--w)"; height: "100%"; background: "blue"; }
        </style>
    }
}

#[test]
fn test_dynamic_custom_properties() {
    let comp = html! { @Progress(width = 75.5) };
    let output = test::render(&comp);
    // Azumi renders as "--w: 75.5%" (with space)
    assert!(
        output.contains("--w: 75.5%"),
        "Custom property should be rendered correctly. Output: {}",
        output
    );
}

#[test]
fn test_multiple_custom_properties() {
    let comp = html! {
        <div style={--a: "1"; --b: "2"; --c: "3"}>"Multi"</div>
    };
    let output = test::render(&comp);
    assert!(output.contains("--a: 1") && output.contains("--b: 2") && output.contains("--c: 3"));
}

// ════════════════════════════════════════════════════════════════════════════
// Complex Scoping (Multiple Components)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_style_multiple_classes() {
    // Test that multiple classes from <style> work together
    let comp = html! {
        <div class={container}>
            <div class={header}>"Header"</div>
            <div class={content}>"Content"</div>
        </div>
        <style>
            .container { padding: "1rem"; }
            .header { font-weight: "bold"; }
            .content { color: "blue"; }
        </style>
    };
    let output = test::render(&comp);

    // All classes should be present
    assert!(output.contains("class=\"container\""));
    assert!(output.contains("class=\"header\""));
    assert!(output.contains("class=\"content\""));
    assert!(output.contains("data-s"));
}

// ════════════════════════════════════════════════════════════════════════════
// Pseudo-elements (::before, ::after)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pseudo_element_before() {
    let comp = html! {
        <div class={tooltip}>"Text"</div>
        <style>
            .tooltip::before { content: "→"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("::before"),
        "::before pseudo-element should be preserved. Got: {}",
        output
    );
}

#[test]
fn test_pseudo_element_after() {
    let comp = html! {
        <div class={arrow}>"Item"</div>
        <style>
            .arrow::after { content: "↦"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("::after"),
        "::after pseudo-element should be preserved. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Pseudo-classes (:hover, :focus)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pseudo_class_hover() {
    let comp = html! {
        <button class={btn}>"Click"</button>
        <style>
            .btn:hover { background: "blue"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains(":hover"),
        ":hover pseudo-class should be preserved. Got: {}",
        output
    );
}

#[test]
fn test_pseudo_class_focus() {
    let comp = html! {
        <input class={field} type="text" />
        <style>
            .field:focus { border: "2px solid blue"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains(":focus"),
        ":focus pseudo-class should be preserved. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Attribute selectors ([attr], [attr=value])
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_attribute_selector_present() {
    let comp = html! {
        <input type="text" />
        <style>
            [type] { border: "1px"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("[type]"),
        "Attribute selector [type] should be preserved. Got: {}",
        output
    );
}

#[test]
fn test_attribute_selector_exact_match() {
    let comp = html! {
        <input type="password" />
        <style>
            [type="password"] { color: "red"; }
        </style>
    };
    let output = test::render(&comp);
    let has_selector = output.contains("[type=") && output.contains("password]");
    assert!(
        has_selector,
        "Attribute selector should be preserved. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Combinators (> child, + adjacent, ~ general sibling)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_combinator_child() {
    let comp = html! {
        <div class={wrapper}>
            <span>"Child"</span>
        </div>
        <style>
            .wrapper > span { color: "blue"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("> span") || output.contains(">span"),
        "Child combinator > should be preserved. Got: {}",
        output
    );
}

#[test]
fn test_combinator_adjacent_sibling() {
    let comp = html! {
        <div class={item}>"First"</div>
        <div class={item}>"Second"</div>
        <style>
            .item + .item { margin-top: "1rem"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("+ .item") || output.contains("+.item"),
        "Adjacent sibling combinator + should be preserved. Got: {}",
        output
    );
}

#[test]
fn test_combinator_general_sibling() {
    let comp = html! {
        <h1 class={title}>"Title"</h1>
        <p class={para}>"Para"</p>
        <style>
            .title ~ .para { color: "gray"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("~ .para") || output.contains("~.para"),
        "General sibling combinator ~ should be preserved. Got: {}",
        output
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Multiple pseudo-classes (:hover:focus)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_multiple_pseudo_classes() {
    let comp = html! {
        <button class={btn}>"Submit"</button>
        <style>
            .btn:hover:focus { background: "green"; }
        </style>
    };
    let output = test::render(&comp);
    assert!(
        output.contains(":hover") && output.contains(":focus"),
        "Both pseudo-classes should be preserved. Got: {}",
        output
    );
}
