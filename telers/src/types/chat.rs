use super::{
    Birthdate, BusinessIntro, BusinessLocation, BusinessOpeningHours, ChatLocation,
    ChatPermissions, ChatPhoto, Message,
};

use crate::extractors::FromContext;

use serde::Deserialize;

/// This object represents a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Debug, Clone, PartialEq, Deserialize, FromContext)]
#[context(
    key = "event_chat",
    description = "This object represents a chat. \
    This context is available only if `UserContext` middleware is used (default middleware) and chat in `Update` is not empty."
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Chat {
    Private(Box<Private>),
    Group(Box<Group>),
    Supergroup(Box<Supergroup>),
    Channel(Box<Channel>),
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Private {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Username
    pub username: Option<Box<str>>,
    /// First name of the other party
    pub first_name: Option<Box<str>>,
    /// Last name of the other party
    pub last_name: Option<Box<str>>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// The date of birth of the user. Returned only in [`GetChat`](crate::methods::GetChat).
    pub birthdate: Option<Birthdate>,
    /// The intro of the business. Returned only in [`GetChat`](crate::methods::GetChat).
    pub business_intro: Option<BusinessIntro>,
    /// The location of the business. Returned only in [`GetChat`](crate::methods::GetChat).
    pub business_location: Option<BusinessLocation>,
    /// The opening hours of the business. Returned only in [`GetChat`](crate::methods::GetChat).
    pub business_opening_hours: Option<BusinessOpeningHours>,
    /// The personal channel of the user. Returned only in [`GetChat`](crate::methods::GetChat).
    pub personal_chat: Option<Chat>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of emoji status of the other party. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status of the other party in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// Bio of the other party. Returned only in [`GetChat`](crate::methods::GetChat).
    pub bio: Option<Box<str>>,
    /// `true`, if privacy settings of the other party allows to use `tg://user?id=<user_id>` links only in chats with the user. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_private_forwards: Option<bool>,
    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Message>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Message>,
    /// Default chat member permissions. Returned only in [`GetChat`](crate::methods::GetChat).
    pub permissions: Option<ChatPermissions>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_visible_history: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Supergroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
    /// `true`, if the chat is a forum (has [`topics`](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    pub is_forum: Option<bool>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api#reactiontypeemoji) are allowed. Returned only in [`GetChat`](crate::methods::GetChat).
    pub available_reactions: Option<Box<[Box<str>]>>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of the emoji status. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// `true`, if users need to join the supergroup before they can send messages. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_to_send_messages: Option<bool>,
    /// `true`, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_by_request: Option<bool>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Message>,
    /// Default chat member permissions. Returned only in [`GetChat`](crate::methods::GetChat).
    pub permissions: Option<ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub slow_mode_delay: Option<i64>,
    /// The minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions. Returned only in [`GetChat`](crate::methods::GetChat).
    pub unrestrict_boost_count: Option<i64>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_visible_history: Option<bool>,
    /// Name of group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub sticker_set_name: Option<Box<str>>,
    /// `true`, if the bot can change the group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub can_set_sticker_set: Option<bool>,
    /// The name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group. Returned only in [`GetChat`](crate::methods::GetChat).
    pub custom_emoji_sticker_set_name: Option<Box<str>>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [`GetChat`](crate::methods::GetChat).
    pub linked_chat_id: Option<i64>,
    /// The location to which the supergroup is connected. Returned only in [`GetChat`](crate::methods::GetChat).
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Channel {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api#reactiontypeemoji) are allowed. Returned only in [`GetChat`](crate::methods::GetChat).
    pub available_reactions: Option<Box<[Box<str>]>>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of the emoji status. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Message>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [`GetChat`](crate::methods::GetChat).
    pub linked_chat_id: Option<i64>,
}

impl Chat {
    #[must_use]
    pub const fn id(&self) -> i64 {
        match self {
            Self::Private(chat) => chat.id,
            Self::Group(chat) => chat.id,
            Self::Supergroup(chat) => chat.id,
            Self::Channel(chat) => chat.id,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn username(&self) -> Option<&str> {
        match self {
            Self::Group(_) => None,
            Self::Private(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
            Self::Supergroup(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
            Self::Channel(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn photo(&self) -> Option<&ChatPhoto> {
        match self {
            Self::Private(chat) => chat.photo.as_ref(),
            Self::Group(chat) => chat.photo.as_ref(),
            Self::Supergroup(chat) => chat.photo.as_ref(),
            Self::Channel(chat) => chat.photo.as_ref(),
        }
    }

    #[must_use]
    pub const fn title(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(chat) => Some(&chat.title),
            Self::Supergroup(chat) => Some(&chat.title),
            Self::Channel(chat) => Some(&chat.title),
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn active_usernames(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Group(_) => None,
            Self::Private(chat) => match chat.active_usernames {
                Some(ref active_usernames) => Some(active_usernames),
                None => None,
            },
            Self::Supergroup(chat) => match chat.active_usernames {
                Some(ref active_usernames) => Some(active_usernames),
                None => None,
            },
            Self::Channel(chat) => match chat.active_usernames {
                Some(ref active_usernames) => Some(active_usernames),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn birthdate(&self) -> Option<&Birthdate> {
        match self {
            Self::Group(_) | Self::Supergroup(_) | Self::Channel(_) => None,
            Self::Private(chat) => chat.birthdate.as_ref(),
        }
    }

    #[must_use]
    pub const fn business_intro(&self) -> Option<&BusinessIntro> {
        match self {
            Self::Group(_) | Self::Supergroup(_) | Self::Channel(_) => None,
            Self::Private(chat) => chat.business_intro.as_ref(),
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn business_location(&self) -> Option<&BusinessLocation> {
        match self {
            Self::Group(_) | Self::Supergroup(_) | Self::Channel(_) => None,
            Self::Private(chat) => match chat.business_location {
                Some(ref business_location) => Some(business_location),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn business_opening_hours(&self) -> Option<&BusinessOpeningHours> {
        match self {
            Self::Group(_) | Self::Supergroup(_) | Self::Channel(_) => None,
            Self::Private(chat) => chat.business_opening_hours.as_ref(),
        }
    }

    #[must_use]
    pub const fn personal_chat(&self) -> Option<&Chat> {
        match self {
            Self::Group(_) | Self::Supergroup(_) | Self::Channel(_) => None,
            Self::Private(chat) => chat.personal_chat.as_ref(),
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn available_reactions(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(chat) => match chat.available_reactions {
                Some(ref available_reactions) => Some(available_reactions),
                None => None,
            },
            Self::Channel(chat) => match chat.available_reactions {
                Some(ref available_reactions) => Some(available_reactions),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(chat) => chat.accent_color_id,
            Self::Channel(chat) => chat.accent_color_id,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(chat) => match chat.background_custom_emoji_id {
                Some(ref background_custom_emoji_id) => Some(background_custom_emoji_id),
                None => None,
            },
            Self::Channel(chat) => match chat.background_custom_emoji_id {
                Some(ref background_custom_emoji_id) => Some(background_custom_emoji_id),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn profile_accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(chat) => chat.profile_accent_color_id,
            Self::Channel(chat) => chat.profile_accent_color_id,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn profile_background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(chat) => match chat.profile_background_custom_emoji_id {
                Some(ref profile_background_custom_emoji_id) => {
                    Some(profile_background_custom_emoji_id)
                }
                None => None,
            },
            Self::Channel(chat) => match chat.profile_background_custom_emoji_id {
                Some(ref profile_background_custom_emoji_id) => {
                    Some(profile_background_custom_emoji_id)
                }
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn has_visible_history(&self) -> Option<bool> {
        match self {
            Self::Private(_) | Self::Channel(_) => None,
            Self::Group(chat) => chat.has_visible_history,
            Self::Supergroup(chat) => chat.has_visible_history,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn description(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(chat) => match chat.description {
                Some(ref description) => Some(description),
                None => None,
            },
            Self::Supergroup(chat) => match chat.description {
                Some(ref description) => Some(description),
                None => None,
            },
            Self::Channel(chat) => match chat.description {
                Some(ref description) => Some(description),
                None => None,
            },
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn invite_link(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(chat) => match chat.invite_link {
                Some(ref invite_link) => Some(invite_link),
                None => None,
            },
            Self::Supergroup(chat) => match chat.invite_link {
                Some(ref invite_link) => Some(invite_link),
                None => None,
            },
            Self::Channel(chat) => match chat.invite_link {
                Some(ref invite_link) => Some(invite_link),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Private(_) => None,
            Self::Group(chat) => chat.has_protected_content,
            Self::Supergroup(chat) => chat.has_protected_content,
            Self::Channel(chat) => chat.has_protected_content,
        }
    }

    #[must_use]
    pub const fn linked_chat_id(&self) -> Option<i64> {
        match self {
            Self::Private(_) | Self::Group(_) => None,
            Self::Supergroup(chat) => chat.linked_chat_id,
            Self::Channel(chat) => chat.linked_chat_id,
        }
    }
}

impl Default for Chat {
    #[must_use]
    fn default() -> Self {
        Self::Private(Box::default())
    }
}
