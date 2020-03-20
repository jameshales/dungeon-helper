use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, OptionalExtension, Row};
use serenity::model::id::ChannelId;

pub struct Channel {
    pub enabled: bool,
    pub locked: bool,
    pub dice_only: bool,
}

impl Channel {
    pub fn get(connection: &Connection, channel_id: ChannelId) -> RusqliteResult<Option<Channel>> {
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
}
