use crate::character::{CharacterAttribute, CharacterAttributeUpdate};
use crate::character_roll::CharacterRoll;
use crate::error;
use crate::intent_parser::parse_intent_result;
use crate::response::Response;
use crate::roll;
use crate::roll::ConditionalRoll;
use crate::roll::Error as RollError;
use crate::weapon::AmbiguousWeaponName;
use regex::Regex;
use snips_nlu_lib::SnipsNluEngine;
use snips_nlu_ontology::IntentParserResult;
use std::fmt;
use symspell::{SymSpell, UnicodeStringStrategy};

#[derive(Debug)]
pub enum Command {
    AttackRoll(crate::attack_roll::AttackRoll),
    CharacterRoll(crate::character_roll::CharacterRoll),
    Help,
    HelpShorthand,
    Roll(crate::roll::ConditionalRoll),
    Set(CharacterAttributeUpdate),
    SetChannelEnabled(bool),
    SetChannelLocked(bool),
    SetChannelDiceOnly(bool),
    Show(CharacterAttribute),
    ShowAbilities,
    ShowSkills,
    ShowWeaponProficiencies,
}

impl Command {
    pub fn description(&self) -> &str {
        match self {
            Command::AttackRoll(_) => "perform an attack roll",
            Command::CharacterRoll(_) => "perform a character roll",
            Command::Help | Command::HelpShorthand => "ask for help",
            Command::Roll(_) => "perform a roll",
            Command::Set(_) => "set a character attribute",
            Command::SetChannelEnabled(_)
            | Command::SetChannelLocked(_)
            | Command::SetChannelDiceOnly(_) => "set a channel attribute",
            Command::Show(_) => "show a character attribute",
            Command::ShowAbilities => "show a character's abilities",
            Command::ShowSkills => "show a character's skills",
            Command::ShowWeaponProficiencies => "show a character's weapon proficiencies",
        }
    }
}

#[derive(Debug)]
pub enum Error {
    // Shorthand commands
    CharacterRollParserError,
    RollParserError(roll::ParserError),

    // Natural language commands
    IntentParserError(::failure::Error),
    NoIntent,
    RollAbilityMissingAbility,
    RollAttackAmbiguousWeapon(AmbiguousWeaponName),
    RollAttackMissingClassification,
    RollAttackMissingHandedness,
    RollAttackMissingWeapon,
    RollDiceMissingSides,
    RollDiceInvalid(RollError, usize, i32),
    RollSavingThrowMissingAbility,
    RollSkillMissingSkill,
    SetAbilityMissingAbility,
    SetAbilityMissingScore,
    SetLevelMissingLevel,
    SetSavingThrowMissingAbility,
    SetSavingThrowMissingProficiency,
    SetSkillMissingSkill,
    SetSkillMissingProficiency,
    SetWeaponProficiencyAmbiguousWeapon(AmbiguousWeaponName),
    SetWeaponProficiencyMissingProficiency,
    SetWeaponProficiencyMissingWeaponAndCategory,
    ShowAbilityMissingAbility,
    ShowPassiveAbilityMissingAbility,
    ShowPassiveSkillMissingSkill,
    ShowSavingThrowMissingAbility,
    ShowSkillMissingSkill,
    UnknownIntent(String),
}

