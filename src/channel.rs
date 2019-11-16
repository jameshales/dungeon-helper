use rusqlite::types::ToSql;
use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, OptionalExtension, Row, Transaction};
use serenity::model::id::ChannelId;

pub struct Channel {
    pub enabled: bool,
    pub locked: bool,
    pub dice_only: bool,
}

impl Channel {
    pub fn get(connection: &Connection, channel_id: &ChannelId) -> RusqliteResult<Option<Channel>> {
        connection
            .query_row(
                "SELECT enabled, locked, dice_only FROM channels WHERE channel_id = $1",
                &[&channel_id.to_string()],
                Channel::from_row,
            )
            .optional()
    }

    fn from_row(row: &Row) -> RusqliteResult<Channel> {
        Ok(Channel {
            enabled: row.get("enabled")?,
            locked: row.get("locked")?,
            dice_only: row.get("dice_only")?,
        })
    }

    pub fn set_enabled(
        connection: &mut Connection,
        channel_id: &ChannelId,
        enabled: bool,
    ) -> RusqliteResult<()> {
        Channel::set_bool_flag(connection, channel_id, "enabled", enabled)
    }

    pub fn set_locked(
        connection: &mut Connection,
        channel_id: &ChannelId,
        locked: bool,
    ) -> RusqliteResult<()> {
        Channel::set_bool_flag(connection, channel_id, "locked", locked)
    }

    pub fn set_dice_only(
        connection: &mut Connection,
        channel_id: &ChannelId,
        dice_only: bool,
    ) -> RusqliteResult<()> {
        Channel::set_bool_flag(connection, channel_id, "dice_only", dice_only)
    }

    fn set_bool_flag(
        connection: &mut Connection,
        channel_id: &ChannelId,
        name: &str,
        value: bool,
    ) -> RusqliteResult<()> {
        connection.transaction().and_then(|transaction| {
            Channel::create_if_not_exists(&transaction, channel_id)
                .and({
                    let params: &[&dyn ToSql] = &[&value, &channel_id.to_string()];
                    transaction.execute(
                        format!("UPDATE channels SET {} = $1 WHERE channel_id = $2", name).as_ref(),
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
