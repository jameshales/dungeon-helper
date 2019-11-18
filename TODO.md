# TODO

- Flip a coin.
- Better shorthand / non-NLP syntax.
- Have admin / DM options, such as:
    - Perform actions on behalf of a users, such as roll, edit attributes, or view attributes.
    - Perform actions for everyone in the channel, e.g. initiative rolls, saving throws.
- Spell rolls.
- Custom attacks and spells.
- Parse character sheets from D&D Beyond.
  
  There is a JSON version available by appending `/json` to the character sheet
  URL. There is also a reference to the JSON URL in the HTML. The JSON is not
  straight-forward to decode, as it includes base attributes and bonuses /
  modifiers separately, requiring parsing the bonuses / modifiers and adding
  them to the base attributes. Can't scrape the JSON automatically, user must
  provide. Urgh.

  Maybe have a GreaseMonkey script that does the needful and produces a Base-64
  code that can be provided to the bot.

## Done

- Add logging.
  - Debug logging, showing all raw messages received.
  - NLP logging, showing text and parsed intents / slots / confidence.
- Have admin / DM options, such as:
    - Enable or disable the bot for a channel.
    - Lock or unlock changes to characters for the channel.
    - Disable character rolls in private channels.
- Attack rolls.
