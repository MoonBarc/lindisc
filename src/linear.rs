use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Update,
    Create,
    Remove
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Data {
    #[serde(rename_all = "camelCase")]
    Issue {
        id: String,
        number: usize,
        title: String,
        state: State,
        team: Team,
        created_at: String,
        updated_at: String
    }
}

#[derive(Debug, Deserialize)]
pub struct State {
    pub color: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearPayload {
    pub action: Action,
    pub data: Data,
    pub url: Option<String>
}