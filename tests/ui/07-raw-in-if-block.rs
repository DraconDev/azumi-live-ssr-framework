use azumi::html;

fn main() {
    let condition = true;
    let _component = html! {
        <div>
            @if condition {
                @{Raw("test")}
            }
        </div>
    };
}