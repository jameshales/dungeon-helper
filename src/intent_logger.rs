use serenity::model::channel::Message;
use snips_nlu_ontology::IntentParserResult;

use rusqlite::types::ToSql;
use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, Transaction};
//use serenity::model::channel::Message;
//use serenity::model::id::{ChannelId, UserId};
//use snips_nlu_ontology::{IntentParserResult, Slot, SlotValue};

pub fn log_intent_result(
    connection: &mut Connection,
    message: &Message,
    intent_result: &Option<IntentParserResult>,
) -> () {
    let _ = connection.transaction().and_then(|transaction| {
        intent_result
            .as_ref()
            .map_or(Result::Ok(()), |intent_result| {
                log_message(&transaction, message, intent_result).map(|_| ())
            })
            .and(transaction.commit())
    });
}

fn log_message(
    transaction: &Transaction,
    message: &Message,
    intent_result: &IntentParserResult,
) -> RusqliteResult<usize> {
    let params: &[&dyn ToSql] = &[
        &message.id.to_string(),
        &message.channel_id.to_string(),
        &message.author.id.to_string(),
        &message.content,
        &message.timestamp,
        &intent_result.intent.intent_name,
        &(intent_result.intent.confidence_score as f64),
    ];
    transaction.execute(
        "INSERT INTO messages (message_id, channel_id, user_id, content, posted, intent_name, confidence_score) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        params,
    )
}
