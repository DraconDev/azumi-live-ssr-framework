use azumi::html;

fn main() {
    let x = "world";
    let _component = html! {
        @{Raw(format!("<div>{}</div>", x))}
    };
}
