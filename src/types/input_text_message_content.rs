use super::MessageEntity;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the [`content`](https://core.telegram.org/bots/api#inputmessagecontent) of a text message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputtextmessagecontent>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Mode for parsing entities in the message text. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in message text, which can be specified instead of *parse_mode*
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    #[must_use]
    pub fn new(message_text: impl Into<String>) -> Self {
        Self {
            message_text: message_text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }

    #[must_use]
    pub fn message_text(self, val: impl Into<String>) -> Self {
        Self {
            message_text: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn entity(self, val: MessageEntity) -> Self {
        Self {
            entities: Some(
                self.entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            entities: Some(
                self.entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn disable_web_page_preview(self, val: bool) -> Self {
        Self {
            disable_web_page_preview: Some(val),
            ..self
        }
    }
}
