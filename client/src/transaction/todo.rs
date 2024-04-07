use std::sync::Arc;

use goose::prelude::*;
use serde::Deserialize;

use crate::scenario::{CreateTodoParam, ScenarioParam};

#[derive(Deserialize)]
struct Todo {
    id: u32,
}

struct Session {
    id: u32,
}

pub fn todo_create(param: ScenarioParam<CreateTodoParam>) -> Transaction {
    let func: TransactionFunction = Arc::new(move |user| {
        let param = param.params.clone();

        Box::pin(async move {
            for _ in 0..param.count {
                let todo = serde_json::json!({
                    "title": "Learn Rust",
                    "completed": false,
                });

                let mut goose_resp = user.post_json("/todos", &todo).await?;

                match goose_resp.response {
                    Ok(resp) => match resp.json::<Todo>().await {
                        Ok(todo) => {
                            user.set_session_data(Session { id: todo.id });
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

            Ok(())
        })
    });

    Transaction::new(func).set_name("create project")
}

pub async fn todo_delete(user: &mut GooseUser) -> TransactionResult {
    let session = user.get_session_data_unchecked::<Session>();
    let _ = user.delete(&format!("/todos/{}", session.id)).await?;

    Ok(())
}

pub async fn todos(user: &mut GooseUser) -> TransactionResult {
    let _ = user.get("/todos").await?;

    Ok(())
}
