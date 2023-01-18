use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a contact message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputcontactmessagecontent>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a `vCard <https://en.wikipedia.org/wiki/VCard>`, 0-2048 bytes
    pub vcard: Option<String>,
}

impl InputContactMessageContent {
    #[must_use]
    pub fn new<T: Into<String>>(phone_number: T, first_name: T) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            ..Default::default()
        }
    }

    pub fn phone_number<T: Into<String>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<T: Into<String>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    pub fn last_name<T: Into<String>>(mut self, val: T) -> Self {
        self.last_name = Some(val.into());
        self
    }

    pub fn vcard<T: Into<String>>(mut self, val: T) -> Self {
        self.vcard = Some(val.into());
        self
    }
}
