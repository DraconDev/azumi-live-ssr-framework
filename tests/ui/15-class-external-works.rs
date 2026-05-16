/// Positive test: class:external should compile and render correctly.
/// Positive test: class:external should compile.
use azumi::html;

fn main() {
    // class:external renders as class="..." (not class:external="...")
    // Multiple classes: bg-blue-500 px-4 are space-separated Tailwind utilities
    let _component = html! { <div class:external="bg-blue-500 px-4"></div> };
    // id:external should also work for external IDs
    let _component = html! { <div id:external="my-id"></div> };
}