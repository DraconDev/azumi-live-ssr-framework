use azumi::html;

fn main() {
    let json = "{\"key\": \"value\"}";
    let _component = html! {
        <p>{format!("window.data = {}", json)}</p>
    };
}
