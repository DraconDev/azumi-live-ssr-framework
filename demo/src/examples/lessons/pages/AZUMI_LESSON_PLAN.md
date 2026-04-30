# 🎓 Azumi Comprehensive Lesson Plan

## 🚀 Mastering Azumi: From Static Components to Live Reactive UI

This lesson plan covers Azumi from basics to advanced Azumi Live features, showcasing the full power of compiler-driven optimistic UI.

---

## Track 1: Foundation (Lessons 1-4)

### Lesson 1: Hello Azumi

**Goal:** Understand basic component structure

```rust
use azumi::html;

#[azumi::component]
fn hello_world() -> impl azumi::Component {
    html! {
        <style>
            .greeting { color: "#2196f3"; font-size: "2rem"; }
        </style>
        <div class={greeting}>"Hello, Azumi!"</div>
    }
}
```

**Key Concepts:**

-   `#[azumi::component]` macro creates type-safe components
-   `html!` macro for JSX-like syntax
-   `<style>` blocks are automatically scoped to component
-   Class names become Rust variables (`greeting`)

---

### Lesson 2: Props and Composition

**Goal:** Pass data between components

```rust
#[azumi::component]
fn card<'a>(title: &'a str, content: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style>
            .card { border: "1px solid #eee"; padding: "1rem"; border-radius: "8px"; }
            .card_title { font-weight: "bold"; color: "#333"; }
        </style>
        <div class={card}>
            <h3 class={card_title}>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

#[azumi::component]
fn dashboard() -> impl azumi::Component {
    html! {
        <div>
            @card(title="Welcome", content="Getting started with Azumi")
            @card(title="Features", content="Type-safe reactive components")
        </div>
    }
}
```

**Key Concepts:**

-   Props are function parameters with lifetimes
-   `@component()` syntax for child components
-   Named arguments: `title="value"`

---

### Lesson 3: Children Pattern

**Goal:** Build layout components with children

```rust
#[azumi::component]
fn container(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <style>
            .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
        </style>
        <div class={container}>
            {children}
        </div>
    }
}

#[azumi::component]
fn page() -> impl azumi::Component {
    html! {
        @container {
            <h1>"My Page"</h1>
            <p>"Content inside container"</p>
        }
    }
}
```

**Key Concepts:**

-   `children: impl Component` parameter
-   `@component { ... }` curly brace syntax for children
-   Layout composition patterns

---

### Lesson 4: Control Flow

**Goal:** Conditional and list rendering

```rust
#[azumi::component]
fn user_list(users: Vec<&str>, show_count: bool) -> impl azumi::Component {
    html! {
        <style>
            .user_item { padding: "0.5rem"; background: "#f5f5f5"; margin: "0.25rem 0"; }
            .count_badge { background: "#2196f3"; color: "white"; padding: "0.25rem 0.5rem"; }
        </style>
        <div>
            @if show_count {
                <span class={count_badge}>"Users: " {users.len()}</span>
            }

            @for user in &users {
                <div class={user_item}>{user}</div>
            }

            @if users.is_empty() {
                <p>"No users found"</p>
            }
        </div>
    }
}
```

**Key Concepts:**

-   `@if` / `@if !condition` for conditionals
-   `@for item in collection` for iteration
-   `@match` for pattern matching (not shown)
-   `@let` for local variables

---

## Track 2: Styling (Lessons 5-6)

### Lesson 5: CSS Scoping & Global Styles

**Goal:** Understand style isolation

```rust
#[azumi::component]
fn styling_demo() -> impl azumi::Component {
    html! {
        // Global styles - NOT scoped
        <style global>
            body { font-family: "Inter, sans-serif"; }
            .global_heading { color: "purple"; }
        </style>

        // Component styles - automatically scoped
        <style>
            .local_heading { color: "blue"; }
            .container { padding: "1rem"; }
        </style>

        <div class={container}>
            <h1 class={local_heading}>"Scoped (blue)"</h1>
            <h2 class="global_heading">"Global (purple)"</h2>
        </div>
    }
}
```

