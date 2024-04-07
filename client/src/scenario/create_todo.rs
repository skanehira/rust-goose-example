use super::{CreateTodoParam, ScenarioParam};
use crate::transaction::todo::todo_create;
use crate::transaction::todo::todo_delete;
use crate::transaction::todo::todos;
use goose::prelude::*;
use goose::scenario;

pub fn create_todo_scenario(
    scenario_name: &str,
    param: ScenarioParam<CreateTodoParam>,
) -> Scenario {
    scenario!(scenario_name)
        .register_transaction(todo_create(param))
        .register_transaction(transaction!(todos).set_name("get todos"))
        .register_transaction(transaction!(todo_delete).set_name("delete todo"))
}
