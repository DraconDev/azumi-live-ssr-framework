use std::sync::Mutex;
use axum::{
    response::{Html, IntoResponse, Redirect},
    Form,
};
use azumi::{component, html, routes};
use serde::Deserialize;

// ── Todo Data ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    done: bool,
}

struct AppState {
    todos: Mutex<Vec<TodoItem>>,
    next_id: Mutex<usize>,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: Mutex::new(vec![
                TodoItem { id: 1, text: "Learn Azumi".into(), done: true },
                TodoItem { id: 2, text: "Build a real app".into(), done: false },
                TodoItem { id: 3, text: "Publish to crates.io".into(), done: false },
            ]),
            next_id: Mutex::new(4),
        }
    }

    fn add(&self, text: String) {
        let mut todos = self.todos.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();
        todos.push(TodoItem { id: *next_id, text, done: false });
        *next_id += 1;
    }

    fn toggle(&self, id: usize) {
        let mut todos = self.todos.lock().unwrap();
        if let Some(item) = todos.iter_mut().find(|t| t.id == id) {
            item.done = !item.done;
        }
    }

    fn delete(&self, id: usize) {
        let mut todos = self.todos.lock().unwrap();
        todos.retain(|t| t.id != id);
    }

    fn list(&self) -> Vec<TodoItem> {
        self.todos.lock().unwrap().clone()
    }
}

// ── Form Data ────────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
struct AddTodoForm {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ToggleForm {
    id: usize,
}

#[derive(Deserialize, Debug)]
struct DeleteForm {
    id: usize,
}

// ── Shared State ─────────────────────────────────────────────────────────

use std::sync::OnceLock;
fn global_state() -> &'static AppState {
    static STATE: OnceLock<AppState> = OnceLock::new();
    STATE.get_or_init(AppState::new)
}

// ── Components ───────────────────────────────────────────────────────────

#[component]
fn TodoPage() -> impl azumi::Component {
    let items = global_state().list();
    let pending = items.iter().filter(|t| !t.done).count();
    let done = items.iter().filter(|t| t.done).count();

    html! {
        <section class={"todo_wrap"}>
            <div class={"todo_inner"}>
                <header class={"vstack-05"}>
                    <h1 class={"todo_title"}>"Todo"</h1>
                    <p class={"todo_subtitle"}>{pending}" tasks remaining, "{done}" completed"</p>
                </header>

                <form class={"vstack-05"} method="POST" action="/add" az-target="#todo_list">
                    <div class={"todo_input_row"}>
                        <input
                            type="text"
                            name="text"
                            class={"todo_input"}
                            placeholder="What needs to be done?"
                            required
                        />
                        <button type="submit" class={"todo_add_btn"}>"Add"</button>
                    </div>
                </form>

                <div id={"todo_list"} class={"vstack-025"}>
                    @for item in items.iter() {
                        <div class={"todo_item"} data-done={item.done}>
                            <form method="POST" action="/toggle" class={"todo_toggle_form"} az-swap="outerHTML">
                                <input type="hidden" name="id" value={item.id.to_string()} />
                                <button type="submit" class={"todo_check"}>
                                    @if item.done {
                                        <span class={"todo_check_done"}>"✓"</span>
                                    } @else {
                                        <span class={"todo_check_empty"}>"○"</span>
                                    }
                                </button>
                            </form>
                            <span class={"todo_text"}>
                                {item.text.clone()}
                            </span>
                            <form method="POST" action="/delete" class={"todo_delete_form"} az-confirm="Delete this task?">
                                <input type="hidden" name="id" value={item.id.to_string()} />
                                <button type="submit" class={"todo_delete_btn"}>"×"</button>
                            </form>
                        </div>
                    }
                </div>
            </div>
        </section>
        <style global>
            .todo_wrap {
                display: "flex";
                justify-content: "center";
                padding: "4rem 1.5rem";
                min-height: "100vh";
                background: "var(--bg_primary)";
            }
            .todo_inner {
                width: "100%";
                max-width: "520px";
            }
            .todo_title {
                font-size: "2.5rem";
                font-weight: "900";
                letter-spacing: "-0.04em";
                line-height: "1.05";
                margin: "0";
            }
            .todo_subtitle {
                color: "var(--text_muted)";
                font-size: "0.9rem";
                margin: "0";
            }
            .todo_input_row {
                display: "flex";
                gap: "0.5rem";
            }
            .todo_input {
                flex: "1";
                background: "var(--bg_elevated)";
                border: "1px solid var(--border)";
                border-radius: "8px";
                padding: "0.75rem 1rem";
                color: "var(--text_primary)";
                font-size: "0.95rem";
                outline: "none";
            }
            .todo_input:focus {
                border-color: "var(--accent)";
            }
            .todo_add_btn {
                background: "var(--accent)";
                color: "var(--text_on_accent)";
                border: "none";
                border-radius: "8px";
                padding: "0.75rem 1.25rem";
                font-weight: "700";
                font-size: "0.9rem";
                cursor: "pointer";
            }
            .todo_add_btn:hover {
                filter: "brightness(0.92)";
            }
            .todo_item {
                display: "flex";
                align-items: "center";
                gap: "0.75rem";
                padding: "0.75rem 0";
                border-bottom: "1px solid var(--border)";
            }
            .todo_item[data-done="true"] .todo_text {
                text-decoration: "line-through";
                opacity: "0.4";
            }
            .todo_text {
                flex: "1";
                font-size: "0.95rem";
                color: "var(--text_primary)";
            }
            .todo_check {
                background: "none";
                border: "none";
                font-size: "1.2rem";
                cursor: "pointer";
                padding: "0";
                color: "var(--accent)";
            }
            .todo_check_empty { color: "var(--text_muted)"; }
            .todo_delete_btn {
                background: "none";
                border: "none";
                font-size: "1.1rem";
                cursor: "pointer";
                color: "var(--text_muted)";
                padding: "0.25rem";
                opacity: "0";
                transition: "opacity 0.15s";
            }
            .todo_item:hover .todo_delete_btn { opacity: "1"; }
            .todo_delete_btn:hover { color: "var(--danger)"; }
            .todo_toggle_form, .todo_delete_form { display: "inline"; margin: "0"; }
            input[type="hidden"] { display: "none"; }
        </style>
    }
}

