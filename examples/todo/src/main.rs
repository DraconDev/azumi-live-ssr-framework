use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use azumi::{component, html, render_to_string, Component};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

type TodoList = Arc<Mutex<Vec<String>>>;

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
fn Layout(children: impl Component) -> impl Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <title>"Azumi Todo"</title>
            <style global>
                :root {
                    --bg: "#0a0a0a";
                    --surface: "#141414";
                    --border: "rgba(255,255,255,0.08)";
                    --text: "#f0f0f0";
                    --text_dim: "#888";
                    --accent: "#7dd3fc";
                    --radius: "8px";
                }
                * { margin: "0"; padding: "0"; box-sizing: "border-box"; }
                body {
                    font-family: "Inter, system-ui, sans-serif";
                    background: "var(--bg)";
                    color: "var(--text)";
                    min-height: "100vh";
                    display: "flex";
                    align-items: "center";
                    justify-content: "center";
                }
                .card {
                    background: "var(--surface)";
                    border: "1px solid var(--border)";
                    border-radius: "var(--radius)";
                    padding: "2rem";
                    width: "min(90vw, 480px)";
                }
                h1 { font-size: "1.4rem"; font-weight: "800"; margin-bottom: "1.5rem"; }
                .form_row { display: "flex"; gap: "0.5rem"; margin-bottom: "1.25rem"; }
                input[type=text] {
                    flex: "1";
                    padding: "0.6rem 0.9rem";
                    border: "1px solid var(--border)";
                    border-radius: "var(--radius)";
                    background: "var(--bg)";
                    color: "var(--text)";
                    font-size: "0.9rem";
                }
                button {
                    padding: "0.6rem 1.2rem";
                    border: "none";
                    border-radius: "var(--radius)";
                    background: "var(--accent)";
                    color: "#000";
                    font-weight: "700";
                    font-size: "0.85rem";
                    cursor: "pointer";
                }
                button:hover { opacity: "0.85"; }
                .empty { color: "var(--text_dim)"; font-size: "0.85rem"; text-align: "center"; padding: "1.5rem 0"; }
                .list { display: "flex"; flex-direction: "column"; }
                .item {
                    display: "flex";
                    align-items: "center";
                    justify-content: "space-between";
                    padding: "0.65rem 0";
                    border-top: "1px solid var(--border)";
                    font-size: "0.92rem";
                }
                .item:first-child { border-top: "none"; }
                .delete_btn {
                    background: "none";
                    border: "none";
                    color: "var(--text_dim)";
                    cursor: "pointer";
                    font-size: "0.8rem";
                    padding: "0.2rem 0.4rem";
                }
                .delete_btn:hover { color: "#f87171"; }
            </style>
        </head>
        <body>
            <div class={"card"}>
                {children}
            </div>
        </body>
        </html>
    }
}

#[component]
fn TodoPage(todos: Vec<String>) -> impl Component {
    html! {
        <h1>"azumi todo"</h1>
        <form class={"form_row"} action="/add" method="POST">
            <input type="text" name="text" placeholder="What needs doing?" required />
            <button type="submit">"Add"</button>
        </form>
        <div class={"list"}>
            @if todos.is_empty() {
                <div class={"empty"}>"No todos yet. Add one above."</div>
            }
            @for (i, todo) in todos.iter().enumerate() {
                <div class={"item"}>
                    <span>{todo.clone()}</span>
                    <form action="/delete" method="POST" style="margin: 0">
                        <input type="hidden" name="index" value={i.to_string()} />
                        <button type="submit" class={"delete_btn"}>"✕"</button>
                    </form>
                </div>
            }
        </div>
    }
}

// ---------------------------------------------------------------------------
// Form payloads
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct AddForm {
    text: String,
}

#[derive(Deserialize)]
struct DeleteForm {
    index: usize,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn index(todos: State<TodoList>) -> impl IntoResponse {
    let list = todos.lock().await;
    Html(azumi::render_to_string(
        &Layout::render(
            Layout::Props::builder().build().expect("missing children"),
            TodoPage::render(TodoPage::Props {
                todos: list.clone(),
            }),
        ),
    ))
}

async fn add_todo(
    todos: State<TodoList>,
    Form(form): Form<AddForm>,
) -> impl IntoResponse {
    let text = form.text.trim().to_string();
    if !text.is_empty() {
        todos.lock().await.push(text);
    }
    Redirect::to("/")
}

async fn delete_todo(
    todos: State<TodoList>,
    Form(form): Form<DeleteForm>,
) -> impl IntoResponse {
    let mut list = todos.lock().await;
    if form.index < list.len() {
        list.remove(form.index);
    }
    Redirect::to("/")
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let todos: TodoList = Arc::new(Mutex::new(vec![
        "Learn Azumi".to_string(),
        "Build something cool".to_string(),
    ]));

    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_todo))
        .route("/delete", post(delete_todo))
        .with_state(todos);

    let addr = "0.0.0.0:8080";
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
