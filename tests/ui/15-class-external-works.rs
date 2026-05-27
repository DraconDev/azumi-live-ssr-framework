/// Positive test: class:external should compile and render correctly.
use azumi::html;

fn main() {
    // class:external for third-party component CSS (CMS widgets, payment forms, etc.)
    let _ = html! { <div class:external="payment-widget cms-card"></div> };
    // id:external for external IDs
    let _ = html! { <div id:external="my-id"></div> };
}