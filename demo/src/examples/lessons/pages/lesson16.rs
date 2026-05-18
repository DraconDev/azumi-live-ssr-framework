use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;
use sqlx::{Pool, Sqlite};
use std::sync::OnceLock;

/// Lesson 16: Async Database
///
/// SQLite integration with optimistic UI

// Global DB pool for the demo
static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

/// Initialize the DB (called from main.rs)
pub async fn init_db() -> Pool<Sqlite> {
    let pool = Pool::<Sqlite>::connect("sqlite::memory:").await.unwrap();

    // Create table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Seed data
    sqlx::query("INSERT INTO todos (text) VALUES ('Learn Azumi'), ('Build a lovely app')")
        .execute(&pool)
        .await
        .unwrap();

    let _ = DB_POOL.set(pool.clone());
    pool
}

#[azumi::live]
pub struct DatabaseTodo {
    pub todos: Vec<TodoItem>,
    pub input: String,
    pub loading: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoItem {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}

#[azumi::live_impl(component = "database_todo_view")]
impl DatabaseTodo {
    /// Async Action: Load from DB
    pub async fn load_todos(&mut self) {
        self.loading = true;
        // Optimistic update not needed for load

        if let Some(pool) = DB_POOL.get() {
            // Real Async Query!
            let rows = sqlx::query_as::<_, TodoItem>("SELECT * FROM todos ORDER BY id DESC")
                .fetch_all(pool)
                .await;

            if let Ok(items) = rows {
                self.todos = items;
            }
        }

        self.loading = false;
    }

    /// Async Action: Add Todo
    pub async fn add_todo(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let text = self.input.clone();

        // 1. Optimistic Update (Instant feedback)
        // We use a temporary negative ID that will be replaced by the server re-render
        self.todos.insert(
            0,
            TodoItem {
                id: -1,
                text: text.clone(),
                completed: false,
            },
        );
        self.input.clear();

        // 2. Real Async DB Insert
        if let Some(pool) = DB_POOL.get() {
            // Artificial delay to show off the optimistic UI vs server consistency
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            let _ = sqlx::query("INSERT INTO todos (text) VALUES (?)")
                .bind(text)
                .execute(pool)
                .await;

            // 3. Re-fetch to get correct ID and state
            self.load_todos().await;
        }
    }

    /// Async Action: Toggle Todo
    pub async fn toggle_todo(&mut self) {
        // NOTE: In a real app we'd pass the ID as a param, but for this demo
        // we'll rely on the client knowing which item was clicked via a comprehensive state update
        // or just toggle the first one for simplicity?
        //
        // Actually, Azumi Live's current demo structure often passes the *entire* state.
        // To toggle a specific item, we need to know WHICH one.
        // The current macro limitations mean we often just mutate the global live struct.
        //
        // Ideally, we'd have `fn toggle(&mut self, id: i64)`.
        // Since we don't have argument support in macros yet, we will skip toggle for this simple demo
        // or implement a "Clear All" instead which is easier.
    }

    pub async fn clear_all(&mut self) {
        // Optimistic
        self.todos.clear();

        // Async content
        if let Some(pool) = DB_POOL.get() {
            let _ = sqlx::query("DELETE FROM todos").execute(pool).await;
        }
    }
}

#[azumi::component]
pub fn Lesson16Page<'a>(state: &'a DatabaseTodo) -> impl Component + 'a {
    html! {
        @DarkModernLayout() {
            @database_todo_view(state=state)
        }
    }
}

