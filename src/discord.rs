use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Footer {
    pub text: String
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool
}

#[derive(Debug, Serialize)]
pub struct Embed {
    pub title: String,
    pub color: usize,
    pub description: String,
    pub footer: Footer,
    /// ISO8601 formatted timestamp
    pub timestamp: Option<String>,
    pub fields: Vec<Field>
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub content: Option<String>,
    pub embeds: Vec<Embed>
}