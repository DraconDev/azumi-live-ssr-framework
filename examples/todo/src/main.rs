use axum::{response::Html, routing::get, Router};
use azumi::{component, html, render_to_string};
use std::sync::Arc;
use tokio::sync::Mutex;

/// A single todo item
#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    done: bool,
}

/// Simple in-memory todo store
type TodoStore = Arc<Mutex<Vec<TodoItem>>>;

/// Layout component with HTML shell, style, and script tags
#[component]
pub fn AppShell(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"Azumi Todo"</title>
            <style>
                :root {
                    --bg: "#0a0a0a";
                    --surface: "#141414";
                    --border: "rgba(255,255,255,0.08)";
                    --text: "#fafafa";
                    --text_muted: "#888";
                    --accent: "#7dd3fc";
                    --radius: "8px";
                }
                * { margin: "0"; padding: "0"; box-sizing: "border-box"; }
                body {
                    font-family: "system-ui, -apple-system, sans-serif";
                    background: "var(--bg)";
                    color: "var(--text)";
                    min-height: "100vh";
                    display: "flex";
                    justify-content: "center";
                    padding: "3rem 1rem";
                }
                .todo_app {
                    width: "100%";
                    max-width: "520px";
                    display: "flex";
                    flex-direction: "column";
                    gap: "1.5rem";
                }
                h1 { font-size: "1.5rem"; font-weight: "800"; letter-spacing: "-0.03em"; }
                .todo_form {
                    display: "flex";
                    gap: "0.5rem";
                }
                .todo_input {
                    flex: "1";
                    padding: "0.65rem 1rem";
                    background: "var(--surface)";
                    border: "1px solid var(--border)";
                    border-radius: "var(--radius)";
                    color: "var(--text)";
                    font-size: "0.9rem";
                    outline: "none";
                }
                .todo_input:focus { border-color: "var(--accent)"; }
                .todo_btn {
                    padding: "0.65rem 1.25rem";
                    background: "var(--accent)";
                    color: "#000";
                    border: "none";
                    border-radius: "var(--radius)";
                    font-weight: "700";
                    font-size: "0.85rem";
                    cursor: "pointer";
                }
                .todo_btn:hover { opacity: "0.85"; }
                .todo_list {
                    display: "flex";
                    flex-direction: "column";
                    gap: "0.35rem";
                    list-style: "none";
                }
                .todo_item {
                    display: "flex";
                    align-items: "center";
                    gap: "0.75rem";
                    padding: "0.75rem 1rem";
                    background: "var(--surface)";
                    border: "1px solid var(--border)";
                    border-radius: "var(--radius)";
                }
                .todo_item.done .todo_text {
                    text-decoration: "line-through";
                    color: "var(--text_muted)";
                }
                .todo_checkbox {
                    width: "18px";
                    height: "18px";
                    accent-color: "var(--accent)";
                    cursor: "pointer";
                }
                .todo_text { flex: "1"; font-size: "0.9rem"; }
                .todo_delete {
                    background: "none";
                    border: "none";
                    color: "var(--text_muted)";
                    cursor: "pointer";
                    font-size: "1.1rem";
                    padding: "0.2rem";
                    line-height: "1";
                }
                .todo_delete:hover { color: "#f87171"; }
                .empty_state {
                    text-align: "center";
                    color: "var(--text_muted)";
                    padding: "3rem 0";
                    font-size: "0.9rem";
                }
            </style>
        </head>
        <body>
            <div class={"todo_app"}>
                <h1>"Tasks"</h1>
                {children}
            </div>
        </body>
        </html>
    }
}