impl Error {
    pub fn into_response(self) -> Response {
        match self {
            Error::IntentParserError(error) => {
                Response::Error(error::Error::IntentParserError(error))
            }
            Error::UnknownIntent(intent_name) => {
                Response::Error(error::Error::UnknownIntent(intent_name))
            }
            error => Response::Clarification(error.to_string()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CharacterRollParserError => {
                write!(f, "It looks like you're trying to roll a skill or ability check, but the syntax is invalid. Try typing `!help` for some examples.")
            }
            Error::RollParserError(error) => {
                write!(f, "It looks like you're trying to some dice, but the syntax is invalid. {} Try typing `!help` for some examples.", error)
            }
            Error::RollAbilityMissingAbility => {
                write!(f, "It looks like you're trying to roll an ability check, but I'm not sure which ability you want. Try \"Roll strength\", \"Dexterity check\", etc.")
            }
            Error::RollAttackAmbiguousWeapon(ambiguous_weapon) => {
                write!(f, "It looks like you're trying to roll an attack check with a {}, but that is an ambiguous weapon name. {}", ambiguous_weapon, ambiguous_weapon.message())
            }
            Error::RollAttackMissingClassification => {
                write!(f, "It looks like you're trying to roll an attack check with an improvised weapon, but I'm not sure whether it should be a melee or ranged attack. Try \"Attack improvised weapon as melee\", \"Roll ranged improvised weapon check\", etc.")
            }
            Error::RollAttackMissingHandedness => {
                write!(f, "It looks like you're trying to roll an attack check with a weapon that has the versatile property, but I'm not sure whether you want to attack with one hand or two hands. Try \"One-handed attack quarterstaff\", \"Roll longsword weapon check with two hands\", etc.")
            }
            Error::RollAttackMissingWeapon => {
                write!(f, "It looks like you're trying to roll an attack check, but I'm not sure which weapon you want to attack with. Try \"Attack club\", \"Dagger attack\", etc.")
            }
            Error::RollDiceMissingSides => {
                write!(f, "It looks like you're trying to roll some dice, but I'm not sure what kind of dice you want. Try \"Roll a d20\", \"Throw two four-sided dice\", etc.")
            }
            Error::RollDiceInvalid(error, rolls, sides) => match error {
                RollError::RollsTooGreat => {
                    write!(f, "It looks like you're trying to roll {} dice. That's too many dice! Try rolling 100 or fewer dice.", rolls)
                }
                RollError::SidesNonPositive => {
                    write!(f, "It looks like you're trying to roll dice with {} sides. I can only roll a positive number of sides. Try rolling dice with one or more sides.", sides)
                }
                RollError::SidesTooGreat => {
                    write!(f, "It looks like you're trying to roll dice with {} sides. That's too many sides! Try rolling dice with 100 or fewer sides.", sides)
                }
            }
            Error::RollSavingThrowMissingAbility => {
                write!(f, "It looks like you're trying to roll a saving throw, but I'm not sure what kind of saving throw you want. Try \"Roll strength saving throw\", \"Dexterity saving throw\", etc.")
            }
            Error::RollSkillMissingSkill => {
                write!(f, "It looks like you're trying to roll a skill check, but I'm not sure what skill you want. Try \"Roll stealth\", \"Athletics check\", etc.")
            }
            Error::SetAbilityMissingAbility => {
                write!(f, "It looks like you're trying to set an ability score, but I'm not sure what ability you want to set. Try \"Set strength as 12\", \"Change dexterity to 14\", etc.")
            }
            Error::SetAbilityMissingScore => {
                write!(f, "It looks like you're trying to set an ability score, but I'm not sure what score you want to set it to. Try \"Set strength as 12\", \"Change dexterity to 14\", etc.")
            }
            Error::SetLevelMissingLevel => {
                write!(f, "It looks like you're trying to set your level, but I'm not sure what level you want to set it to. Try \"Set level as 3\", \"Change level to 5\", etc.")
            }
            Error::SetSavingThrowMissingAbility => {
                write!(f, "It looks like you're trying to set a saving throw proficiency, but I'm not sure what saving throw you want to set. Try \"Set strength saving throw to proficient\", \"Change dexterity saving throw to normal\", etc.")
            }
            Error::SetSavingThrowMissingProficiency => {
                write!(f, "It looks like you're trying to set a saving throw proficiency, but I'm not sure what proficiency you want to set it to. Try \"Set strength saving throw to proficient\", \"Change dexterity saving throw to normal\", etc.")
            }
            Error::SetSkillMissingSkill => {
                write!(f, "It looks like you're trying to set a skill proficiency, but I'm not sure what skill you want to set. Try \"Set athletics to proficient\", \"Change stealth to expert\", \"Update nature to normal\" etc.")
            }
            Error::SetSkillMissingProficiency => {
                write!(f, "It looks like you're trying to set a skill proficiency, but I'm not sure what proficiency you want to set it to. Try \"Set athletics to proficient\", \"Change stealth to expert\", \"Update nature to normal\" etc.")
            }
            Error::SetWeaponProficiencyAmbiguousWeapon(ambiguous_weapon) => {
                write!(f, "It looks like you're trying to set a weapon proficiency for {}, but that is an ambiguous weapon name. {}", ambiguous_weapon, ambiguous_weapon.message())
            }
            Error::SetWeaponProficiencyMissingProficiency => {
                write!(f, "It looks like you're trying to set a weapon proficiency, but I'm not sure what proficiency you want to set it to. Try \"Set club to proficient\", \"Change martial weapons to normal\", etc.")
            }
            Error::SetWeaponProficiencyMissingWeaponAndCategory => {
                write!(f, "It looks like you're trying to set a weapon proficiency, but I'm not sure what weapon or category of weapons you want to set. Try \"Set club to proficient\", \"Change martial weapons to normal\", etc.")
            }
            Error::ShowAbilityMissingAbility => {
                write!(f, "It looks like you're trying to view an ability score, but I'm not sure what ability you want. Try \"Show strength\", \"Display dexterity\", etc.")
            }
            Error::ShowPassiveAbilityMissingAbility => {
                write!(f, "It looks like you're trying to view a passive ability score, but I'm not sure what ability you want. Try \"Show passive strength\", \"Display passive dexterity\", etc.")
            }
            Error::ShowPassiveSkillMissingSkill => {
                write!(f, "It looks like you're trying to view a passive skill score, but I'm not sure what skill you want. Try \"Show passive athletics\", \"Display passive stealth\", etc.")
            }
            Error::ShowSavingThrowMissingAbility => {
                write!(f, "It looks like you're trying to view a saving throw modifier, but I'm not sure what ability you want. Try \"Show strength saving throw\", \"Display passive saving throw\", etc.")
            }
            Error::ShowSkillMissingSkill => {
                write!(f, "It looks like you're trying to view a skill modifier, but I'm not sure what skill you want. Try \"Show athletics\", \"Display stealth\", etc.")
            }
            Error::NoIntent => {
                write!(f, "I'm not sure what you mean. Try asking again with a different or simpler phrasing. Try asking for help to see some examples.")
            }
            Error::UnknownIntent(intent_name) => {
                write!(f, "An unknown intent name was returned by the NLP engine: {}", intent_name)
            },
            Error::IntentParserError(error) => {
                write!(f, "An unknown error was returned by the NLP engine: {}", error)
            }
        }
    }
}

type NaturalLanguageCommandResult =
    Option<Result<(Result<Command, Error>, IntentParserResult, Option<String>), Error>>;

impl Command {
    pub fn is_admin(&self) -> bool {
        match self {
            Command::SetChannelDiceOnly(_)
            | Command::SetChannelEnabled(_)
            | Command::SetChannelLocked(_) => true,
            _ => false,
        }
    }

