use serde::{Deserialize, Serialize};

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfiles>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorFiles {
    /// Error source, must be *files*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the issue, one of 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration', 'temporary_registration'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<Box<str>>,
    /// Error message
    pub message: Box<str>,
}
