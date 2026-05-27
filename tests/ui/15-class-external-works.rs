/// Positive test: class:external should compile and render correctly.
use azumi::html;

fn main() {
    // class:external for third-party component CSS (CMS widgets, payment forms, etc.)
    let _component = html! { <div class:external="payment-widget cms-card"></div> };
    // id:external should also work for external IDs
    let _component = html! { <div id:external="my-id"></div> };
}