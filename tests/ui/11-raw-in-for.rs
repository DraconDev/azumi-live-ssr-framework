use azumi::html;

fn main() {
    let dangerous = "<script>alert('xss')</script>";
    let _ = html! {
        @for item in [1, 2, 3] {
            @{azumi::Raw(dangerous)}
        }
    };
}
