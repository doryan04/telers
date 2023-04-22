use super::base::Filter;

use crate::{
    client::{Bot, Session},
    context::Context,
    error::SessionErrorKind,
    types::{BotCommand, Update},
};

use async_trait::async_trait;
use regex::Regex;
use std::{borrow::Cow, iter::once, result::Result as StdResult};
use thiserror;

pub type Result<T> = StdResult<T, Error>;

/// This enum represents all possible errors that can occur when using the command filter
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid mention")]
    InvalidMention,
    #[error("Invalid command")]
    InvalidCommand,
    /// Occurs when the filter try to get the bot username. \
    /// For more information about the error, see [`SessionErrorKind`]
    #[error(transparent)]
    Session(#[from] SessionErrorKind),
}

/// Represents a command pattern type for verification
/// # Variants
/// * [`PatternType::Text(Cow<str>)`] - A command pattern with text
/// * [`PatternType::Object(BotCommand)`] -
/// A command pattern with [`BotCommand`] object. \
/// Just a shortcut for [`PatternType::Text(command.command)`].
/// * [`PatternType::Regex(Regex)`] -
/// A command pattern with regex, compiled with [`Regex`] struct. \
/// If filter used with `ignore_case` flag, then the regex will be compiled with `(?i)` flag (ignore case sensitive flag).
#[derive(Debug, Clone)]
pub enum PatternType<'a> {
    Text(Cow<'a, str>),
    Object(BotCommand),
    Regex(Regex),
}

impl<'a> From<Cow<'a, str>> for PatternType<'a> {
    fn from(text: Cow<'a, str>) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a str> for PatternType<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<BotCommand> for PatternType<'_> {
    fn from(command: BotCommand) -> Self {
        Self::Object(command)
    }
}

impl From<Regex> for PatternType<'_> {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

/// This filter checks if the message is a command
///
/// You can use parsed command using [`CommandObject`] struct in handler arguments,
/// or get it from [`Context`] by `command` key.
#[derive(Debug, Clone)]
pub struct Command<'a> {
    /// List of commands ([`Cow`], [`BotCommand`] or compiled [`Regex`] patterns)
    commands: Vec<PatternType<'a>>,
    /// Command prefix
    prefix: &'a str,
    /// Ignore case sensitive
    ignore_case: bool,
    /// Ignore bot mention
    ignore_mention: bool,
}

impl<'a> Command<'a> {
    /// Creates a new [`Command`] filter
    /// # Arguments
    /// * `commands` - List of commands (texts, [`BotCommand`] or compiled [`Regex`] patterns)
    /// * `prefix` - Command prefix
    /// * `ignore_case` - Ignore other command case
    /// * `ignore_mention` - Ignore bot mention
    /// # Panics
    /// If `ignore_case` is `true` and [`Regex`],
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn new<T, I>(commands: I, prefix: &'a str, ignore_case: bool, ignore_mention: bool) -> Self
    where
        T: Into<PatternType<'a>>,
        I: IntoIterator<Item = T>,
    {
        let commands = if ignore_case {
            commands
                .into_iter()
                .map(|command| match command.into() {
                    PatternType::Text(text) => PatternType::Text(text.to_lowercase().into()),
                    PatternType::Object(command) => {
                        PatternType::Text(command.command.to_lowercase().into())
                    }
                    PatternType::Regex(regex) => PatternType::Regex(
                        Regex::new(&format!("(?i){regex}"))
                            .expect("Failed to compile regex with (?i) flag"),
                    ),
                })
                .collect()
        } else {
            commands
                .into_iter()
                .map(|command| match command.into() {
                    PatternType::Object(command) => PatternType::Text(command.command.into()),
                    command => command,
                })
                .collect()
        };

        Self {
            commands,
            prefix,
            ignore_case,
            ignore_mention,
        }
    }

    /// Creates a new [`Command`] filter with passed command and default values
    #[must_use]
    pub fn one(val: impl Into<PatternType<'a>>) -> Self {
        Self::builder().command(val).build()
    }

    /// Creates a new [`Command`] filter with passed commands and default values
    #[must_use]
    pub fn many<T, I>(commands: I) -> Self
    where
        T: Into<PatternType<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().commands(commands).build()
    }

    #[must_use]
    pub fn builder() -> CommandBuilder<'a> {
        CommandBuilder::new()
    }
}

impl Default for Command<'_> {
    #[must_use]
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct CommandBuilder<'a> {
    commands: Vec<PatternType<'a>>,
    prefix: &'a str,
    ignore_case: bool,
    ignore_mention: bool,
}

