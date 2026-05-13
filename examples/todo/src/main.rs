use azumi::{component, html, routes};
use axum::Router;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<TodoItem>>>,
    pub next_id: Arc<Mutex<usize>>,
}

impl AppState {
    pub fn new() -> Self {
        let todos = vec![
            TodoItem { id: 1, text: "Try Azumi".to_string(), done: true },
            TodoItem { id: 2, text: "Build something awesome".to_string(), done: false },
        ];
        Self {
            todos: Arc::new(Mutex::new(todos)),
            next_id: Arc::new(Mutex::new(3)),
        }
    }
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn Layout(title: String, children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{title}</title>
                {azumi::azumi_script()}
            </head>
            <body class={"body"}>
                <main class={"main"}>
                    {children}
                </main>
            </body>
        </html>
        <style>
            .body {
                font-family: "Inter, system-ui, sans-serif";
                background: "#0a0a0a";
                color: "#e4e4e7";
                margin: "0";
                min-height: "100vh";
            }
            .main {
                max-width: "640px";
                margin: "0 auto";
                padding: "3rem 1.5rem";
            }
            .todo_card {
                background: "#141414";
                border: "1px solid rgba(255,255,255,0.06)";
                border-radius: "12px";
                padding: "2rem";
            }
            .todo_title {
                font-size: "1.75rem";
                font-weight: "900";
                letter-spacing: "-0.03em";
                margin: "0 0 1.5rem";
            }
            .todo_form {
                display: "flex";
                gap: "0.75rem";
                margin-bottom: "1.5rem";
            }
            .todo_input {
                flex: "1";
                background: "#1a1a1a";
                border: "1px solid rgba(255,255,255,0.08)";
                border-radius: "8px";
                padding: "0.75rem 1rem";
                color: "#e4e4e7";
                font-size: "0.95rem";
                outline: "none";
            }
            .todo_input:focus {
                border-color: "#7dd3fc";
            }
            .todo_btn {
                background: "#7dd3fc";
                color: "#000";
                border: "none";
                border-radius: "8px";
                padding: "0.75rem 1.25rem";
                font-weight: "700";
                font-size: "0.9rem";
                cursor: "pointer";
            }
            .todo_btn:hover {
                filter: "brightness(0.92)";
            }
            .todo_list {
                list-style: "none";
                padding: "0";
                margin: "0";
            }
            .todo_item {
                display: "flex";
                align-items: "center";
                gap: "0.75rem";
                padding: "0.875rem 0";
                border-bottom: "1px solid rgba(255,255,255,0.04)";
            }
            .todo_item:last-child {
                border-bottom: "none";
            }
            .todo_check {
                appearance: "none";
                width: "20px";
                height: "20px";
                border: "2px solid rgba(255,255,255,0.15)";
                border-radius: "6px";
                cursor: "pointer";
                flex-shrink: "0";
            }
            .todo_check:checked {
                background: "#7dd3fc";
                border-color: "#7dd3fc";
            }
            .todo_text {
                flex: "1";
                font-size: "0.95rem";
            }
            .todo_text_done {
                text-decoration: "line-through";
                opacity: "0.5";
            }
            .todo_delete {
                background: "none";
                border: "none";
                color: "#f87171";
                font-size: "0.8rem";
                cursor: "pointer";
                opacity: "0.6";
            }
            .todo_delete:hover {
                opacity: "1";
            }
            .todo_empty {
                text-align: "center";
                color: "rgba(255,255,255,0.3)";
                padding: "2rem 0";
                font-size: "0.9rem";
            }
        </style>
    }
}

#[component]
pub fn TodoList(todos: Vec<TodoItem>) -> impl azumi::Component {
    html! {
        <div class={"todo_card"} id={"todo_list"}>
            <h1 class={"todo_title"}>"Todos"</h1>
            <form class={"todo_form"} action="/todos" az-action={true} az-target={"#todo_list"}>
                <input class={"todo_input"} type="text" name="text" placeholder="What needs doing?" required={true} autofocus={true}/>
                <button class={"todo_btn"} type="submit">"Add"</button>
            </form>
            @if todos.is_empty() {
                <p class={"todo_empty"}>"All done! Add a task above."</p>
            } else {
                <ul class={"todo_list"}>
                    @for todo in todos.iter() {
                        <li class={"todo_item"}>
                            <form action={format!("/todos/{}/toggle", todo.id)} az-action={true} az-target={"#todo_list"}>
                                <input class={"todo_check"} type="checkbox" name="done" checked={todo.done}/>
                            </form>
                            <span class={if todo.done { "todo_text todo_text_done" } else { "todo_text" }}>
                                {todo.text.clone()}
                            </span>
                            <form action={format!("/todos/{}", todo.id)} az-action={true} az-target={"#todo_list"} az-confirm={"Delete this task?"}>
                                <input type="hidden" name="_method" value="delete"/>
                                <button class={"todo_delete"} type="submit">"Delete"</button>
                            </form>
                        </li>
                    }
                </ul>
            }
        </div>
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_todos(state: axum::extract::State<AppState>) -> impl axum::response::IntoResponse {
    let todos = state.todos.lock().unwrap().clone();
    azumi::render_to_string(&Layout {
        title: "Todos".to_string(),
        children: TodoList { todos },
    })
}

async fn create_todo(
    state: axum::extract::State<AppState>,
    axum::extract::Form(form): axum::extract::Form<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    if let Some(text) = form.get("text").filter(|t| !t.trim().is_empty()) {
        let mut todos = state.todos.lock().unwrap();
        let mut next_id = state.next_id.lock().unwrap();
        todos.push(TodoItem {
            id: *next_id,
            text: text.trim().to_string(),
            done: false,
        });
        *next_id += 1;
    }
    let todos = state.todos.lock().unwrap().clone();
    azumi::render_to_string(&TodoList { todos })
}

async fn toggle_todo(
    state: axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> impl axum::response::IntoResponse {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = !todo.done;
    }
    let todos = todos.clone();
    azumi::render_to_string(&TodoList { todos })
}

async fn delete_todo(
    state: axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> impl axum::response::IntoResponse {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|t| t.id != id);
    let todos = todos.clone();
    azumi::render_to_string(&TodoList { todos })
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = routes! {
        "/" => list_todos,
        "/todos" => create_todo,
        "/todos/:id" => delete_todo,
        "/todos/:id/toggle" => toggle_todo,
    }
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Todo app running at http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}
