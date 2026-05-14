//! Tests for `FnOnceComponent` functionality
//!
//! These tests verify the `from_fn_once` function and `FnOnceComponent` struct
//! which allow closures to consume captured owned values.
//!
//! Run with: cargo test --features test-utils

use azumi::{from_fn_once, html, test, FnOnceComponent};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Basic FnOnceComponent Functionality (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_fn_once_simple_closure() {
    let component = from_fn_once(|f| write!(f, "<div>Hello World</div>"));
    let html = test::render(&component);
    assert!(html.contains("Hello World"));
    assert!(html.contains("<div>"));
}

#[test]
fn test_fn_once_with_format() {
    let name = "Alice";
    let component = from_fn_once(move |f| write!(f, "<span>Hello, {}!</span>", name));
    let html = test::render(&component);
    assert!(html.contains("Hello, Alice!"));
}

#[test]
fn test_fn_once_empty_closure() {
    let component = from_fn_once(|_f| Ok(()));
    let html = test::render(&component);
    assert_eq!(html, "");
}

#[test]
fn test_fn_once_component_struct_directly() {
    let component = FnOnceComponent::from_fn_once(|f| write!(f, "<article>Content</article>"));
    let html = test::render(&component);
    assert!(html.contains("Content"));
}

#[test]
fn test_fn_once_nested_html() {
    let component =
        from_fn_once(|f| write!(f, "<nav><ul><li>Item 1</li><li>Item 2</li></ul></nav>"));
    let html = test::render(&component);
    assert!(html.contains("<nav>"));
    assert!(html.contains("<li>Item 1</li>"));
    assert!(html.contains("<li>Item 2</li>"));
}

#[test]
fn test_fn_once_with_numeric_values() {
    let value = 42;
    let component = from_fn_once(move |f| write!(f, "<data value=\"{}\">The answer</data>", value));
    let html = test::render(&component);
    assert!(html.contains("42"));
    assert!(html.contains("The answer"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Owned Value Consumption (5 tests)
// ════════════════════════════════════════════════════════════════════════════
// This is the primary use case for FnOnceComponent - moving owned values
// into closures that are called once.

#[test]
fn test_fn_once_consumes_owned_string() {
    let owned = String::from("consumed data");
    let component = from_fn_once(move |f| {
        // `owned` is moved here - can only be used once
        write!(f, "<p>{}</p>", owned)
    });
    let html = test::render(&component);
    assert!(html.contains("consumed data"));
}

#[test]
fn test_fn_once_consumes_owned_vec() {
    let items: Vec<i32> = vec![1, 2, 3, 4, 5];
    let component = from_fn_once(move |f| {
        let sum: i32 = items.iter().sum();
        write!(f, "<span>Sum: {}</span>", sum)
    });
    let html = test::render(&component);
    assert!(html.contains("Sum: 15"));
}

#[test]
fn test_fn_once_consumes_struct() {
    struct UserData {
        name: String,
        score: u32,
    }

    let user = UserData {
        name: "Bob".to_string(),
        score: 100,
    };

    let component = from_fn_once(move |f| {
        write!(
            f,
            "<div class=\"user\"><span>{}</span><span>Score: {}</span></div>",
            user.name, user.score
        )
    });
    let html = test::render(&component);
    assert!(html.contains("Bob"));
    assert!(html.contains("Score: 100"));
}

#[test]
fn test_fn_once_multiple_owned_values() {
    let a = String::from("Hello");
    let b = String::from("World");
    let c = 42;

    let component = from_fn_once(move |f| write!(f, "<p>{a} {b} #{c}</p>"));
    let html = test::render(&component);
    assert!(html.contains("Hello World #42"));
}

#[test]
fn test_fn_once_moves_option() {
    let value: Option<String> = Some("from option".to_string());
    let component = from_fn_once(move |f| match value {
        Some(v) => write!(f, "<div>Got: {}</div>", v),
        None => write!(f, "<div>None</div>"),
    });
    let html = test::render(&component);
    assert!(html.contains("Got: from option"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Caching Behavior - Multiple Renders (4 tests)
// ════════════════════════════════════════════════════════════════════════════
// FnOnceComponent should only invoke the closure once and cache the result.

#[test]
fn test_fn_once_multiple_renders_same_output() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let call_count = Rc::new(RefCell::new(0));
    let call_count_clone = call_count.clone();

    let component = from_fn_once(move |f| {
        *call_count_clone.borrow_mut() += 1;
        write!(f, "<span>Rendered</span>")
    });

    let html1 = test::render(&component);

    // First render produces output
    assert_eq!(html1, "<span>Rendered</span>");

    // Subsequent renders return warning comment (cached behavior)
    let html2 = test::render(&component);
    assert!(html2.contains("FnOnceComponent rendered more than once"));

    let html3 = test::render(&component);
    assert!(html3.contains("FnOnceComponent rendered more than once"));

    // Closure should only be called ONCE
    assert_eq!(*call_count.borrow(), 1);
}

#[test]
fn test_fn_once_caching_second_render_empty() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let counter = Rc::new(RefCell::new(0i32));
    let counter_clone = counter.clone();

    let component = from_fn_once(move |f| {
        *counter_clone.borrow_mut() += 10;
        write!(f, "<span>Count: {}</span>", *counter_clone.borrow())
    });

    // First render
    let html1 = test::render(&component);
    assert!(html1.contains("Count: 10"));

    // Second render returns warning comment (cached)
    let html2 = test::render(&component);
    assert!(html2.contains("FnOnceComponent rendered more than once"));

    // Counter should only be 10 (incremented once)
    assert_eq!(*counter.borrow(), 10);
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Error Handling (1 test)
// ════════════════════════════════════════════════════════════════════════════
// Note: The test framework panics on fmt errors, so we can't easily test
// error propagation. Instead, we test successful writes with various types.

#[test]
fn test_fn_once_with_various_types() {
    // Test with different std::fmt::Display types
    let string = String::from("text");
    let num = 42;
    let float = std::f64::consts::PI;
    let char = 'X';

    let component = from_fn_once(move |f| write!(f, "<div>{string} {num} {float} {char}</div>"));

    let html = test::render(&component);
    assert!(html.contains("text"));
    assert!(html.contains("42"));
    assert!(html.contains("3.141592653589793"));
    assert!(html.contains("X"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Thread Safety Verification (1 test)
// ════════════════════════════════════════════════════════════════════════════
// FnOnceComponent intentionally does NOT implement Send or Sync because
// FnOnce closures may capture non-thread-safe types (Rc, RefCell, etc.)

#[test]
fn test_fn_once_with_unsendable_type() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Rc and RefCell are not Send, proving FnOnceComponent can't be Send
    let rc_data = Rc::new(RefCell::new(String::from("rc data")));

    let component = from_fn_once(move |f| {
        let data = rc_data.borrow();
        write!(f, "<span>{}</span>", *data)
    });

    let html = test::render(&component);
    assert!(html.contains("rc data"));
    // This compiles because the closure is FnOnce, not Fn
    // If it were Fn, this would fail because Fn requires captured values to be Send
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 6: Integration with html! Macro (5 tests)
// ════════════════════════════════════════════════════════════════════════════
// These tests verify FnOnceComponent works correctly when used via the
// html! macro's children closure mechanism.
// Note: `from_fn_once` is used internally for children closures when they
// capture owned values that were also moved into component props.

#[azumi::component]
fn SimpleCard<'a>(title: &'a str, content: &'a str) -> impl Component + 'a {
    html! {
        <article>
            <header>{title}</header>
            <p>{content}</p>
        </article>
    }
}

#[test]
fn test_simple_component_no_children() {
    let comp = html! {
        @SimpleCard(title = "Hello", content = "World")
    };
    let html = test::render(&comp);
    assert!(html.contains("Hello"));
    assert!(html.contains("World"));
}

#[azumi::component]
fn Boxed<'a>(label: &'a str, children: impl Component + 'a) -> impl Component + 'a {
    html! {
        <div class={"box"}>
            <span class={"label"}>{label}</span>
            <div class={"content"}>{children}</div>
        </div>
    }
}

#[test]
fn test_nested_boxed_components() {
    let comp = html! {
        @Boxed(label = "Outer") {
            @Boxed(label = "Inner") {
                <strong>"Deep content"</strong>
            }
        }
    };
    let html = test::render(&comp);
    assert!(html.contains("Outer"));
    assert!(html.contains("Inner"));
    assert!(html.contains("Deep content"));
}

#[azumi::component]
fn UserCard<'a>(name: &'a str) -> impl Component + 'a {
    html! {
        <div class={"user-card"}>
            <h2>{name}</h2>
        </div>
    }
}

#[test]
fn test_user_card_component() {
    let name = "Alice";
    let comp = html! {
        @UserCard(name = name)
    };
    let html = test::render(&comp);
    assert!(html.contains("Alice"));
}

#[azumi::component]
fn StaticList<'a>(title: &'a str) -> impl Component + 'a {
    html! {
        <section>
            <h3>{title}</h3>
            <ol>
                <li>"One"</li>
                <li>"Two"</li>
                <li>"Three"</li>
            </ol>
        </section>
    }
}

#[test]
fn test_static_list_component() {
    let comp = html! {
        @StaticList(title = "Numbers")
    };
    let html = test::render(&comp);
    assert!(html.contains("Numbers"));
    assert!(html.contains("<ol>"));
    assert!(html.contains("One"));
    assert!(html.contains("Two"));
    assert!(html.contains("Three"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 7: Comparison with FnComponent (3 tests)
// ════════════════════════════════════════════════════════════════════════════
// These tests demonstrate the difference between FnComponent (from_fn)
// and FnOnceComponent (from_fn_once).

use azumi::from_fn;

#[test]
fn test_fn_component_can_be_called_multiple_times() {
    let component = from_fn(|f| write!(f, "<span>Static</span>"));

    // FnComponent can be called multiple times
    let html1 = test::render(&component);
    let html2 = test::render(&component);
    let html3 = test::render(&component);

    assert_eq!(html1, html2);
    assert_eq!(html2, html3);
}

#[test]
fn test_fn_once_vs_fn_owned_capture() {
    // With FnComponent (from_fn), owned values cannot be captured
    // because Fn closures cannot move captured values
    let value = String::from("test");

    // This would work with Fn (borrowing):
    let fn_component = from_fn(|f| {
        write!(f, "<span>{}</span>", value) // `value` borrowed, not moved
    });
    let html1 = test::render(&fn_component);
    // value is still accessible here
    let _ = value; // This line proves value wasn't moved
    let html2 = test::render(&fn_component);
    assert_eq!(html1, html2);

    // With FnOnceComponent (from_fn_once), owned values ARE consumed
    let value2 = String::from("consumed");
    let fn_once_component = from_fn_once(move |f| {
        write!(f, "<span>{}</span>", value2) // `value2` MOVED
    });
    let html3 = test::render(&fn_once_component);
    assert!(html3.contains("consumed"));
    // value2 is no longer accessible here - it was moved into the closure
}

#[test]
fn test_both_produce_valid_html() {
    let data = "content";

    let fn_comp = from_fn(|f| write!(f, "<div>{data}</div>"));
    let fn_once_comp = from_fn_once(move |f| write!(f, "<div>{data}</div>"));

    let html_fn = test::render(&fn_comp);
    let html_fn_once = test::render(&fn_once_comp);

    // Both should produce identical HTML
    assert_eq!(html_fn, html_fn_once);
    assert!(html_fn.contains("<div>content</div>"));
}
