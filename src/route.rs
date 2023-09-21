use axum::{routing::get, Router};

use crate::{
    handlers::{create_todo_handler, get_todo_handler, health_checker_handler, todos_list_handler},
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/todos",
            get(todos_list_handler).post(create_todo_handler),
        )
        .route("/api/todos/:id", get(get_todo_handler))
        .with_state(db)
}