**Key Concepts:**

-   `<style>` = scoped to component (class becomes variable)
-   `<style global>` = not scoped (use string for class)
-   CSS validation at compile time
-   **No inline styles** - all styling via `<style>` blocks

---

### Lesson 6: @let Pattern & Dynamic CSS Variables

**Goal:** Use local variables and pass dynamic values to CSS

```rust
#[azumi::component]
fn progress_meter(completion: f64, accent_color: &str) -> impl azumi::Component {
    html! {
        <style>
            .meter {
                width: "100%";
                height: "20px";
                background: "#eee";
                border-radius: "10px";
                overflow: "hidden";
            }
            .fill {
                height: "100%";
                background: "var(--accent, #2196f3)";
                width: "calc(var(--progress) * 100%)";
                transition: "width 0.3s ease";
            }
            .label { text-align: "center"; margin-top: "0.5rem"; }
        </style>

        <div class={meter}>
            // style={} ONLY allows CSS custom properties (--variables)
            // Direct properties like style={width: "50%"} would be a compile error!
            <div class={fill} style={--progress: completion; --accent: accent_color}></div>
        </div>

        // @let for computed values
        @let percent = (completion * 100.0) as i32;
        <div class={label}>{percent}"% complete"</div>
    }
}
```

**Key Concepts:**

-   `@let name = expr;` for local variables inside templates
-   CSS custom properties in `<style>`: `--my-var: value;` then `var(--my-var)`
-   Pass dynamic values via `style={--var: value}`
-   **Only CSS variables allowed in style attribute** - direct properties cause compile errors

---

## Track 3: Interactivity (Lessons 7-8)

### Lesson 7: Event DSL Basics (`az-on`)

**Goal:** Add server-side interactivity

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CounterState { count: i32 }

#[azumi::component]
fn counter_view(state: CounterState) -> impl azumi::Component {
    html! {
        {azumi_script()}
        <style>
            .counter { text-align: "center"; padding: "2rem"; }
            .count { font-size: "3rem"; color: "#2196f3"; }
            .btn { padding: "1rem 2rem"; font-size: "1rem"; cursor: "pointer"; }
        </style>

        // az-scope defines reactive state boundary
        <div class={counter} az-scope={serde_json::to_string(&state).unwrap()}>
            <div class={count}>{state.count}</div>

            // az-on DSL: {event} call {action} -> {target}
            <button class={btn} az-on="click call increment -> this">
                "Increment"
            </button>
        </div>
    }
}
```

**Key Concepts:**

-   `az-scope={json}` stores component state
-   `az-on="{event} call {action}"` binds events to server actions
-   `-> this` / `-> #id` targets which scope to update
-   Requires `azumi.js` runtime

---

### Lesson 8: Server Actions

**Goal:** Define server-side mutation handlers

```rust
// State struct
#[derive(Serialize, Deserialize)]
struct TodoState {
    items: Vec<String>,
    input: String,
}

// Action handler (receives state, returns new HTML)
#[azumi::action]
async fn add_todo(mut state: TodoState) -> impl azumi::Component {
    if !state.input.is_empty() {
        state.items.push(state.input.clone());
        state.input.clear();
    }
    todo_list(state)  // Return updated component
}

// Component using the action
#[azumi::component]
fn todo_list(state: TodoState) -> impl azumi::Component {
    html! {
        <div az-scope={serde_json::to_string(&state).unwrap()}>
            <input type="text" name="input" value={state.input} />
            <button az-on="click call add_todo -> this">"Add"</button>

            @for item in &state.items {
                <div>{item}</div>
            }
        </div>
    }
}
```

**Key Concepts:**

-   `#[azumi::action]` registers server handler at `/_azumi/action/{name}`
-   Actions receive state JSON, return new HTML
-   DOM morphing preserves focus/scroll state

