use azumi::html;
use azumi::Raw;

fn main() {
    let _component = html! {
        <div class={Raw("test")}></div>
    };
}
