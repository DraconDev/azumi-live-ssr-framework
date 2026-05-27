# Azumi — AI Developer Cookbook

> **For AI agents.** This is the step-by-step guide to writing Azumi apps. For rules and constraints, see [AGENTS.md](AGENTS.md). For detailed docs, see [docs/guide.md](docs/guide.md).

---

## Quick Start: The Complete App Skeleton

Every Azumi app follows this structure. Copy this pattern:

```rust
use axum::{Router, routing::get};
use azumi::prelude::*;

// 1. Define a page
#[azumi::page(route = "/")]
fn home_page() -> impl Component {
    html! {
        <div>
            <h1>"Hello Azumi"</h1>
        </div>
    }
}

// 2. Wire up the router
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(home_page_ROUTE, get(home_page));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**Pattern:** `#[azumi::page(route = "/path")]` → generates `PAGE_ROUTE` constant. Use it everywhere.

---

## Pattern 1: Build a Static Page

Use `html!` macro. Everything inside is auto-escaped. Control flow with `@if`/`@for`/`@match`/`@let`.

```rust
#[azumi::page(route = "/products")]
fn products_page(products: Vec<Product>) -> impl Component {
    html! {
        <div>
            <h1>"Products"</h1>
            @if products.is_empty() {
                <p>"No products found."</p>
            }
            @for product in &products {
                <div class={"product-card"}>
                    <h2>{&product.name}</h2>
                    <p>{&product.description}</p>
                    <span>{product.price}</span>
                </div>
            }
        </div>
    }
}
```

**Rules:**
- All text content must be double-quoted: `"hello"`, not bare text
- Static classes use variables: `class={my_class}`, not `class={"button"}` (bypasses validation)
- Use `@for item in &items` — borrow to avoid `.clone()`
- Use `{&field}` in text nodes — borrow instead of consuming

---

## Pattern 2: Create a Reusable Component

Use `#[azumi::component]`. Parameters become a builder pattern.

```rust
#[azumi::component]
fn alert(message: &str, #[prop(default = "\"info\"")] level: &str) -> impl Component {
    html! {
        <div class={level}>
            <p>{message}</p>
        </div>
    }
}

// Usage:
let comp = alert::render(
    alert::Props::builder()
        .message("Saved!")
        .build()
        .unwrap(),
);
```

**Rules:**
- Use `&str` for string props — avoids `.clone()` from callers
- Use `#[prop(default = "...")]` for optional props
- The `level` param defaults to `"info"` when omitted
- For children: add `children: impl Component` as last parameter

---

## Pattern 3: Build an Interactive Form

Use `az-action` + `az-target` for form submission. Add `bind:value` for two-way binding. Add `data-validate` for client-side validation.

```rust
#[azumi::component]
fn contact_form() -> impl Component {
    html! {
        <form az-action={"submit_contact"} az-target={"#contact-result"}>
            <div>
                <label for={"name"}>"Name"</label>
                <input type={"text"} id={"name"} name={"name"}
                       bind:value={""}
                       data-validate={"name:required,min-length:2"} />
                <p id={"name_error"} class={"form-error"} style={"display:none"}></p>
            </div>
            
            <div>
                <label for={"email"}>"Email"</label>
                <input type={"email"} id={"email"} name={"email"}
                       bind:value={""}
                       data-validate={"email:required,email"} />
                <p id={"email_error"} class={"form-error"} style={"display:none"}></p>
            </div>
            
            <div>
                <label for={"message"}>"Message"</label>
                <textarea id={"message"} name={"message"}
                          bind:value={""}
                          data-validate={"message:required,max-length:500"}>
                </textarea>
                <p id={"message_error"} class={"form-error"} style={"display:none"}></p>
            </div>
            
            <button type={"submit"}>"Send"</button>
        </form>
        <div id={"contact-result"}></div>
    }
}

// Server-side handler:
#[azumi::action]
fn submit_contact(form: Form<ContactData>) -> ActionResult {
    // Process form, return success or error fragment
    success_fragment(&html! { <p>"Message sent!"</p> })
}
```

**Rules:**
- Error messages appear in `id="{field}_error"` elements
- `data-validate` format: `"fieldname:rule1,rule2"`
- Available rules: `required`, `email`, `min-length:N`, `max-length:N`, `url`, `min:N`, `max:N`, `pattern:regex`
- `azar-action` on form = submit via fetch + morph result into `az-target`
- `bind:value={""}` starts with empty value, syncs on input

---

## Pattern 4: Build a Live Counter (Optimistic UI)

Use `#[azumi::live]` for reactive state. The compiler auto-detects predictions.

