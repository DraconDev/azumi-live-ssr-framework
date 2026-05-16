/// Compile-fail tests for Azumi compile-time validations.
///
/// These verify that invalid patterns (Raw(), format! with web content)
/// produce compile errors, and that valid patterns compile successfully.
#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    // Negative tests: these should fail to compile
    t.compile_fail("tests/ui/01-raw-blocked.rs");
    t.compile_fail("tests/ui/02-raw-format-blocked.rs");
    t.compile_fail("tests/ui/03-format-html-blocked.rs");
    t.compile_fail("tests/ui/04-format-js-blocked.rs");
    t.compile_fail("tests/ui/05-format-css-blocked.rs");
    t.compile_fail("tests/ui/07-raw-in-if-block.rs");
    t.compile_fail("tests/ui/09-raw-azumi-path.rs");
    t.compile_fail("tests/ui/11-raw-in-for.rs");
    t.compile_fail("tests/ui/12-raw-in-match.rs");
    t.compile_fail("tests/ui/13-raw-in-attribute.rs");
    t.compile_fail("tests/ui/14-class-static-banned.rs");

    // Positive test: safe macros should compile
    t.pass("tests/ui/06-macros-work.rs");
    t.pass("tests/ui/08-safe-format.rs");
}
