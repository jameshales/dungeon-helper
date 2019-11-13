use rusqlite::Result as RusqliteResult;
use rusqlite::types::ToSql;
use rusqlite::{Connection, Transaction};
use serenity::model::channel::Message;
use serenity::model::id::MessageId;
use snips_nlu_ontology::{IntentParserResult, Slot, SlotValue};

pub fn log_intent_result(
    connection: &mut Connection,
    message: &Message,
    intent_result: &Option<IntentParserResult>,
) -> () {
    let _ = connection.transaction().and_then(|transaction| {
        intent_result
            .as_ref()
            .map_or(Result::Ok(()), |intent_result| {
                log_message(&transaction, message, intent_result)
                    .and(
                        intent_result.slots.iter().enumerate()
                            .fold(
                                Result::Ok(()),
                                |result, (index, slot)| result.and(log_slot(&transaction, &message.id, index as i32, slot)).map(|_| ())
                            )
                    )
                    .and(Result::Ok(()))
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

fn log_slot(
    transaction: &Transaction,
    message_id: &MessageId,
    index: i32,
    slot: &Slot
) -> RusqliteResult<usize> {
    let params: &[&dyn ToSql] = &[
        &message_id.to_string(),
        &index,
        &slot.raw_value,
        &slot_value_to_string(&slot.value),
        &slot.slot_name,
        &slot.confidence_score.map(|v| v as f64)
    ];
    transaction.execute(
        "INSERT INTO slots (message_id, slot_index, raw_value, value, slot_name, confidence_score) VALUES ($1, $2, $3, $4, $5, $6)",
        params
    )
}

fn slot_value_to_string(slot_value: &SlotValue) -> Option<String> {
    match slot_value {
        SlotValue::Custom(inner_value) => Some(inner_value.value.to_string()),
        SlotValue::Number(inner_value) => Some(inner_value.value.to_string()),
        _ => None
    }
}
