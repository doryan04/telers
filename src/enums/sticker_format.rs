use std::fmt::{self, Debug};

/// This enum represents all possible types of the sticker
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

impl Debug for StickerFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl StickerFormat {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            StickerFormat::Static => "static",
            StickerFormat::Animated => "animated",
            StickerFormat::Video => "video",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [StickerFormat; 3] {
        &[
            StickerFormat::Static,
            StickerFormat::Animated,
            StickerFormat::Video,
        ]
    }
}

impl<'a> PartialEq<&'a str> for StickerFormat {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl From<StickerFormat> for String {
    fn from(format: StickerFormat) -> Self {
        format.as_str().to_string()
    }
}
