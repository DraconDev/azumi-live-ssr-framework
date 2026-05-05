use azumi::{html, test};
#[test]
fn test_simple_inline_script() {
    let js = "console.log('hello');</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    eprintln!("SCRIPT OUTPUT: {:?}", output);
    assert!(!output.contains("</script>"), "Should not contain raw </script>");
    assert!(output.contains(r"<\/script>"), r"Should contain escaped <\/script>");
}
