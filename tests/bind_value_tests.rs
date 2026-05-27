use azumi::{component, html, render_to_string};

// ═════════════════════════════════════════════════════
// bind:value Two-Way Binding Tests
// ═════════════════════════════════════════════════════

#[test]
fn test_bind_value_generates_data_bind_value_attribute() {
    #[azumi::component]
    fn my_input(value: &str) -> impl azumi::Component {
        html! {
            <input type="text" bind:value={value} />
        }
    }
    let output = render_to_string(&my_input::render(
        my_input::Props::builder().value("hello").build().unwrap(),
    ));
    assert!(
        output.contains("data-bind-value="),
        "Should generate data-bind-value attribute. Got: {}",
        output
    );
    assert!(
        output.contains("type=\"text\""),
        "Should preserve type attribute"
    );
}

#[test]
fn test_bind_checked_generates_data_bind_value_attribute() {
    #[azumi::component]
    fn my_checkbox(checked: bool) -> impl azumi::Component {
        html! {
            <input type="checkbox" bind:checked={checked} />
        }
    }
    let output = render_to_string(&my_checkbox::render(
        my_checkbox::Props::builder().checked(true).build().unwrap(),
    ));
    assert!(
        output.contains("data-bind-value="),
        "bind:checked should also generate data-bind-value. Got: {}",
        output
    );
    assert!(output.contains("type=\"checkbox\""));
}

#[test]
fn test_bind_value_with_nested_path() {
    #[azumi::component]
    fn user_form(name: &str, email: &str) -> impl azumi::Component {
        html! {
            <div>
                <input type="text" bind:value={name} />
                <input type="email" bind:value={email} />
            </div>
        }
    }
    let output = render_to_string(&user_form::render(
        user_form::Props::builder()
            .name("Alice")
            .email("alice@example.com")
            .build()
            .unwrap(),
    ));
    // Should have two data-bind-value attributes
    let count = output.matches("data-bind-value=").count();
    assert_eq!(count, 2, "Should have 2 data-bind-value attributes. Got: {}", output);
}

#[test]
fn test_bind_value_in_component() {
    #[azumi::component]
    fn form_input(value: &str) -> impl azumi::Component {
        html! {
            <input type="text" bind:value={value} />
        }
    }
    let output = render_to_string(&form_input::render(
        form_input::Props::builder().value("test").build().unwrap(),
    ));
    assert!(
        output.contains("data-bind-value="),
        "bind:value should emit data-bind-value. Got: {}",
        output
    );
}

#[test]
fn test_bind_value_preserves_other_attributes() {
    #[azumi::component]
    fn styled_input(value: &str) -> impl azumi::Component {
        html! {
            <input type="email" class:external="form-input" placeholder="you@example.com" required bind:value={value} />
        }
    }
    let output = render_to_string(&styled_input::render(
        styled_input::Props::builder()
            .value("test@test.com")
            .build()
            .unwrap(),
    ));
    assert!(output.contains("data-bind-value="));
    assert!(output.contains("type=\"email\""));
    assert!(output.contains("class=\"form-input\""));
    assert!(output.contains("placeholder=\"you@example.com\""));
    assert!(output.contains("required"));
}

#[test]
fn test_bind_value_on_select_element() {
    #[azumi::component]
    fn my_select(choice: &str) -> impl azumi::Component {
        html! {
            <select bind:value={choice}>
                <option value="rust">"Rust"</option>
                <option value="go">"Go"</option>
            </select>
        }
    }
    let output = render_to_string(&my_select::render(
        my_select::Props::builder().choice("rust").build().unwrap(),
    ));
    assert!(
        output.contains("data-bind-value="),
        "bind:value should work on select. Got: {}",
        output
    );
}

#[test]
fn test_bind_value_on_textarea() {
    #[azumi::component]
    fn my_textarea(content: &str) -> impl azumi::Component {
        html! {
            <textarea bind:value={content}></textarea>
        }
    }
    let output = render_to_string(&my_textarea::render(
        my_textarea::Props::builder().content("hello world").build().unwrap(),
    ));
    assert!(
        output.contains("data-bind-value="),
        "bind:value should work on textarea. Got: {}",
        output
    );
}

#[test]
fn test_bind_value_multiple_inputs() {
    #[azumi::component]
    fn login_form(username: &str, password: &str) -> impl azumi::Component {
        html! {
            <form>
                <input type="text" bind:value={username} />
                <input type="password" bind:value={password} />
            </form>
        }
    }
    let output = render_to_string(&login_form::render(
        login_form::Props::builder()
            .username("admin")
            .password("secret")
            .build()
            .unwrap(),
    ));
    let count = output.matches("data-bind-value=").count();
    assert_eq!(count, 2, "Both inputs should have data-bind-value. Got: {}", output);
}
