use goose::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Todo {
    id: u32,
}

struct Session {
    id: u32,
}

async fn todo_create(user: &mut GooseUser) -> TransactionResult {
    let todo = serde_json::json!({
        "title": "Learn Rust",
        "completed": false,
    });

    let mut goose_resp = user.post_json("/todos", &todo).await?;

    match goose_resp.response {
        Ok(resp) => match resp.json::<Todo>().await {
            Ok(todo) => {
                user.set_session_data(Session { id: todo.id });
                Ok(())
            }
            Err(err) => {
                return user.set_failure(
                    "create todo",
                    &mut goose_resp.request,
                    None,
                    Some(err.to_string().as_str()),
                );
            }
        },
        Err(err) => {
            return user.set_failure(
                "create todo",
                &mut goose_resp.request,
                None,
                Some(err.to_string().as_str()),
            );
        }
    }
}

async fn todo_delete(user: &mut GooseUser) -> TransactionResult {
    let session = user.get_session_data_unchecked::<Session>();
    let _ = user.delete(&format!("/todos/{}", session.id)).await?;

    Ok(())
}

async fn todos(user: &mut GooseUser) -> TransactionResult {
    let _ = user.get("/todos").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("create and get todos")
                .register_transaction(transaction!(todo_create).set_name("create todo"))
                .register_transaction(transaction!(todos).set_name("get todos"))
                .register_transaction(transaction!(todo_delete).set_name("delete todo")),
        )
        .execute()
        .await?;

    Ok(())
}
