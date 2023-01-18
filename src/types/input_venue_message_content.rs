use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a venue message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputvenuemessagecontent>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// *Optional*. Foursquare type of the venue, if known. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// *Optional*. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// *Optional*. Google Places type of the venue. (See `supported types <https://developers.google.com/places/web-service/supported_types>`.)
    pub google_place_type: Option<String>,
}

impl InputVenueMessageContent {
    #[must_use]
    pub fn new<T: Into<String>>(latitude: f64, longitude: f64, title: T, address: T) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn address<T: Into<String>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    #[must_use]
    pub fn foursquare_id<T: Into<String>>(mut self, val: T) -> Self {
        self.foursquare_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn foursquare_type<T: Into<String>>(mut self, val: T) -> Self {
        self.foursquare_type = Some(val.into());
        self
    }

    #[must_use]
    pub fn google_place_id<T: Into<String>>(mut self, val: T) -> Self {
        self.google_place_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn google_place_type<T: Into<String>>(mut self, val: T) -> Self {
        self.google_place_type = Some(val.into());
        self
    }
}