```rust
#[azumi::live]
struct Counter {
    count: i32,
}

#[azumi::live_impl]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[azumi::component]
fn counter_view(state: &Counter) -> impl Component {
    html! {
        <div>
            <h2>"Count: " <span data-bind={"count"}>{state.count}</span></h2>
            <button on:click={state.increment} data-predict={"count = count + 1"}>
                "+1"
            </button>
            <button on:click={state.reset} data-predict={"count = 0"}>
                "Reset"
            </button>
        </div>
    }
}
```

**Rules:**
- `#[azumi::live]` on struct — generates serialization + prediction metadata
- `#[azumi::live_impl]` on impl — auto-detects predictable mutations (toggle, inc, dec, set)
- `on:click={state.method}` — generates `az-on` attribute automatically
- `data-predict="field = expr"` — optimistic update before server confirms
- Component auto-wraps in `<div az-scope="...">` with HMAC-signed state

---

## Pattern 5: Keyed List with Transitions

Use `@keyed(id)` for smooth list updates. Add `az-transition:` for animations.

```rust
#[azumi::component]
fn task_list(tasks: Vec<Task>) -> impl Component {
    html! {
        <div>
            @for task in &tasks @keyed(task.id) {
                <div class={"task-row"} az-transition:fade={"true"}>
                    <span>{&task.title}</span>
                    <button az-confirm={"Delete this task?"}
                            az-action={delete_task_PATH}
                            az-target={"#tasks-container"}>
                        "Delete"
                    </button>
                </div>
            }
        </div>
    }
}
```

**Rules:**
- `@keyed(expr)` gives each list item a stable identity
- Without `@keyed`: every change destroys + recreates all DOM nodes (scroll resets, focus lost)
- With `@keyed`: items tracked by key, smooth morphing, scroll + focus preserved
- `az-transition:fade={"true"}` — elements fade in on enter, fade out on exit
- `az-transition:slide={"true"}` — elements slide open/closed
- `az-transition:scale={"true"}` — elements scale 0.95↔1.0 with opacity

---

## Pattern 6: Scoped CSS Per Component

Every `<style>` block is auto-scoped. No class name collisions between components.

```rust
#[azumi::component]
fn card(title: &str) -> impl Component {
    html! {
        <div class={"card"}>
            <h3 class={"title"}>{title}</h3>
            <p class={"body"}>"Card content here"</p>
        </div>
        <style>
            .card {
                border: "1px solid #e5e7eb";
                border-radius: "8px";
                padding: "1.5rem";
            }
            .title {
                font-size: "1.25rem";
                font-weight: "700";
            }
            .body {
                color: "var(--text-secondary)";
            }
        </style>
    }
}
// Output: .card[data-sabc123] { ... }   ← scoped!
//         <div class="card" data-sabc123="sabc123">
```

**Rules:**
- CSS values must be double-quoted: `"1rem"`, `"#e5e7eb"`, `"700"`
- CSS class names use snake_case: `my_button`, not `myButton` or `my-button`
- `style! global { ... }` — for unscoped global CSS
- `<style>{DYNAMIC_CSS}</style>` — inject pre-built CSS without scoping
- Use CSS custom properties (`var(--accent)`) for theme values

---

## Pattern 7: Layout with Chrome

Wrap pages in a layout component for shared header/footer.

```rust
#[azumi::component]
fn base_layout(children: impl Component, page_title: &str) -> impl Component {
    html! {
        <!DOCTYPE html>
        <html>
        <head>
            <title>{page_title}</title>
            <meta charset={"utf-8"} />
        </head>
        <body>
            <header>
                <nav>
                    <a href={home_page_ROUTE}>"Home"</a>
                    <a href={about_page_ROUTE}>"About"</a>
                </nav>
            </header>
            <main>
                {children}
            </main>
        </body>
        </html>
    }
}

// Usage in a page handler:
fn render_page() -> String {
    let content = html! { <div>"Page content"</div> };
    render_to_string(&base_layout::render(
        base_layout::Props::builder()
            .page_title("My Page")
            .build()
            .unwrap(),
        content,
    ))
}
```

**Rules:**
- Layout component takes `children: impl Component` as last param
- Pass children as second arg to `render()`, not via builder
- Use `render_to_string()` to get the final HTML string
- Use route constants (`home_page_ROUTE`) for all links

---

## Pattern 8: SEO Head Generation

Use `#[azumi::page]` for automatic title + description from doc comments. Use `context::set_page_meta()` for manual control.

```rust
/// Learn about our company and mission
#[azumi::page(route = "/about")]
fn about_page() -> impl Component {
    html! {
        <div>
            <h1>"About Us"</h1>
            <p>"We build things."</p>
        </div>
    }
}
// Auto-generates: <title>About Page</title>
//                 <meta name="description" content="Learn about our company and mission">
```

**Rules:**
- `/// Doc comment` on page function → `<meta description>`
- Function name `about_page` → title "About Page" (auto-capitalized)
- Use `#[azumi::page(route = "/path")]` for all pages

