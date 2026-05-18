//! Tests for borrowed (&T) parameter support in #[azumi::component]
//!
//! Verifies that components can use &str, &CustomStruct, etc. without
//! explicit lifetime annotations — the macro injects them automatically.

use azumi::prelude::*;

// ─── Test 1: Simple &str parameter ──────────────────────────────────────

#[azumi::component]
fn greeting(name: &str) -> impl Component {
    html! { <span>"Hello, " {name} "!"</span> }
}

#[test]
fn borrowed_str_component_renders() {
    let comp = greeting::render(
        greeting::Props::builder().name("Azumi").build().unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Hello, Azumi!"), "Expected greeting text, got: {}", html);
}

// ─── Test 2: Mixed &str + owned parameters ──────────────────────────────

#[azumi::component]
fn user_card(name: &str, age: i32) -> impl Component {
    html! { <div>{name} " is " {age} " years old"</div> }
}

#[test]
fn mixed_borrowed_and_owned_component_renders() {
    let comp = user_card::render(
        user_card::Props::builder().name("Alice").age(30).build().unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Alice"), "Expected name, got: {}", html);
    assert!(html.contains("30"), "Expected age, got: {}", html);
}

// ─── Test 3: Multiple &str parameters ───────────────────────────────────

#[azumi::component]
fn info_card(title: &str, description: &str) -> impl Component {
    html! {
        <div>
            <h3>{title}</h3>
            <p>{description}</p>
        </div>
    }
}

#[test]
fn multiple_borrowed_str_component_renders() {
    let comp = info_card::render(
        info_card::Props::builder()
            .title("Test Title")
            .description("Test Description")
            .build()
            .unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Test Title"), "Expected title, got: {}", html);
    assert!(html.contains("Test Description"), "Expected description, got: {}", html);
}

// ─── Test 4: &str with default value (static str coerces to &str) ───────

#[azumi::component]
fn labeled_value(#[prop(default = "\"N/A\"")] label: &str, value: i32) -> impl Component {
    html! { <div>{label} ": " {value}</div> }
}

#[test]
fn borrowed_str_with_default_uses_default() {
    let comp = labeled_value::render(
        labeled_value::Props::builder().value(42).build().unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("N/A"), "Expected default label, got: {}", html);
    assert!(html.contains("42"), "Expected value, got: {}", html);
}

#[test]
fn borrowed_str_with_default_uses_provided() {
    let comp = labeled_value::render(
        labeled_value::Props::builder().label("Count").value(7).build().unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Count"), "Expected provided label, got: {}", html);
    assert!(html.contains("7"), "Expected value, got: {}", html);
}

// ─── Test 5: &str with children (explicit lifetime for children) ────────

#[azumi::component]
fn titled_section<'a>(title: &'a str, children: impl Component + 'a) -> impl Component + 'a {
    html! {
        <section>
            <h2>{title}</h2>
            {children}
        </section>
    }
}

#[test]
fn borrowed_str_with_children_renders() {
    let comp = titled_section::render(
        titled_section::Props::builder().title("Section Title").build().unwrap(),
        html! { <p>"Section content"</p> },
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Section Title"), "Expected title, got: {}", html);
    assert!(html.contains("Section content"), "Expected children, got: {}", html);
}

// ─── Test 6: Borrowed struct parameter ──────────────────────────────────

struct Product {
    name: String,
    price: f64,
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (${:.2})", self.name, self.price)
    }
}

#[azumi::component]
fn product_display(product: &Product) -> impl Component {
    html! { <div>{product}</div> }
}

#[test]
fn borrowed_struct_parameter_renders() {
    let product = Product {
        name: "Widget".to_string(),
        price: 9.99,
    };
    let comp = product_display::render(
        product_display::Props::builder().product(&product).build().unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Widget"), "Expected product name, got: {}", html);
    assert!(html.contains("9.99"), "Expected price, got: {}", html);
}

// ─── Test 7: Backward compat — explicit lifetimes still work ─────────────

#[azumi::component]
fn explicit_lifetime_card<'a>(title: &'a str, content: String) -> impl Component + 'a {
    html! {
        <div>
            <h3>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

#[test]
fn explicit_lifetime_component_still_works() {
    let comp = explicit_lifetime_card::render(
        explicit_lifetime_card::Props::builder()
            .title("Explicit")
            .content("Works fine".to_string())
            .build()
            .unwrap(),
    );
    let html = render_to_string(&comp);
    assert!(html.contains("Explicit"), "Expected title, got: {}", html);
    assert!(html.contains("Works fine"), "Expected content, got: {}", html);
}
