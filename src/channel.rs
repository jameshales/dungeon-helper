use rusqlite::types::ToSql;
use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, Row, Transaction};
use serenity::model::id::ChannelId;

pub struct Channel {
    pub enabled: bool,
    pub locked: bool,
}

impl Channel {
    pub fn get(connection: &Connection, channel_id: &ChannelId) -> RusqliteResult<Channel> {
        connection.query_row(
            "SELECT enabled, locked FROM channels WHERE channel_id = $1",
            &[&channel_id.to_string()],
            Channel::from_row,
        )
    }

    fn from_row(row: &Row) -> RusqliteResult<Channel> {
        Result::Ok(Channel {
            enabled: row.get("enabled")?,
            locked: row.get("locked")?,
        })
    }

    pub fn set_enabled(
        connection: &mut Connection,
        channel_id: &ChannelId,
        enabled: bool,
    ) -> RusqliteResult<()> {
        connection.transaction().and_then(|transaction| {
            Channel::create_if_not_exists(&transaction, channel_id)
                .and({
                    let params: &[&dyn ToSql] = &[&enabled, &channel_id.to_string()];
                    transaction.execute(
                        "UPDATE channels SET enabled = $1 WHERE channel_id = $2",
                        params,
                    )
                })
                .and(transaction.commit())
        })
    }

    pub fn set_locked(
        connection: &mut Connection,
        channel_id: &ChannelId,
        locked: bool,
    ) -> RusqliteResult<()> {
        connection.transaction().and_then(|transaction| {
            Channel::create_if_not_exists(&transaction, channel_id)
                .and({
                    let params: &[&dyn ToSql] = &[&locked, &channel_id.to_string()];
                    transaction.execute(
                        "UPDATE channels SET locked = $1 WHERE channel_id = $2",
                        params,
                    )
                })
                .and(transaction.commit())
        })
    }

    fn create_if_not_exists(
        transaction: &Transaction,
        channel_id: &ChannelId,
    ) -> RusqliteResult<usize> {
        let params: &[&dyn ToSql] = &[&channel_id.to_string()];
        transaction.execute(
            "INSERT OR IGNORE INTO channels (channel_id) VALUES ($1)",
            params,
        )
    }
}
