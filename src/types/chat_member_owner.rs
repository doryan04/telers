use super::User;

use crate::enums::ChatMemberStatus;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that owns the chat and has all administrator privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always 'creator'
    #[serde(default = "owner")]
    pub status: Box<str>,
    /// Information about the user
    pub user: User,
    /// `True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<Box<str>>,
}

fn owner() -> Box<str> {
    ChatMemberStatus::Creator.into()
}
