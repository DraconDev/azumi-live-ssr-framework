use azumi::html;

fn main() {
    let dangerous = "<script>alert('xss')</script>";
    let _ = html! {
        @match Some(1) {
            Some(_) => { @{azumi::Raw(dangerous)} }
            None => {},
        }
    };
}
