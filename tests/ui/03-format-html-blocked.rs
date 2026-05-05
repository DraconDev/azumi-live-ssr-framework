use azumi::html;

fn main() {
    let value = "test";
    let _component = html! {
        <p>{format!("<div>{}</div>", value)}</p>
    };
}