impl<'a> CommandBuilder<'a> {
    #[must_use]
    pub fn new() -> CommandBuilder<'a> {
        Self::default()
    }

    #[must_use]
    pub fn command(self, val: impl Into<PatternType<'a>>) -> Self {
        Self {
            commands: self.commands.into_iter().chain(once(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn commands<T, I>(self, val: I) -> Self
    where
        T: Into<PatternType<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            commands: self
                .commands
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn prefix(self, val: &'a str) -> Self {
        Self {
            prefix: val,
            ..self
        }
    }

    #[must_use]
    pub fn ignore_case(self, val: bool) -> Self {
        Self {
            ignore_case: val,
            ..self
        }
    }

    #[must_use]
    pub fn ignore_mention(self, val: bool) -> Self {
        Self {
            ignore_mention: val,
            ..self
        }
    }

    /// # Panics
    /// If `ignore_case` is `true` and [`Regex`],
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn build(self) -> Command<'a> {
        Command::new(
            self.commands,
            self.prefix,
            self.ignore_case,
            self.ignore_mention,
        )
    }
}

impl Default for CommandBuilder<'_> {
    #[must_use]
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

impl<'a> Command<'a> {
    /// # Errors
    /// If prefix is invalid.
    pub fn validate_prefix(&self, command: &CommandObject) -> Result<()> {
        if command.prefix == self.prefix {
            Ok(())
        } else {
            Err(Error::InvalidPrefix)
        }
    }

    /// # Errors
    /// If mention is invalid.
    pub async fn validate_mention(
        &self,
        command: &CommandObject,
        bot: &Bot<impl Session>,
    ) -> Result<()> {
        if self.ignore_mention {
            Ok(())
        } else if let Some(ref mention) = command.mention {
            if let Some(ref username) = bot.get_me(None).await?.username {
                if mention == username {
                    Ok(())
                } else {
                    Err(Error::InvalidMention)
                }
            } else {
                Err(Error::InvalidMention)
            }
        } else {
            Ok(())
        }
    }

    /// # Errors
    /// If command is invalid.
    pub fn validate_command(&self, command: &CommandObject) -> Result<()> {
        let command = if self.ignore_case {
            command.command.to_lowercase()
        } else {
            command.command.clone()
        };

        for pattern in &self.commands {
            match pattern {
                PatternType::Text(allowed_command) => {
                    if command == *allowed_command {
                        return Ok(());
                    }
                }
                PatternType::Regex(regex) => {
                    if regex.is_match(&command) {
                        return Ok(());
                    }
                }
                PatternType::Object(_) => unreachable!(
                    "`PatternType::Object` should be converted to `PatternType::Text` before validation"
                ),
            }
        }

        Err(Error::InvalidCommand)
    }

    /// # Errors
    /// - If prefix is invalid
    /// - If mention is invalid
    /// - If command is invalid
    pub async fn parse_command(
        &self,
        text: &str,
        bot: &Bot<impl Session>,
    ) -> Result<CommandObject> {
        let command = CommandObject::extract(text);

        self.validate_prefix(&command)?;
        self.validate_command(&command)?;
        self.validate_mention(&command, bot).await?;

        Ok(command)
    }
}

/// Represents parsed command from text
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CommandObject {
    /// Command without prefix and mention
    pub command: String,
    /// Command prefix
    pub prefix: String,
    /// Mention in command
    pub mention: Option<String>,
    /// Command arguments
    pub args: Vec<String>,
}

impl CommandObject {
    /// Extracts [`CommandObject`] from text
    #[must_use]
    pub fn extract(text: &str) -> Self {
        let result: Vec<_> = text.trim().split(' ').collect();
        let full_command = result[0].to_string();
        let args: Vec<String> = result[1..].iter().map(ToString::to_string).collect();

        let prefix = full_command[0..1].to_string();
        let command = full_command[1..].to_string();

        // Check if command contains mention, e.g. `/command@mention`, `/command@mention args`
        // and extract it, if it exists and isn't empty
        let (command, mention) = if command.contains('@') {
            let result: Vec<_> = command.split('@').collect();

            let command = result[0].to_string();
            let mention = result[1].to_string();

            let mention = if mention.is_empty() {
                None
            } else {
                Some(mention)
            };

            (command, mention)
        } else {
            (command, None)
        };

        CommandObject {
            command,
            prefix,
            mention,
            args,
        }
    }
}

#[async_trait]
impl<Client> Filter<Client> for Command<'_>
where
    Client: Session,
{
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        let Some(ref message) = update.message else { return false; };
        let Some(text) = message.get_text_or_caption() else { return false; };

        match self.parse_command(text, bot).await {
            Ok(command) => {
                context.insert("command", Box::new(command));
                true
            }
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_extract() {
        let command_obj = CommandObject::extract("/start");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@bot_username");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, Some("bot_username".to_string()));
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@bot_username arg1 arg2");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, Some("bot_username".to_string()));
        assert_eq!(command_obj.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_validate_prefix() {
        let command = Command::builder().prefix("/").command("start").build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/start_other");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("!start");
        assert!(command.validate_prefix(&command_obj).is_err());
    }

    #[test]
    fn test_validate_command() {
        let command = Command::builder()
            .prefix("/")
            .command("start")
            .ignore_case(false)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        let command = Command::builder()
            .prefix("/")
            .command("start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        // Special case: `command` with uppercase letters and `ignore_case` is `true`
        // command should be converted to lowercase
        let command = Command::builder()
            .prefix("/")
            .command("Start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());
    }

    // TODO: Add tests for `validate_mention` method
}