---

## Track 4: Azumi Live (Lessons 9-12)

### Lesson 9: Introducing Azumi Live

**Goal:** Understand compiler-driven optimistic UI

```rust
use azumi::prelude::*;

// #[azumi::live] auto-derives Serialize/Deserialize + LiveState trait
#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub active: bool,
}

// #[azumi::live_impl] analyzes mutations for predictions
#[azumi::live_impl(component = "counter_view")]
impl Counter {
    // Compiler detects: self.count += 1 → Prediction: "count += 1"
    pub fn increment(&mut self) {
        self.count += 1;
    }

    // Compiler detects: self.active = !self.active → Prediction: "active = !active"
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}
```

**Key Concepts:**

-   `#[azumi::live]` marks struct as reactive state
-   `#[azumi::live_impl]` analyzes mutations at compile time
-   Auto-generates prediction DSL from Rust code
-   Single source of truth - no JS duplication

---

### Lesson 10: Live Components with Auto-Detection

**Goal:** Let the `#[azumi::component]` macro detect live state and auto-inject predictions

```rust
// When first parameter is `state: &T`, component auto-detects live mode
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <style>
            .counter_box { padding: "2rem"; border: "1px solid #ccc"; border-radius: "8px"; }
            .value { font-size: "3rem"; font-weight: "bold"; color: "#2196f3"; }
        </style>

        // Auto-wrapped in az-scope div with az-predictions JSON!
        <div class={counter_box}>
            <div class={value}>{state.count}</div>
            <p>"Active: " {state.active}</p>

            // Predictions are auto-detected from #[azumi::live_impl]!
            // No data-predict needed for simple mutations
            <button on:click={state.increment}>
                "Increment"
            </button>
        </div>
    }
}
```

**Key Concepts:**

-   First parameter `state: &T` triggers live detection
-   Component auto-wraps in `<div az-scope="...">` with `az-predictions` JSON
-   `#[azumi::live_impl]` analyzes methods and stores predictions automatically
-   Manual `data-predict` only needed for complex mutations that can't be auto-detected

---

### Lesson 11: Declarative Event Binding (`on:event`)

**Goal:** Use the new concise event syntax

```rust
#[azumi::live]
pub struct LikeButton {
    pub liked: bool,
    pub count: i32,
}

#[azumi::live_impl(component = "view_name")]
impl LikeButton {
    pub fn toggle(&mut self) {
        self.liked = !self.liked;
        self.count += if self.liked { 1 } else { -1 };
    }
}

#[azumi::component]
pub fn like_view<'a>(state: &'a LikeButton) -> impl Component + 'a {
    html! {
        <style>
            .like_btn { padding: "1rem"; font-size: "1.5rem"; cursor: "pointer"; }
        </style>

        // NEW: on:event={state.method} syntax!
        // Auto-generates: az-on="click call toggle" data-predict="..."
        <button class={like_btn} on:click={state.toggle}>
            {if state.liked { "❤️" } else { "🤍" }}
            " " {state.count}
        </button>
    }
}
```

**Key Concepts:**

-   `on:click={state.method}` is the new declarative syntax
-   Auto-generates both `az-on` and `data-predict` attributes
-   Predictions come from `#[azumi::live_impl]` analysis
-   Zero boilerplate event binding!

---

### Lesson 12: How Optimistic UI Works

**Goal:** Understand the prediction → confirm flow

```
┌────────────────────────────────────────────────────────────────┐
│  User clicks button with on:click={state.increment}           │
├────────────────────────────────────────────────────────────────┤
│  1. OPTIMISTIC: Execute data-predict="count += 1" instantly   │
│  2. SERVER: POST /_azumi/action/increment with state JSON     │
│  3. MORPH: Server returns new HTML, morphed into DOM          │
│  4. RECONCILE: If prediction wrong, server HTML wins          │
└────────────────────────────────────────────────────────────────┘
```

