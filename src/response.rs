use crate::error::Error;
use serenity::model::id::{MessageId, UserId};

pub enum Response {
    Clarification(String),
    DiceRoll(String),
    Error(Error),
    Help(String),
    Show(String),
    Update(String),
    Warning(String),
}

impl Response {
    pub fn render(&self, author_id: UserId, message_id: MessageId) -> String {
        match self {
            Response::Clarification(message) => format!("📎 <@{}> {}", author_id, message),
            Response::DiceRoll(message) => format!("🎲 <@{}> {}", author_id, message),
            Response::Error(_) => format!(
                "💥 <@{}> **Error:** A technical error has occurred. Reference ID: {}",
                author_id, message_id
            ),
            Response::Help(message) => format!("🎱 <@{}> {}", author_id, message),
            Response::Show(message) => format!("<@{}> {}", author_id, message),
            Response::Update(message) => format!("💾 <@{}> {}", author_id, message),
            Response::Warning(message) => format!("⚠️ <@{}> {}", author_id, message),
        }
    }
}