---

## Pattern 9: SSE Streaming

Stream HTML fragments to connected clients for real-time updates.

```rust
use azumi::streaming::{sse, SseEvent};

async fn event_stream() -> impl axum::response::IntoResponse {
    let stream = async_stream::stream! {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let count = get_latest_count();
            yield SseEvent::fragment(html! {
                <span id={"count"}>{count}</span>
            });
        }
    };
    sse(stream)
}
```

---

## Pattern 10: CSP (Content Security Policy)

```rust
use azumi::csp::{ContentSecurityPolicy, CspNonce};

let nonce = CspNonce::generate();
let csp = ContentSecurityPolicy::new()
    .azumi_nonce_defaults(&nonce)  // Allows azumi.js with nonce
    .script_src("'self'")          // Blocks inline scripts
    .style_src(&format!("'nonce-{}'", nonce.as_str()))
    .build();
```

---

## Common Mistakes AI Agents Make

| ❌ Wrong | ✅ Right | Why |
|----------|---------|-----|
| `class="button"` | `class={btn_class}` | Static strings banned. Use variable — validator checks it exists in `<style>`. |
| `class={"button"}` | `class={btn_class}` | String literal in braces bypasses compile-time class validation. |
| `style="color: red"` | Use `<style>` block | Inline styles banned. Scoped CSS only. |
| `<img src="...">` | `<img src={"..."} alt={"desc"} />` | `alt` required. Use braces. |
| `format!("<div>{}</div>", x)` in `html!` | Build outside, inject safely | `format!` + HTML blocked. |
| `Raw(html_string)` | `TrustedHtml::new(html_string)` | `Raw()` is banned. |
| `href="/about"` | `href={about_page_ROUTE}` | Route typo = 404 at runtime. |
| `{self.title.clone()}` | `{&self.title}` | Borrow instead of clone. |
| Text without quotes: `hello` | `"hello"` | All text must be double-quoted. |

---

## File-by-File Template

When generating a new Azumi project, create these files:

**`src/main.rs`** — router + entry point:
```rust
use axum::{Router, routing::get};

mod pages;
mod components;
mod actions;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(pages::home::home_page_ROUTE, get(pages::home::home_page))
        .route(pages::about::about_page_ROUTE, get(pages::about::about_page));

    azumi::action::register_actions(app)
        .await
        .unwrap()
        .serve("0.0.0.0:3000")
        .await
        .unwrap();
}
```

**`src/pages/home.rs`** — a page:
```rust
use azumi::prelude::*;

#[azumi::page(route = "/")]
pub fn home_page() -> impl Component {
    html! {
        <div>
            <h1>"Welcome"</h1>
        </div>
    }
}
```

**`src/components/button.rs`** — a reusable component:
```rust
use azumi::prelude::*;

#[azumi::component]
pub fn primary_button(label: &str, href: &str) -> impl Component {
    html! {
        <a href={href} class={"btn-primary"}>{label}</a>
    }
}
```

**`src/actions/contact.rs`** — a form action:
```rust
use azumi::prelude::*;
use axum::Form;

#[derive(serde::Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[azumi::action]
pub async fn submit_contact(form: Form<ContactForm>) -> ActionResult {
    // Process the form...
    success_fragment(&html! { <p>"Message sent!"</p> })
}
```

---

## When to Use Each Macro

| Situation | Macro |
|-----------|-------|
| "I need a page with a URL" | `#[azumi::page(route = "/path")]` |
| "I need a reusable UI piece" | `#[azumi::component]` |
| "I need a form handler" | `#[azumi::action]` |
| "I need reactive state" | `#[azumi::live]` + `#[azumi::live_impl]` |
| "I need to inject server data into JS" | `json_data!("VAR" = &data)` |
| "I need HTML from untrusted sources" | `{TrustedHtml::new(html)}` |

---

## When to Use Each Attribute

| Situation | Attribute |
|-----------|----------|
| "I need two-way input binding" | `bind:value={state.field}` |
| "I need checkbox/radio binding" | `bind:checked={state.field}` |
| "I need efficient list updates" | `@for item in items @keyed(item.id)` |
| "I need fade animation" | `az-transition:fade={"true"}` |
| "I need slide animation" | `az-transition:slide={"true"}` |
| "I need form validation" | `data-validate={"field:required,email"}` |
| "I need scroll reveal" | `az-reveal={true}` |
| "I need confirm dialog" | `az-confirm={"Are you sure?"}` |
| "I need optimistic UI" | `data-predict={"count = count + 1"}` |
| "I need a link (type-safe)" | `href={page_ROUTE}` |

---

*This cookbook is the authoritative AI guide for generating Azumi code. For safety rules and constraints, see [AGENTS.md](AGENTS.md). For detailed docs, see [docs/guide.md](docs/guide.md).*
