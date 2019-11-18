# Commands Reference

## Rolls

Rolls other than simple dice rolls will require relevant character abilities, and a character level to be set (see next section).
Proficiency in saving throws, skills, and weapons may also optionally be set, but default to normal proficiency. 

### Dice

- Roll three d8s
- Throw two twelve-sided dice
- Toss a d20 with advantage

### Ability Check

- Do a strength check
- Roll wisdom with advantage
- Perform a constitution check with disadvantage

Supported abilities:
- Strength
- Dexterity
- Constitution
- Intelligence
- Wisdom
- Charisma

### Skill Check

- Roll for athletics 
- Check investigation with advantage
- Do a stealth check with disadvantage

Supported skills:
- Acrobatics
- Animal Handling
- Arcana
- Athletics
- Deception
- History
- Insight
- Intimidation
- Investigation
- Medicine
- Nature
- Perception
- Performance
- Persuasion
- Religion
- Sleight of Hand
- Stealth
- Survival

### Saving Throw

- Perform a wisdom saving throw
- Try a strength saving throw with advantage
- Do a saving throw for constitution with disadvantage

Supported saving throws:
- Strength
- Dexterity
- Constitution
- Intelligence
- Wisdom
- Charisma

### Initiative

- Roll initiative
- Perform an initiative check with advantage
- Do an initiative throw with disadvantage

An initiative roll is simply a dexterity check.

### Attacking

- Attack with a club
- Attack using a dagger as melee
- Do a ranged dagger attack
- Perform a two handed longsword attack
- Roll quarterstaff with one hand
- Hand crossbow attack with advantage
- Shortbow attack with disadvantage

Supported weapons:
- Battleaxe
- Club
- Hand Crossbow
- Heavy Crossbow
- Light Crossbow
- Dagger
- Dart
- Flail
- Glaive
- Greataxe
- Greatclub
- Greatsword
- Halberd
- Handaxe
- Javelin
- Lance
- Light Hammer
- Longbow
- Longsword
- Mace
- Maul
- Morningstar
- Pike
- Quarterstaff
- Rapier
- Scimitar
- Shortbow
- Shortsword
- Sickle
- Sling
- Spear
- Trident
- War Pick
- Warhammer
- Whip

## Character Attributes

Character attributes are used to determine modifiers in ability, skill, saving, and attack rolls.

Character attributes are tied to a specific Discord channel and user, so a user must set their attributes separately for each channel they participate in.

### Viewing Attributes

- Show strength
- What is my dexterity score?
- Tell me my stealth modifier
- Display level
- Do I have jack of all trades?
- What is my strength saving throw?
- What are my abilities?
- Show me my skill proficiencies
- What are my weapon proficiencies?

### Setting Attributes

- Set strength to 15
- Update dexterity score to 18
- Make stealth proficient
- Set athletics to expert
- Change persuasion to normal
- Set my level to 5
- Give me jack of all trades
- Make strength saving throw proficient
- Make wisdom saving throw normal
- Set club to proficient
- Update simple weapons to proficient

## Direct Messages

Dungeon Helper will respond to direct messages to roll dice, but won't perform character-related rolls, such as attribute or skill checks.

## Administrator Commands

The administrators of a Discord server may set per-channel settings to control the Dungeon Helper.

### Enable or Disable a Channel

After Dungeon Helper has been invited to a Discord server, it receives messages from all of the text channels in the server, but only responds to messages from channels that Dungeon Helper has been enabled in, or messages from server administrators.
By default Dungeon Helper is not enabled in any channel.

- Enable channel
- Disable channel

### Dice Only Mode

By default Dungeon Helper only responds to messages that begin with an tag for the Dungeon Helper user, e.g. "@Dungeon Helper Roll a d20".
To reduce typing one may enable dice only mode in the channel, in which case Dungeon Helper will respond to all messages, without requiring a tag.
It's recommended that this be enabled in a text channel separate to the general chat channel.

- Enable dice only mode
- Disable dice only mode

### Lock Character Attributes

Once character attributes have been set in a channel, an administrator may lock the channel to prevent further changes to the attributes.

- Lock this channel
- Unlock this channel
