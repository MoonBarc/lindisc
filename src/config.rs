use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub webhook_url: String,
    pub bind: String
}