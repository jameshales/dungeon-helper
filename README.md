# Dungeon Helper

🎲 A Discord bot for performing Dungeons and Dragons 5th Edition dice rolls.

## Usage

### Requirements

- [Rust](https://www.rust-lang.org/) >= 0.28
- [SQLite](https://www.sqlite.org/) >= 3.24.0

### Create an application in Discord

In the Discord Developer Portal:

1.  Create a "New Application" in the [Discord Developer Portal](https://discordapp.com/developers/applications).
2.  Under the "Bot" tab, create a new Bot user. Set its name and icon appropriately. Take note of the "Token".
3.  Under the "OAuth2" tab, in the "Scopes" section select the "bot" scope, and in the "Bot Permissions" section select the "Send Messages" permission. Copy the URL that appears below. It should have the form:

    ```
    https://discordapp.com/api/oauth2/authorize?client_id=012345678912345678&permissions=2048&scope=bot
    ```

    The `client_id` should match the "Client ID" of your application, found under the "General Information" tab.
4.  Go to the URL. Use the form to authorise the bot to join a server that you manage.

### Train the Snips NLU model

See separate [README](./train/README.md).

### Build and run the application

In a local command-line environment:

1.  Build the application with Cargo.

    ```
    cargo build --release
    ```
2.  Set the `DISCORD_TOKEN` environment variable to the bot token noted before.
3.  Set the `DATABASE_PATH` to the path of a SQLite database, initialised with the SQL schema in `./config/sql/`.
4.  Set the `MODEL_PATH` to the path of a trained Snips NLU model.
5.  Set `RUST_LOG=dungeon_helper=info` to enable logging.
6.  Run the application.

    ```
    ./target/release/dungeon_helper
    ```

### Interact with the bot

In a Discord server that the bot has joined:

- Type `!help` for usage instructions.
- Type `!roll 1d20` to roll one 20-sided die.
- Type `!roll 2d8 + 4` to roll two 8-sided dice with a modifier of +4 (i.e. adding 4 to the sum of the two dice).
- Type `!roll 1d20 + 5 with advantage` to roll one 20-sided die with a modifier of +5 with advantage (taking the highest of two rolls).
- Type `!roll 1d20 - 1 with advantage` to roll one 20-sided die with a modifier of -5 with disadvantage (taking the lowest of two rolls).
