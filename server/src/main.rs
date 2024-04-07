use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct TodoCreateInput {
    title: String,
    completed: bool,
}

async fn todos_create(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(input): Json<TodoCreateInput>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let id = state.id + 1;
    state.id = id;

    let todo = Todo {
        id: state.id,
        title: input.title,
        completed: input.completed,
    };

    state.todos.insert(id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}

async fn todos_delete(
    State(state): State<Arc<Mutex<AppState>>>,
    path: Path<u32>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.todos.remove(&path.0);
    StatusCode::NO_CONTENT
}

async fn todos(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    Json(state.lock().unwrap().todos.clone())
}

struct AppState {
    todos: HashMap<u32, Todo>,
    id: u32,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        todos: HashMap::new(),
        id: 0,
    }));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(todos).post(todos_create))
        .route("/todos/:id", delete(todos_delete))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
