# Adding Azumi to Your Axum App

Azumi builds **on top of Axum**. You don't replace anything — add Azumi to one route at a time, keeping your existing Axum handlers alongside new `html!` components. Axum stays.

## The Spectrum

```
Plain Axum ──────> Azumi Components ──────> Azumi Actions/Live
   (start)           (add html!)            (add interactivity)
```

Every step is optional. Stop wherever you're comfortable.

---

## Step 1: Add Azumi Dependency

```toml
# Cargo.toml
[dependencies]
azumi = { version = "47", features = ["axum"] }
```

Keep all your existing code unchanged. Azumi doesn't take over your router.

---

## Step 2: Replace One Handler with `html!`

Pick a simple page — an "About" page, a settings page, anything with static HTML.

**Before (plain Axum):**
```rust
async fn about_page() -> impl IntoResponse {
    axum::response::Html(r#"
        <div class="about">
            <h1>About Us</h1>
            <p>We build things.</p>
        </div>
    "#.to_string())
}
```

**After (Azumi):**
```rust
#[azumi::component]
fn about_page() -> impl Component {
    html! {
        <div class={about}>
            <h1>"About Us"</h1>
            <p>"We build things."</p>
        </div>

        <style>
            .about {
                max-width: 800px;
                margin: 0 auto;
                padding: 2rem;
            }
        </style>
    }
}

async fn about_handler() -> impl IntoResponse {
    axum::response::Html(azumi::render_to_string(&about_page()))
}
```

**What changed:**
- HTML is now compile-time validated (missing closing tags, invalid attributes, XSS — all caught by the compiler)
- CSS is co-located and scoped automatically
- No more string formatting HTML

**Keep your existing handlers.** This one route is now Azumi; everything else stays the same.

---

## Step 3: Add `#[azumi::component]` for Repeated UI

When you find yourself writing the same HTML structure in multiple handlers, extract it into a component.

**Before (repeated card HTML):**
```rust
// In handler A:
axum::response::Html(format!(r#"<div class="card"><h2>{}</h2><p>{}</p></div>"#, title, body))

// In handler B:
axum::response::Html(format!(r#"<div class="card"><h2>{}</h2><p>{}</p></div>"#, other_title, other_body))
```

**After (Azumi component):**
```rust
#[azumi::component]
fn card(title: &str, body: &str) -> impl Component {
    html! {
        <div class={card_wrap}>
            <h2>{title}</h2>
            <p>{body}</p>
        </div>

        <style>
            .card_wrap {
                border: 1px solid #e0e0e0;
                border-radius: 8px;
                padding: 1.5rem;
            }
        </style>
    }
}

// Usage — no format! building HTML, no XSS risk
let comp = card::render(
    card::Props::builder().title("Hello").body("World").build().unwrap(),
);
```

