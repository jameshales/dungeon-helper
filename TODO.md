# TODO

- Flip a coin.
- Better shorthand / non-NLP syntax.
- Have admin / DM options, such as:
    - Perform actions on behalf of a users, such as roll, edit attributes, or view attributes.
    - Perform actions for everyone in the channel, e.g. initiative rolls, saving throws.
- Spell rolls.
- Custom attacks and spells.
- Spelling correction for skills, abilities, weapons, etc. ("did you mean ... ?")
  - Make entities automatically extensible and lower the matching strictness.
    This gives a slot with the misspelled entity. Use Levenshtein metric to
    find the closest matching entity, or ask for a clarification if the
    Levenshtein distance is too great.
  - Add entities to the frequency dictionary, so they won't be corrected to different words.
  - Add bigrams to the bigram dictionary so that they will be split correctly.
    - Bigrams, such as "roll dex", "throw dex", "saving throw", etc.
  - Work out valid frequencies for words and bigrams:
    - Rate entities and words appearing in intent examples higher.
    - Rate nouns, adjectives, and verbs lower.
    - Keep connective words in-place.
    - Mostly, want to make sure that words in the domain are ranked higher than
      words outside of the domain (or something like that). Really, for a given
      word, we only care about the relative rankings for words within a given
      edit distance. So perhaps do a lookup for each word by edit distance, and
      give words in the domain a +1 frequency.
- Parse character sheets from D&D Beyond.
  
  There is a JSON version available by appending `/json` to the character sheet
  URL. There is also a reference to the JSON URL in the HTML. The JSON is not
  straight-forward to decode, as it includes base attributes and bonuses /
  modifiers separately, requiring parsing the bonuses / modifiers and adding
  them to the base attributes. Can't scrape the JSON automatically, user must
  provide. Urgh.

  Maybe have a GreaseMonkey script that does the needful and produces a Base-64
  code that can be provided to the bot.
- Delete error messages from channel after a short delay, to keep the chat log
  clean.

## Done

- Add logging.
  - Debug logging, showing all raw messages received.
  - NLP logging, showing text and parsed intents / slots / confidence.
- Have admin / DM options, such as:
    - Enable or disable the bot for a channel.
    - Lock or unlock changes to characters for the channel.
    - Disable character rolls in private channels.
- Attack rolls.
