use azumi::html;

fn main() {
    let id = 42;
    let name = "user";
    let _component = html! {
        <div>
            <p>{format!("{}: {}", name, id)}</p>
        </div>
    };
}