**Supported Predictions:**

| Rust Pattern       | Generated Prediction |
| ------------------ | -------------------- |
| `self.x = !self.x` | `x = !x` (toggle)    |
| `self.x = true`    | `x = true` (literal) |
| `self.x += 1`      | `x += 1` (increment) |
| `self.x -= 1`      | `x -= 1` (decrement) |

**Why This Matters:**

-   0ms perceived latency (instant UI updates)
-   Server is source of truth (can't trust client)
-   No JavaScript to write or maintain
-   Compiler catches bugs at build time

---

## Track 5: Advanced Patterns (Lessons 13-15)

### Lesson 13: Form Handling

**Goal:** Build forms with manual state management

```rust
#[azumi::live]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
    pub submitted: bool,
}

#[azumi::live_impl(component = "contact_form_view")]
impl ContactForm {
    pub fn submit(&mut self) {
        // Validation would happen here
        self.submitted = true;
    }

    pub fn reset(&mut self) {
        self.name.clear();
        self.email.clear();
        self.message.clear();
        self.submitted = false;
    }
}

#[azumi::component]
pub fn contact_form<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {
        <style>
            .form { display: "grid"; gap: "1rem"; max-width: "400px"; }
            .field { display: "grid"; gap: "0.5rem"; }
            .input { padding: "0.5rem"; border: "1px solid #ddd"; }
            .btn { padding: "0.75rem"; background: "#2196f3"; color: "white"; border: "none"; }
            .success { color: "green"; font-weight: "bold"; }
        </style>

        <form class={form}>
            @if state.submitted {
                <div class={success}>"Thank you for your message!"</div>
                <button type="button" class={btn} on:click={state.reset}>"Send Another"</button>
            }

            @if !state.submitted {
                <div class={field}>
                    <label>"Name"</label>
                    <input class={input} type="text" name="name" value={state.name} />
                </div>
                <div class={field}>
                    <label>"Email"</label>
                    <input class={input} type="email" name="email" value={state.email} />
                </div>
                <div class={field}>
                    <label>"Message"</label>
                    <textarea class={input} name="message">{state.message}</textarea>
                </div>
                <button class={btn} type="button" on:click={state.submit}>"Submit"</button>
            }
        </form>
    }
}
```

---

### Lesson 14: Composition with Live Components

**Goal:** Compose multiple live components

```rust
#[azumi::live]
pub struct Tab {
    pub active_index: usize,
}

#[azumi::live_impl(component = "todo_list_view")]
impl Tab {
    pub fn select_0(&mut self) { self.active_index = 0; }
    pub fn select_1(&mut self) { self.active_index = 1; }
    pub fn select_2(&mut self) { self.active_index = 2; }
}

#[azumi::component]
pub fn tab_view<'a>(state: &'a Tab, tabs: Vec<&'a str>) -> impl Component + 'a {
    html! {
        <style>
            .tabs { display: "flex"; gap: "0.5rem"; }
            .tab { padding: "0.5rem 1rem"; cursor: "pointer"; border: "none"; background: "#eee"; }
            .tab_active { background: "#2196f3"; color: "white"; }
            .content { padding: "1rem"; border: "1px solid #ddd"; }
        </style>

        <div>
            <div class={tabs}>
                <button class={if state.active_index == 0 { "tab tab_active" } else { "tab" }}
                        on:click={state.select_0}>{tabs[0]}</button>
                <button class={if state.active_index == 1 { "tab tab_active" } else { "tab" }}
                        on:click={state.select_1}>{tabs[1]}</button>
                <button class={if state.active_index == 2 { "tab tab_active" } else { "tab" }}
                        on:click={state.select_2}>{tabs[2]}</button>
            </div>
            <div class={content}>
                @match state.active_index {
                    0 => { <p>"Content for tab 1"</p> }
                    1 => { <p>"Content for tab 2"</p> }
                    2 => { <p>"Content for tab 3"</p> }
                    _ => { <p>"Unknown tab"</p> }
                }
            </div>
        </div>
    }
}
```

---

### Lesson 15: Full Application Pattern

**Goal:** Build a complete interactive application

```rust
#[azumi::live]
pub struct App {
    pub todos: Vec<String>,
    pub input: String,
    pub filter: String,  // "all" | "completed" | "active"
}

#[azumi::live_impl]
impl App {
    pub fn add(&mut self) {
        if !self.input.is_empty() {
            self.todos.push(self.input.clone());
            self.input.clear();
        }
    }

    pub fn show_all(&mut self) { self.filter = "all".to_string(); }
    pub fn show_active(&mut self) { self.filter = "active".to_string(); }
}

#[azumi::component]
pub fn app_view<'a>(state: &'a App) -> impl Component + 'a {
    html! {
        <style>
            .app { max-width: "500px"; margin: "0 auto"; padding: "2rem"; }
            .header { font-size: "2rem"; color: "#2196f3"; text-align: "center"; }
            .input_row { display: "flex"; gap: "0.5rem"; }
            .input { flex: "1"; padding: "0.5rem"; }
            .btn { padding: "0.5rem 1rem"; background: "#4caf50"; color: "white"; border: "none"; }
            .filters { display: "flex"; gap: "0.5rem"; margin: "1rem 0"; }
            .filter_btn { padding: "0.5rem"; background: "#eee"; border: "none"; }
            .active { background: "#2196f3"; color: "white"; }
            .todo_item { padding: "0.5rem"; background: "#f5f5f5"; margin: "0.25rem 0"; }
        </style>

        <div class={app}>
            <h1 class={header}>"Azumi Todos"</h1>

            <div class={input_row}>
                <input class={input} placeholder="What needs to be done?" value={state.input} />
                <button class={btn} on:click={state.add}>"Add"</button>
            </div>

            <div class={filters}>
                <button class={if state.filter == "all" { "filter_btn active" } else { "filter_btn" }}
                        on:click={state.show_all}>"All"</button>
                <button class={if state.filter == "active" { "filter_btn active" } else { "filter_btn" }}
                        on:click={state.show_active}>"Active"</button>
            </div>

            @for todo in &state.todos {
                <div class={todo_item}>{todo}</div>
            }
        </div>
    }
}
```

---

## 🎯 Learning Path Summary

| Track             | Lessons | Focus                                        |
| ----------------- | ------- | -------------------------------------------- |
| **Foundation**    | 1-4     | Components, props, composition, control flow |
| **Styling**       | 5-6     | CSS scoping, @let pattern, CSS custom props  |
| **Interactivity** | 7-8     | `az-on` DSL, server actions                  |
| **Azumi Live**    | 9-12    | `#[azumi::live]`, predictions, `on:event`    |
| **Advanced**      | 13-15   | Forms, composition, full apps                |

---

## 🚀 Key Takeaways

1. **Single Source of Truth** - Write logic once in Rust, compiler generates client predictions
2. **Compile-Time Safety** - Macros catch errors before runtime
3. **Zero JavaScript (Almost)** - Optimistic UI without writing JS
4. **Progressive Enhancement** - Start static, add live features incrementally
5. **Declarative Events** - `on:click={state.method}` is all you need

---

## ⚠️ Constraints & Escape Hatches

**What Azumi Validates at Compile Time:**

-   All CSS (no external imports, no inline styles)
-   All HTML structure
-   All event bindings

**When You Might Need JavaScript:**

For rare client-only interactions like drag-and-drop, canvas drawing, or complex animations,
you can use external script files:

```rust
html! {
    <script src="/static/drag-handler.js"></script>
    <div id="draggable" data-draggable="true">
        "Drag me"
    </div>
}
```

But for 99% of cases, `on:event` + Azumi Live handles everything with zero JS.