// ── Handlers ─────────────────────────────────────────────────────────────

async fn page_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&TodoPage()))
}

async fn add_handler(Form(form): Form<AddTodoForm>) -> impl IntoResponse {
    let text = form.text.trim().to_string();
    if !text.is_empty() {
        global_state().add(text);
    }
    let items = global_state().list();
    let html = render_todo_list(&items);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], html)
}

async fn toggle_handler(Form(form): Form<ToggleForm>) -> impl IntoResponse {
    global_state().toggle(form.id);
    let items = global_state().list();
    let html = render_todo_list(&items);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], html)
}

async fn delete_handler(Form(form): Form<DeleteForm>) -> impl IntoResponse {
    global_state().delete(form.id);
    let items = global_state().list();
    let html = render_todo_list(&items);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], html)
}

fn render_todo_list(items: &[TodoItem]) -> String {
    let mut out = String::new();
    for item in items {
        let check = if item.done { "<span class=\"todo_check_done\">\u{2713}</span>" } else { "<span class=\"todo_check_empty\">\u{25CB}</span>" };
        let done_attr = if item.done { "true" } else { "false" };
        out.push_str(&format!(
            r#"<div class="todo_item" data-done="{done_attr}"><form method="POST" action="/toggle" class="todo_toggle_form" az-swap="outerHTML"><input type="hidden" name="id" value="{}"/><button type="submit" class="todo_check">{}</button></form><span class="todo_text">{}</span><form method="POST" action="/delete" class="todo_delete_form" az-confirm="Delete this task?"><input type="hidden" name="id" value="{}"/><button type="submit" class="todo_delete_btn">\u{00D7}</button></form></div>"#,
            item.id, check, item.text, item.id
        ));
    }
    if items.is_empty() {
        out.push_str(r#"<p style="color:var(--text_muted);padding:1rem 0;text-align:center">Nothing yet — add your first task above.</p>"#);
    }
    out
}

// ── App ──────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let app = routes! {
        "/"     => page_handler,
        "/add"  => add_handler,
        "/toggle" => toggle_handler,
        "/delete" => delete_handler,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Todo app running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
