use azumi::html;

fn main() {
    let color = "red";
    let _component = html! {
        <p>{format!(".btn {{ color: {}; }}", color)}</p>
    };
}