#[azumi::component]
pub fn database_todo_view<'a>(state: &'a DatabaseTodo) -> impl Component + 'a {
    html! {
        <div class={card}>
            <h2 class={header_title}>"Async SQLite Todo List"</h2>

            <form class={input_group} on:submit={state.add_todo}>
                <input
                    class={input}
                    type="text"
                    name="input"
                    value={state.input}
                    placeholder="Add persistent todo..."
                    autofocus
                />

                <button class={btn} type="submit" disabled={state.loading}>
                    @if state.loading {
                        <span>"Saving..."</span>
                    } else {
                        <span>"Add Task"</span>
                    }
                </button>
            </form>

            <ul class={list}>
                @for todo in &state.todos {
                    <li class={if todo.id == -1 { "item item_optimistic" } else { "item" }}>
                        <span>
                            {if todo.id == -1 { "⏳ " } else { "✅ " }}
                            {&todo.text}
                        </span>
                        <span class={meta}>"ID: " {todo.id}</span>
                    </li>
                }
            </ul>

            @if state.todos.is_empty() {
                <div class={empty_state}>
                    "No tasks in SQLite database yet."
                </div>
            }

            <div class={action_bar}>
                <button class={format!("{} {}", btn, btn_danger)} on:click={state.clear_all}>
                    "Clear Database"
                </button>
            </div>

            @LessonNav(
                prev_num=Some(15),
                next_num=Some(17),
                prev_title="Full Application",
                next_title="Testing",
            )
        </div>

        <style>
             .card {
                background: "rgba(30, 41, 59, 0.7)";
                backdrop-filter: "blur(12px)";
                -webkit-backdrop-filter: "blur(12px)";
                border: "1px solid rgba(255, 255, 255, 0.1)";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
                padding: "2rem";
                border-radius: "1rem";
                max-width: "600px";
                margin: "4rem auto";
            }
            .header_title {
                text-align: "center";
                color: "#f8fafc";
                font-size: "1.875rem";
                font-weight: "700";
                margin-bottom: "1.5rem";
                letter-spacing: "-0.025em";
            }
            .input_group {
                display: "flex";
                gap: "0.75rem";
                margin-bottom: "2rem";
                align-items: "center";
            }
            .input {
                flex: "1 1 0";
                min-width: "0";
                padding: "0.75rem 1rem";
                background: "rgba(15, 23, 42, 0.6)";
                border: "1px solid rgba(255, 255, 255, 0.1)";
                border-radius: "0.5rem";
                color: "#e2e8f0";
                font-family: "inherit";
                font-size: "0.95rem";
                transition: "all 0.2s";
            }
            .input:focus {
                outline: "none";
                border-color: "#3b82f6";
                box-shadow: "0 0 0 2px rgba(59, 130, 246, 0.2)";
                background: "rgba(15, 23, 42, 0.8)";
            }
            .btn {
                padding: "0.75rem 1.5rem";
                background: "#3b82f6";
                color: "white";
                border: "none";
                border-radius: "0.5rem";
                font-weight: "500";
                cursor: "pointer";
                transition: "all 0.2s";
                display: "flex";
                align-items: "center";
                gap: "0.5rem";
            }
            .btn:hover {
                background: "#2563eb";
                transform: "translateY(-1px)";
            }
            .btn:disabled {
                background: "#475569";
                cursor: "not-allowed";
                transform: "none";
                opacity: "0.7";
            }
            .list {
                list-style: "none";
                padding: "0";
                margin: "0";
                display: "flex";
                flex-direction: "column";
                gap: "0.5rem";
            }
            .item {
                padding: "1rem";
                background: "rgba(255, 255, 255, 0.03)";
                border: "1px solid rgba(255, 255, 255, 0.05)";
                border-radius: "0.5rem";
                display: "flex";
                justify-content: "space-between";
                align-items: "center";
                color: "#cbd5e1";
                transition: "all 0.2s";
            }
            .item:hover {
                 background: "rgba(255, 255, 255, 0.05)";
            }
            .item_optimistic {
                opacity: "0.5";
                background: "rgba(59, 130, 246, 0.1)";
                border-color: "rgba(59, 130, 246, 0.2)";
            }
            .meta {
                font-size: "0.75rem";
                color: "#64748b";
                font-family: "ui-monospace, monospace";
            }
            .empty_state {
                text-align: "center";
                padding: "3rem 1rem";
                color: "#64748b";
                font-style: "italic";
            }
            .action_bar {
                margin-top: "2rem";
                display: "flex";
                justify-content: "flex-end";
                border-top: "1px solid rgba(255, 255, 255, 0.1)";
                padding-top: "1rem";
            }
            .btn_danger {
                background: "transparent";
                color: "#ef4444";
                border: "1px solid rgba(239, 68, 68, 0.2)";
            }
            .btn_danger:hover {
                background: "rgba(239, 68, 68, 0.1)";
                border-color: "#ef4444";
            }
        </style>
    }
}

pub async fn lesson16_handler() -> axum::response::Html<String> {
    // Ensure DB is init
    if DB_POOL.get().is_none() {
        init_db().await;
    }

    let mut state = DatabaseTodo {
        todos: vec![],
        input: String::new(),
        loading: false,
    };

    // Initial fetch
    state.load_todos().await;

    // Use the page component wrapper
    // Lesson16Page is a module generated by the macro

    // RENDER THE PAGE WRAPPER, NOT THE VIEW DIRECTLY
    let component_html = azumi::render_to_string(&Lesson16Page::render(
        Lesson16Page::Props::builder()
            .state(&state)
            .build()
            .expect("props"),
    ));

    axum::response::Html(component_html)
}
