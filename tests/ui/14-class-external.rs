use azumi::prelude::*;

fn main() {
    // Static class="..." is banned
    let _ = html! { <div class="foo"></div> };
}