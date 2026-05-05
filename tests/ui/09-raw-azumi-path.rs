use azumi::html;

fn main() {
    let _component = html! {
        <div>
            @{azumi::Raw("test")}
        </div>
    };
}