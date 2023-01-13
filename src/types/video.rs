use super::PhotoSize;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a video file.
/// <https://core.telegram.org/bots/api#video>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// *Optional*. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// *Optional*. Original filename as defined by sender
    pub file_name: Option<String>,
    /// *Optional*. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