/// The todo list page component
#[component]
pub fn TodoPage(todos: Vec<TodoItem>) -> impl azumi::Component {
    html! {
        <form class={"todo_form"} action="/add" method="POST" az-target="#todo_list">
            <input class={"todo_input"} type="text" name="text" placeholder="Add a new task..." required />
            <button class={"todo_btn"} type="submit">"Add"</button>
        </form>

        <div id="todo_list" class={"vstack-035"}>
            @if todos.is_empty() {
                <p class={"empty_state"}>"No tasks yet. Add one above!"</p>
            }
            @for item in todos.iter() {
                @if item.done {
                    <div class={"todo_item done vstack-035"}>
                        <form action={format!("/toggle/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_checkbox"} type="submit" aria-label="Mark as incomplete">{"✓"}</button>
                        </form>
                        <span class={"todo_text"}>{item.text.clone()}</span>
                        <form action={format!("/delete/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_delete"} type="submit" aria-label="Delete task">{"✕"}</button>
                        </form>
                    </div>
                }
                @if !item.done {
                    <div class={"todo_item vstack-035"}>
                        <form action={format!("/toggle/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_checkbox"} type="submit" aria-label="Mark as complete"></button>
                        </form>
                        <span class={"todo_text"}>{item.text.clone()}</span>
                        <form action={format!("/delete/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_delete"} type="submit" aria-label="Delete task">{"✕"}</button>
                        </form>
                    </div>
                }
            }
        </div>
    }
}

/// Render the full page: shell layout + todo page inside
fn render_page(todos: &[TodoItem]) -> Html<String> {
    let body: String = {
        let page = html! { @TodoPage(todos: todos) };
        azumi::render_to_string(&page)
    };
    let shell = html! { @AppShell(children: body) };
    Html(azumi::render_to_string(&shell))
}

/// Homepage handler
async fn home_handler(store: axum::extract::State<TodoStore>) -> impl axum::response::IntoResponse {
    let todos = store.lock().await;
    render_page(&todos)
}

/// Add a new todo
async fn add_handler(
    axum::extract::State(store): axum::extract::State<TodoStore>,
    axum::extract::Form(form): axum::extract::Form<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    if let Some(text) = form.get("text") {
        if !text.trim().is_empty() {
            let mut todos = store.lock().await;
            let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            todos.push(TodoItem {
                id,
                text: text.trim().to_string(),
                done: false,
            });
        }
    }
    let todos = store.lock().await;
    // Return just the todo list fragment for az-target swap
    let fragment = render_todo_list(&todos);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], fragment)
}

/// Toggle a todo's done status
async fn toggle_handler(
    axum::extract::State(store): axum::extract::State<TodoStore>,
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> impl axum::response::IntoResponse {
    let mut todos = store.lock().await;
    if let Some(item) = todos.iter_mut().find(|t| t.id == id) {
        item.done = !item.done;
    }
    let fragment = render_todo_list(&todos);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], fragment)
}

/// Delete a todo
async fn delete_handler(
    axum::extract::State(store): axum::extract::State<TodoStore>,
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> impl axum::response::IntoResponse {
    let mut todos = store.lock().await;
    todos.retain(|t| t.id != id);
    let fragment = render_todo_list(&todos);
    ([(axum::http::header::CONTENT_TYPE, "text/html")], fragment)
}

/// Render just the todo list (for az-target fragment swapping)
fn render_todo_list(todos: &[TodoItem]) -> String {
    let fragment = html! {
        <div id="todo_list" class={"vstack-035"}>
            @if todos.is_empty() {
                <p class={"empty_state"}>"No tasks yet. Add one above!"</p>
            }
            @for item in todos.iter() {
                @if item.done {
                    <div class={"todo_item done vstack-035"}>
                        <form action={format!("/toggle/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_checkbox"} type="submit" aria-label="Mark as incomplete">{"✓"}</button>
                        </form>
                        <span class={"todo_text"}>{item.text.clone()}</span>
                        <form action={format!("/delete/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_delete"} type="submit" aria-label="Delete task">{"✕"}</button>
                        </form>
                    </div>
                }
                @if !item.done {
                    <div class={"todo_item vstack-035"}>
                        <form action={format!("/toggle/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_checkbox"} type="submit" aria-label="Mark as complete"></button>
                        </form>
                        <span class={"todo_text"}>{item.text.clone()}</span>
                        <form action={format!("/delete/{}", item.id)} method="POST" az-target="#todo_list">
                            <button class={"todo_delete"} type="submit" aria-label="Delete task">{"✕"}</button>
                        </form>
                    </div>
                }
            }
        </div>
    };
    azumi::render_to_string(&fragment)
}

#[tokio::main]
async fn main() {
    let store: TodoStore = Arc::new(Mutex::new(vec![
        TodoItem { id: 1, text: "Learn Azumi".into(), done: true },
        TodoItem { id: 2, text: "Build something cool".into(), done: false },
        TodoItem { id: 3, text: "Publish to crates.io".into(), done: false },
    ]));

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/add", axum::routing::post(add_handler))
        .route("/toggle/{id}", axum::routing::post(toggle_handler))
        .route("/delete/{id}", axum::routing::post(delete_handler))
        .route("/azumi.js", axum::routing::get(azumi_js))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Todo app running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

/// Serve the azumi client runtime
async fn azumi_js() -> impl axum::response::IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/javascript")],
        azumi::AZUMI_JS,
    )
}