    pub fn is_editing(&self) -> bool {
        match self {
            Command::Set(_) => true,
            _ => false,
        }
    }

    pub fn is_private(&self) -> bool {
        match self {
            Command::Help | Command::HelpShorthand | Command::Roll(_) => true,
            _ => false,
        }
    }

    pub fn parse(
        engine: &SnipsNluEngine,
        symspell: &SymSpell<UnicodeStringStrategy>,
        content: &str,
        bot_id: Option<&str>,
        dice_only: bool,
    ) -> Option<Result<CommandResult, Error>> {
        Command::parse_shorthand(content)
            .map(CommandResult::Shorthand)
            .map(Ok)
            .or({
                Command::parse_natural_language(engine, symspell, content, bot_id, dice_only).map(|result| {
                    result.map(|(command, intent_result, corrected)| {
                        CommandResult::NaturalLanguage(command, intent_result, corrected)
                    })
                })
            })
    }

    fn parse_natural_language(
        engine: &SnipsNluEngine,
        symspell: &SymSpell<UnicodeStringStrategy>,
        message: &str,
        bot_id: Option<&str>,
        dice_only: bool,
    ) -> NaturalLanguageCommandResult {
        Command::extract_at_message(message, bot_id, dice_only).as_ref().map(|at_message| {
            let corrected = Command::spelling_correction(symspell, at_message);
            let used = corrected.as_ref().unwrap_or(at_message).as_str();
            engine
                .parse(used, None, None)
                .map(|result| (parse_intent_result(&result), result, corrected))
                .map_err(Error::IntentParserError)
        })
    }

    fn extract_at_message(message: &str, bot_id: Option<&str>, dice_only: bool) -> Option<String> {
        lazy_static! {
            static ref COMMAND_REGEX: Regex = Regex::new(r"^(?:<@!?(\d+)> *)?(.*)$").unwrap();
        }

        COMMAND_REGEX.captures(&message).and_then(|c| {
            let is_at_message = c
                .get(1)
                .map_or(false, |m| bot_id.iter().any(|bot_id| bot_id == &m.as_str()));
            if dice_only || is_at_message {
                c.get(2).map(|m| m.as_str().to_owned())
            } else {
                None
            }
        })
    }

    fn spelling_correction(symspell: &SymSpell<UnicodeStringStrategy>, message: &str) -> Option<String> {
        let trimmed = message.trim();
        let suggestions = symspell.lookup_compound(trimmed, 2);
        suggestions.into_iter().next().map(|s| s.term)
    }

    fn parse_shorthand(command: &str) -> Option<Result<Command, Error>> {
        lazy_static! {
            static ref ROLL_COMMAND_REGEX: Regex = Regex::new(r"^!(?:r|roll) +(.*)$").unwrap();
        }

        if command == "!help" {
            Some(Ok(Command::HelpShorthand))
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&command) {
            let roll_command = captures.get(1).map_or("", |m| m.as_str()).to_owned();
            Some(
                ConditionalRoll::parse(&roll_command)
                    .map(Command::Roll)
                    .map_err(Error::RollParserError)
                    .or_else(|_| {
                        CharacterRoll::parse(&roll_command)
                            .map(Command::CharacterRoll)
                            .ok_or(Error::CharacterRollParserError)
                    }),
            )
        } else {
            None
        }
    }
}

pub enum CommandResult {
    Shorthand(Result<Command, Error>),
    NaturalLanguage(Result<Command, Error>, IntentParserResult, Option<String>),
}
