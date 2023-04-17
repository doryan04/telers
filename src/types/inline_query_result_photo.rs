use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultphoto>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be *photo*
    #[serde(rename = "type", default = "photo")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// Width of the photo
    pub photo_width: Option<i64>,
    /// Height of the photo
    pub photo_height: Option<i64>,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultPhoto {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, photo_url: T, thumb_url: T) -> Self {
        Self {
            id: id.into(),
            photo_url: photo_url.into(),
            thumb_url: thumb_url.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id(mut self, val: impl Into<String>) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn photo_url(mut self, val: impl Into<String>) -> Self {
        self.photo_url = val.into();
        self
    }

    #[must_use]
    pub fn thumb_url(mut self, val: impl Into<String>) -> Self {
        self.thumb_url = val.into();
        self
    }

    #[must_use]
    pub fn photo_width(mut self, val: i64) -> Self {
        self.photo_width = Some(val);
        self
    }

    #[must_use]
    pub fn photo_height(mut self, val: i64) -> Self {
        self.photo_height = Some(val);
        self
    }

    #[must_use]
    pub fn title(mut self, val: impl Into<String>) -> Self {
        self.title = Some(val.into());
        self
    }

    #[must_use]
    pub fn description(mut self, val: impl Into<String>) -> Self {
        self.description = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
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
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}

impl Default for InlineQueryResultPhoto {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: photo(),
            id: String::default(),
            photo_url: String::default(),
            thumb_url: String::default(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn photo() -> String {
    InlineQueryResultType::Photo.into()
}
