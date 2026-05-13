use azumi::{component, html, azumi_script, routes};
use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared todo state
type TodoList = Arc<Mutex<Vec<TodoItem>>>;

#[derive(Clone, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

/// Root layout — HTML shell with global styles
#[component]
#[allow(non_snake_case)]
pub fn AppShell(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <title>"Azumi Todo"</title>
        </head>
        <body>
            <div class={app}>
                <header class={header}>
                    <h1 class={logo}>"azumi"</h1>
                    <p class={tagline}>"A todo app in Rust."</p>
                </header>
                <main class={main}>
                    {children}
                </main>
                {azumi_script()}
            </div>
            <style global>
                :root {
                    --bg: "#0f1117";
                    --surface: "#181b24";
                    --surface-2: "#1e2231";
                    --border: "rgba(255,255,255,0.06)";
                    --text: "#e1e4ea";
                    --text-dim: "#868b99";
                    --accent: "#7dd3fc";
                    --accent-dim: "rgba(125,211,252,0.12)";
                    --green: "#34d399";
                    --red: "#f87171";
                    --radius: "10px";
                    --font: "Inter, system-ui, -apple-system, sans-serif";
                }
                * { margin: "0"; padding: "0"; box-sizing: "border-box"; }
                body {
                    font-family: "var(--font)";
                    background: "var(--bg)";
                    color: "var(--text)";
                    line-height: "1.6";
                    min-height: "100vh";
                }
                .app { max-width: "640px"; margin: "0 auto"; padding: "3rem 1.5rem"; }
                .header { margin-bottom: "2.5rem"; }
                .logo {
                    font-size: "1.4rem";
                    font-weight: "800";
                    letter-spacing: "-0.03em";
                    color: "var(--accent)";
                    margin-bottom: "0.25rem";
                }
                .tagline { color: "var(--text-dim)"; font-size: "0.88rem"; }
                .main { display: "flex"; flex-direction: "column"; gap: "1.5rem"; }
            </style>
        </body>
        </html>
    }
}

