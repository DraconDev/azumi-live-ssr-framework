use azumi::{html, test};

// Since I don't have paste crate available in the workspace easily without adding it,
// I will just write a macro that takes the name manually or just create them.
// Actually, I can use a simpler approach.

macro_rules! test_val {
    ($name:ident, $val:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let v = $val;
            let component = html! { <div>{v}</div> };
            let output = test::render(&component);
            assert!(
                output.contains($expected),
                "Failed {}: expected {}, got {}",
                stringify!($name),
                $expected,
                output
            );
        }
    };
}

// Integers
test_val!(test_i8, 42i8, "42");
test_val!(test_i8_neg, -42i8, "-42");
test_val!(test_u8, 255u8, "255");
test_val!(test_i16, 12345i16, "12345");
test_val!(test_u16, 65535u16, "65535");
test_val!(test_i32, 1000000i32, "1000000");
test_val!(test_u32, 4000000000u32, "4000000000");
test_val!(test_i64, 9000000000000i64, "9000000000000");
test_val!(test_u64, 18000000000000000000u64, "18000000000000000000");
test_val!(
    test_i128,
    170141183460469231731687303715884105727i128,
    "170141183460469231731687303715884105727"
);
test_val!(test_isize, 100isize, "100");
test_val!(test_usize, 200usize, "200");

// Floats
test_val!(test_f32, std::f32::consts::PI, "3.14");
test_val!(test_f64, std::f64::consts::E, "2.71828");

// Strings and Characters
test_val!(test_str, "hello", "hello");
test_val!(test_string, "world".to_string(), "world");
test_val!(test_char, 'Z', "Z");

// Escaping
test_val!(test_escape_lt, "<", "&lt;");
test_val!(test_escape_gt, ">", "&gt;");
test_val!(test_escape_amp, "&", "&amp;");
test_val!(test_escape_quot, "\"", "&quot;");

// Collections
#[test]
fn test_vec_render() {
    let items = vec![1, 2, 3];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let output = test::render(&component);
    assert!(output.contains("<li>1</li>"));
    assert!(output.contains("<li>2</li>"));
    assert!(output.contains("<li>3</li>"));
}

#[test]
fn test_empty_vec_render() {
    let items: Vec<i32> = vec![];
    let component = html! {
        <div>
            @if items.is_empty() {
                <span>"Empty"</span>
            }
            @for item in &items {
                <li>{item}</li>
            }
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains("<span>Empty</span>"));
}

// Deep Nesting
#[azumi::component]
fn Nest(depth: i32) -> impl Component {
    html! {
        <div>
            @if depth > 0 {
                @Nest(depth = depth - 1)
            } else {
                "Bottom"
            }
        </div>
    }
}

#[test]
fn test_deep_nesting() {
    // 20 levels deep
    let comp = html! { @Nest(depth = 20) };
    let output = test::render(&comp);
    // Should contain "Bottom" and many <div> tags
    assert!(output.contains("Bottom"));
    let div_count = output.matches("<div>").count();
    assert_eq!(div_count, 21); // 20 nested + the one around "Bottom"
}

// Multiple children permutations
#[azumi::component]
fn Layout<'a>(title: &'a str, children: impl Component + 'a) -> impl Component + 'a {
    // Avoid undefined class error - using a static class name literal
    html! {
        <section>
            <h1>{title}</h1>
            <div class={"content_class"}>
                {children}
            </div>
        </section>
        <style>
            .content_class { padding: "1rem"; }
        </style>
    }
}

#[test]
fn test_layout_multiple_children() {
    let comp = html! {
        @Layout(title = "Main") {
            <p>"First"</p>
            <p>"Second"</p>
            @Layout(title = "Sub") {
                "Inner"
            }
        }
    };
    let output = test::render(&comp);
    assert!(
        output.contains("Main") && output.contains("<h1"),
        "Output was: {}",
        output
    );
    assert!(output.contains("<p>First</p>"), "Output was: {}", output);
    assert!(output.contains("<p>Second</p>"), "Output was: {}", output);
    assert!(
        output.contains("Sub") && output.contains("<h1"),
        "Output was: {}",
        output
    );
    assert!(output.contains("Inner"), "Output was: {}", output);
}

// Complex Expressions in Props
#[test]
fn test_complex_expressions() {
    let a = 10;
    let b = 20;
    let component = html! {
        <div data-val={a + b}>
            {format!("Result: {}", a * b)}
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains("data-val=\"30\""));
    assert!(output.contains("Result: 200"));
}

// Enums
enum Status {
    Active,
    Inactive,
    Pending(String),
}

#[test]
fn test_enum_rendering() {
    let s1 = Status::Active;
    let s2 = Status::Inactive;
    let s3 = Status::Pending("Wait".to_string());

    let r1 = html! {
        <div>
            @match &s1 {
                Status::Active => { "ACTIVE" }
                Status::Inactive => { "INACTIVE" }
                Status::Pending(msg) => { "PENDING: " {msg} }
            }
        </div>
    };
    let r2 = html! {
        <div>
            @match &s2 {
                Status::Active => { "ACTIVE" }
                Status::Inactive => { "INACTIVE" }
                Status::Pending(msg) => { "PENDING: " {msg} }
            }
        </div>
    };
    let r3 = html! {
        <div>
            @match &s3 {
                Status::Active => { "ACTIVE" }
                Status::Inactive => { "INACTIVE" }
                Status::Pending(msg) => { "PENDING: " {msg} }
            }
        </div>
    };

    assert!(test::render(&r1).contains("ACTIVE"));
    assert!(test::render(&r2).contains("INACTIVE"));
    assert!(test::render(&r3).contains("PENDING: Wait"));
}
