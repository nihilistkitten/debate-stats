//! For representing individual tournaments.

mod from_url;

use crate::{Event, Result};

/// A tournament.
#[derive(Debug)]
pub struct Tournament {
    name: String,
    events: Vec<Event>,
}

impl Tournament {
    /// Create a new tournament from a url.
    ///
    /// We currently only support [tabroom.com](https://tabroom.com).
    ///
    /// # Errors
    /// Tournament construction may [error](crate::Error) if:
    ///  * An invalid URL was passed: [`UrlConversion`](crate::Error::UrlConversion). Note that
    ///  URLs must include the protocol; i.e., `https://` or `http://`.
    ///  * A URL with an unknown tournament host was passed:
    ///  [`UnsupportedHost`](crate::Error::UnsupportedHost).
    ///  Note that the base URL must be taboom.com or www.tabroom.com; i.e., no shortlinks like
    ///  npdi.tabroom.com.
    ///
    pub fn from_url(url: &str) -> Result<Self> {
        Self::from_url_impl(url)
    }
}