/// The main todo page
#[component]
#[allow(non_snake_case)]
pub fn TodoPage(todos: Vec<TodoItem>) -> impl azumi::Component {
    let remaining = todos.iter().filter(|t| !t.done).count();
    html! {
        <section class={page}>

            <!-- Add todo form — uses az-action for server-roundtrip form handling -->
            <form class={add_form} az-action={"/todo/add"} az-target={"todo_list"} method="post">
                <div class={add_row}>
                    <input
                        class={add_input}
                        type="text"
                        name="text"
                        placeholder="Add a new task…"
                        required=true
                    />
                    <button class={add_btn} type="submit">"Add"</button>
                </div>
            </form>

            <!-- Stats bar -->
            <div class={stats}>
                <span>{remaining}</span><span class={stats_label}>" tasks remaining"</span>
            </div>

            <!-- Todo list — swapped by az-action responses -->
            <div id={"todo_list"} class={list}>
                @for todo in todos.iter() {
                    <div class={item}>
                        <div class={item_left}>
                            <!-- Toggle done/undone -->
                            <form az-action={"/todo/toggle"} az-target={"todo_list"} method="post">
                                <input type="hidden" name="id" value={todo.id.to_string()} />
                                <button class={if todo.done { check_done } else { check }} type="submit" aria-label="Toggle done">
                                    @if todo.done {
                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg>
                                    } @else {
                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/></svg>
                                    }
                                </button>
                            </form>
                            <span class={if todo.done { text_done } else { text }}>{todo.text.clone()}</span>
                        </div>
                        <div class={item_right}>
                            <!-- Delete -->
                            <form az-action={"/todo/delete"} az-target={"todo_list"} method="post">
                                <input type="hidden" name="id" value={todo.id.to_string()} />
                                <button class={delete_btn} type="submit" aria-label="Delete" az-confirm={"Delete this task?"}>{"✕"}</button>
                            </form>
                        </div>
                    </div>
                }
            </div>
        </section>
        <style>
            .page { display: "flex"; flex-direction: "column"; gap: "1.25rem"; }

            .add_form { display: "flex"; flex-direction: "column"; gap: "0.5rem"; }
            .add_row { display: "flex"; gap: "0.5rem"; }
            .add_input {
                flex: "1";
                padding: "0.7rem 1rem";
                border: "1px solid var(--border)";
                border-radius: "var(--radius)";
                background: "var(--surface)";
                color: "var(--text)";
                font-size: "0.9rem";
                outline: "none";
                transition: "border-color 0.15s";
            }
            .add_input:focus { border-color: "var(--accent)"; }
            .add_btn {
                padding: "0.7rem 1.25rem";
                border: "none";
                border-radius: "var(--radius)";
                background: "var(--accent)";
                color: "#000";
                font-weight: "700";
                font-size: "0.85rem";
                cursor: "pointer";
            }
            .add_btn:hover { opacity: "0.85"; }

            .stats { color: "var(--text-dim)"; font-size: "0.82rem"; font-weight: "700"; }
            .stats_label { font-weight: "400"; }

            .list { display: "flex"; flex-direction: "column"; gap: "0.35rem"; }
            .item {
                display: "flex";
                align-items: "center";
                justify-content: "space-between";
                padding: "0.65rem 0.9rem";
                background: "var(--surface)";
                border: "1px solid var(--border)";
                border-radius: "var(--radius)";
                transition: "border-color 0.15s";
            }
            .item:hover { border-color: "var(--accent-dim)"; }
            .item_left { display: "flex"; align-items: "center"; gap: "0.65rem"; flex: "1"; }
            .item_right { display: "flex"; align-items: "center"; }
            .check {
                background: "none";
                border: "none";
                color: "var(--text-dim)";
                cursor: "pointer";
                padding: "0.2rem";
                display: "flex";
            }
            .check:hover { color: "var(--accent)"; }
            .check_done {
                background: "none";
                border: "none";
                color: "var(--green)";
                cursor: "pointer";
                padding: "0.2rem";
                display: "flex";
            }
            .text { font-size: "0.9rem"; }
            .text_done { font-size: "0.9rem"; color: "var(--text-dim)"; text-decoration: "line-through"; }
            .delete_btn {
                background: "none";
                border: "none";
                color: "var(--text-dim)";
                cursor: "pointer";
                padding: "0.2rem 0.35rem";
                font-size: "0.8rem";
                opacity: "0";
                transition: "opacity 0.15s, color 0.15s";
            }
            .item:hover .delete_btn { opacity: "1"; }
            .delete_btn:hover { color: "var(--red)"; }
        </style>
    }
}

/// Action handlers — return HTML fragments for az-target swapping

#[azumi::action]
pub async fn handle_add(
    todos: axum::extract::State<TodoList>,
    form: axum::form::Form<AddForm>,
) -> impl axum::response::IntoResponse {
    let AddForm { text } = form.0;
    let text = text.trim().to_string();
    if text.is_empty() {
        return Html(String::new());
    }
    let mut list = todos.lock().await;
    let id = list.len() + 1;
    list.push(TodoItem { id, text, done: false });
    Html(render_todo_list(&list))
}

#[derive(serde::Deserialize)]
pub struct AddForm {
    pub text: String,
}

#[azumi::action]
pub async fn handle_toggle(
    todos: axum::extract::State<TodoList>,
    form: axum::form::Form<ToggleForm>,
) -> impl axum::response::IntoResponse {
    let mut list = todos.lock().await;
    if let Some(item) = list.iter_mut().find(|t| t.id == form.id) {
        item.done = !item.done;
    }
    Html(render_todo_list(&list))
}

#[derive(serde::Deserialize)]
pub struct ToggleForm {
    pub id: usize,
}

