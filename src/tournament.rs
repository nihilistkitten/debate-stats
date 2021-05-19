//! For representing individual tournaments.

use crate::{Event, Result};

mod from_url;

/// A tournament.
#[derive(Debug)]
pub struct Tournament {
    name: String,
    events: Vec<Event>,
}

impl Tournament {
    /// Create a new tournament from a url.
    ///
    /// # Errors
    /// Tournament construction may [error](crate::Error) if:
    ///  * An invalid URL was passed: [`UrlConversion`](crate::Error::UrlConversion)
    ///
    pub fn from_url(url: &str) -> Result<Self> {
        Self::from_url_impl(url)
    }
}