**Key insight:** `&str` props accept references directly — no `.clone()` needed. See [Borrowed Parameters](#) in the guide.

---

## Step 4: Add `#[azumi::action]` for One Form

Replace one form submission with Azumi's action system. Actions handle form validation, error display, and DOM updates — all server-side, zero custom JavaScript.

**Before (manual form handling):**
```rust
async fn submit_contact(Form(data): Form<ContactForm>) -> impl IntoResponse {
    // Validate, process, return redirect or error page...
    // Where do errors go? How does the UI update?
}
```

**After (Azumi action):**
```rust
#[azumi::action]
pub async fn contact_action(form: ContactForm) -> ActionResult {
    if form.message.len() < 10 {
        return ActionResult::err("Message must be at least 10 characters");
    }
    // Process form...
    ActionResult::ok(&html! {
        <div style={success_style}>
            <strong>"Message sent!"</strong>
        </div>
    })
}
```

```rust
// In your page component:
html! {
    <form az-action={contact_action_PATH} az-target={"#contact-result"}>
        <input type="text" name="message" />
        <button type="submit">"Send"</button>
    </form>
    <div id="contact-result"></div>
}
```

**What you get:**
- Form submits via AJAX (no page reload)
- `az-target` specifies where the response goes
- Errors replace the target div automatically
- `contact_action_PATH` is auto-generated — no hardcoded URLs

---

## Step 5: Add `#[azumi::live]` for One Counter

Add client-side interactivity without JavaScript. Live components update the DOM in-place.

```rust
#[azumi::live]
struct Counter { count: i32 }

#[azumi::component]
fn counter(state: &Counter) -> impl Component {
    html! {
        <div>
            <span>{state.count}</span>
            <button az-on:click={format!("count:{}", state.count + 1)}>"+"</button>
            <button az-on:click={format!("count:{}", state.count - 1)}>"-"</button>
        </div>
    }
}
```

**What you get:**
- Button clicks trigger server round-trips
- The `az-scope` div automatically morphs with the new HTML
- No JavaScript written by you
- State is HMAC-signed (tamper-proof)

---

## Step 6: Replace `format!` + `Raw()` with Safe Patterns

If you're migrating from an older Azumi codebase or from a framework that encouraged string-building HTML:

| ❌ Old Pattern | ✅ Azumi Replacement |
|---------------|---------------------|
| `Raw(format!("<div>{}</div>", x))` | `html! { <div>{x}</div> }` |
| `format!("<script>var={}</script>", v)` | `json_data!("VAR" = &data)` |
| `format!("<style>.x{{color:{}}}</style>", c)` | `<style>{css_var}</style>` |
| Iteration with `format!` + `join()` | `@for item in &items { ... }` |
| `{user_input}` (unescaped) | `{user_input}` (auto-escaped in `html!`) |
| `Raw(cms_html)` | `TrustedHtml::new(cms_html)` |

---

## What NOT to Migrate

Not everything needs Azumi. These stay as plain Axum:

- **JSON API endpoints** — `axum::Json` is already perfect
- **WebSocket handlers** — Azumi doesn't replace WebSockets
- **File upload handlers** — Use Axum's `Multipart` directly
- **Authentication middleware** — Keep as Axum middleware
- **Health check / metrics endpoints** — Plain Axum is simpler

Azumi handles **HTML rendering and form interactivity**. Let Axum handle everything else.

---

## Common Gotchas

### `Box<dyn Component>` requires owned data

If you return `Box<dyn Component>`, borrowed references (`&str`) won't work because the box needs `'static` data. Either:
- Return `impl Component` instead (preferred)
- Pre-render to `String` with `render_to_string()` and use `TrustedHtml`

### `#[azumi::page]` vs Axum handlers

`#[azumi::page(route = "/path")]` works on **component functions** (returning `impl Component`), not on **Axum handler functions** (returning `impl IntoResponse`). For Axum handlers, define route constants manually:

```rust
// Component function — gets auto-constant
#[azumi::page(route = "/about")]
fn about_page() -> impl Component { ... }
// Generates: about_page_ROUTE == "/about"

// Axum handler — define constant manually
const CONTACT_ROUTE: &str = "/contact";
async fn contact_handler() -> impl IntoResponse { ... }
```

### Static attributes are banned

In `html!`, you can't write `class="foo"` or `style="color: red"`. Use the Azumi syntax:

```rust
// ✅ Azumi syntax
html! { <div class={my_class}>...</div> }
html! { <div style={--bg: "blue"}>...</div> }

// ❌ Banned (caught at compile time)
html! { <div class="my-class">...</div> }
html! { <div style="background: blue">...</div> }
```

For third-party CSS classes (Tailwind, Bootstrap), use `class:external`:

```rust
html! { <div class:external="container-fluid">...</div> }
```

---

## Migration Checklist

- [ ] Add `azumi` dependency with `features = ["axum"]`
- [ ] Replace one simple page with `html!` + component
- [ ] Move CSS into `<style>` blocks (auto-scoped)
- [ ] Extract repeated HTML into `#[azumi::component]`
- [ ] Replace one form with `#[azumi::action]`
- [ ] Add one `#[azumi::live]` interactive component
- [ ] Replace any `format!` + `Raw()` with safe patterns
- [ ] Define route constants for all `href` values
- [ ] Use `{&field}` instead of `.clone()` for `&self` fields
- [ ] Use `&str` props in components for zero-clone rendering