#[azumi::action]
pub async fn handle_delete(
    todos: axum::extract::State<TodoList>,
    form: axum::form::Form<DeleteForm>,
) -> impl axum::response::IntoResponse {
    let mut list = todos.lock().await;
    list.retain(|t| t.id != form.id);
    Html(render_todo_list(&list))
}

#[derive(serde::Deserialize)]
pub struct DeleteForm {
    pub id: usize,
}

/// Renders just the todo list fragment (for az-target swapping)
fn render_todo_list(todos: &[TodoItem]) -> String {
    // Render the list portion without the full page shell
    let list = {
        use azumi::html;
        let remaining = todos.iter().filter(|t| !t.done).count();
        html! {
            <div id={"todo_list"} class={list_style}>
                @for todo in todos.iter() {
                    <div class={item_style}>
                        <div class={item_left_style}>
                            <form az-action={"/todo/toggle"} az-target={"todo_list"} method="post">
                                <input type="hidden" name="id" value={todo.id.to_string()} />
                                <button class={if todo.done { check_done_style } else { check_style }} type="submit" aria-label="Toggle done">
                                    @if todo.done {
                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg>
                                    } @else {
                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/></svg>
                                    }
                                </button>
                            </form>
                            <span class={if todo.done { text_done_style } else { text_style }}>{todo.text.clone()}</span>
                        </div>
                        <div class={item_right_style}>
                            <form az-action={"/todo/delete"} az-target={"todo_list"} method="post">
                                <input type="hidden" name="id" value={todo.id.to_string()} />
                                <button class={delete_btn_style} type="submit" aria-label="Delete" az-confirm={"Delete this task?"}>{"✕"}</button>
                            </form>
                        </div>
                    </div>
                }
            </div>
            <style>
                .list_style { display: "flex"; flex-direction: "column"; gap: "0.35rem"; }
                .item_style {
                    display: "flex";
                    align-items: "center";
                    justify-content: "space-between";
                    padding: "0.65rem 0.9rem";
                    background: "var(--surface)";
                    border: "1px solid var(--border)";
                    border-radius: "var(--radius)";
                }
                .item_left_style { display: "flex"; align-items: "center"; gap: "0.65rem"; flex: "1"; }
                .item_right_style { display: "flex"; align-items: "center"; }
                .check_style { background: "none"; border: "none"; color: "var(--text-dim)"; cursor: "pointer"; padding: "0.2rem"; display: "flex"; }
                .check_style:hover { color: "var(--accent)"; }
                .check_done_style { background: "none"; border: "none"; color: "var(--green)"; cursor: "pointer"; padding: "0.2rem"; display: "flex"; }
                .text_style { font-size: "0.9rem"; color: "var(--text)"; }
                .text_done_style { font-size: "0.9rem"; color: "var(--text-dim)"; text-decoration: "line-through"; }
                .delete_btn_style { background: "none"; border: "none"; color: "var(--text-dim)"; cursor: "pointer"; padding: "0.2rem 0.35rem"; font-size: "0.8rem"; }
                .delete_btn_style:hover { color: "var(--red)"; }
            </style>
        }
    };
    azumi::render_to_string(&list)
}

/// Page handler — renders full page with shell
pub async fn page_handler(todos: axum::extract::State<TodoList>) -> impl axum::response::IntoResponse {
    let list = todos.lock().await;
    let page = TodoPage(todos: list.clone());
    let html = azumi::render_to_string(&AppShell(children: page));
    Html(html)
}

#[tokio::main]
async fn main() {
    let todos: TodoList = Arc::new(Mutex::new(Vec::new()));

    let app = routes! {
        "/" => get(page_handler),
        "/todo/add" => axum::routing::post(handle_add),
        "/todo/toggle" => axum::routing::post(handle_toggle),
        "/todo/delete" => axum::routing::post(handle_delete),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Azumi Todo → http://localhost:8080");
    axum::serve(listener, app.with_state(todos)).await.unwrap();
}
