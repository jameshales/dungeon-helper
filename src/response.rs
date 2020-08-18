use crate::attack_roll::Handedness;
use crate::character_roll::Check;
use crate::error::Error;
use crate::roll::{Condition, ConditionalRoll, ConditionalRollResult, Roll, RollResult};
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;

pub enum Response {
    AttackRoll {
        attack_name: String,
        attack_handedness: Option<Handedness>,
        to_hit_roll: ConditionalRoll,
        to_hit_result: ConditionalRollResult,
        damage_roll: Roll,
        damage_result: RollResult,
    },
    CharacterRoll {
        check: Check,
        roll: ConditionalRoll,
        result: ConditionalRollResult,
    },
    DiceRoll {
        roll: ConditionalRoll,
        result: ConditionalRollResult,
    },
    Clarification(String),
    Error(Error),
    Help(String),
    Warning(String),
}

impl Response {
    pub fn is_roll(&self) -> bool {
        match self {
            Response::AttackRoll { .. }
            | Response::CharacterRoll { .. }
            | Response::DiceRoll { .. } => true,
            _ => false,
        }
    }

    pub fn to_message<'a, 'b>(
        &self,
        author_nick: &str,
        message: &Message,
        builder: &'b mut CreateMessage<'a>,
    ) -> &'b mut CreateMessage<'a> {
        match self {
            Response::AttackRoll {
                attack_name,
                attack_handedness,
                to_hit_roll,
                to_hit_result,
                damage_roll,
                damage_result,
            } => {
                let condition = conditional_message(to_hit_roll.condition());
                let attack_handedness = match attack_handedness {
                    Some(Handedness::OneHanded) => " one handed",
                    Some(Handedness::TwoHanded) => " two handed",
                    None => "",
                };
                builder.embed(|e| {
                    e.title(format!(
                        "{} attacks{} using {}{}!",
                        author_nick, attack_handedness, attack_name, condition
                    ));
                    e.field("Attack", format!("🛡️ {}", to_hit_result), true);
                    e.field("Damage", format!("❤️ {}", damage_result), true);
                    e.footer(|f| {
                        f.text(format!(
                            "Attack Roll: {} | Damage Roll: {}",
                            to_hit_roll, damage_roll
                        ))
                    });
                    e.thumbnail(&message.author.face())
                })
            }
            Response::CharacterRoll {
                check,
                roll,
                result,
            } => {
                let condition = conditional_message(roll.condition());
                builder.embed(|e| {
                    e.title(format!("{} rolls {}{}!", author_nick, check, condition));
                    e.field("Result", format!("🎲 {}", result), false);
                    e.footer(|f| f.text(format!("Roll: {}", roll)));
                    e.thumbnail(&message.author.face())
                })
            }
            Response::DiceRoll { roll, result } => builder.embed(|e| {
                e.title(format!("{} rolls {}!", author_nick, roll));
                e.field("Result", format!("🎲 {}", result), false);
                e.thumbnail(&message.author.face())
            }),
            Response::Clarification(text) => {
                builder.content(format!("📎 <@{}> {}", message.author.id, text))
            }
            Response::Error(_) => builder.content(format!(
                "💥 <@{}> **Error:** A technical error has occurred. Reference ID: {}",
                message.author.id, &message.id
            )),
            Response::Help(text) => {
                builder.content(format!("🎱 <@{}> {}", message.author.id, text))
            }
            Response::Warning(text) => {
                builder.content(format!("⚠️ <@{}> {}", message.author.id, text))
            }
        }
    }
}

fn conditional_message(condition: Option<Condition>) -> &'static str {
    match condition {
        Some(Condition::Advantage) => " with advantage",
        Some(Condition::Disadvantage) => " with disadvantage",
        None => "",
    }
}
