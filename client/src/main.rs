mod macros;
mod scenario;
mod transaction;

use goose::prelude::*;
use scenario::{create_todo::create_todo_scenario, CreateTodoParam, Parameter, ScenarioParam};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let parameter_file = std::env::var("PARAMETER").expect("not found PARAMETER in environment");

    let file = std::fs::File::open(parameter_file)?;
    let Parameter { scenarios } =
        serde_yaml::from_reader(file).expect("failed to read parameter file");

    let mut attack = GooseAttack::initialize()?;

    create_scenario!(
        attack,
        "create todo",
        scenarios,
        CreateTodoParam,
        create_todo_scenario
    );

    attack.execute().await?;

    Ok(())
}
