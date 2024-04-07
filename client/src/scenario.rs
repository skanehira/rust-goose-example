pub mod create_todo;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Parameter {
    pub scenarios: Vec<Scenario>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub params: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParam<T> {
    pub params: T,
}

impl<T: DeserializeOwned> ScenarioParam<T> {
    pub fn try_from(scenario: Scenario) -> Result<Self, serde_yaml::Error> {
        let params = serde_yaml::from_value::<T>(scenario.params)?;
        Ok(ScenarioParam { params })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoParam {
    pub count: usize,
}
