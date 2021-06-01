//! For representing individual tournaments.

mod from_url;

use chrono::NaiveDate;

use crate::{Event, Result};

/// A tournament.
#[derive(Debug)]
pub struct Tournament {
    name: String,
    events: Vec<Event>,
    start_date: NaiveDate,
}

impl Tournament {
    /// Create a new tournament from a url.
    ///
    /// We currently only support [tabroom.com](https://tabroom.com).
    ///
    /// # Example
    /// ```no_run
    /// # use debate_stats::Tournament;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let url = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
    /// let tournament = Tournament::from_url(url)?;
    /// assert_eq!(tournament.name(), "National Parliamentary Debate Invitational");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Tournament construction may error for a number of reasons. See [`Error`](crate::Error).
    pub fn from_url(url: &str) -> Result<Self> {
        Self::from_url_impl(url)
    }

    /// Get a reference to the tournament's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a reference to the tournament's events.
    ///
    /// # Example
    /// ```no_run
    /// # use debate_stats::Tournament;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let url = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
    /// let tournament = Tournament::from_url(url)?;
    /// assert_eq!(tournament.events()[0].name(), "JV Parli");
    /// assert_eq!(tournament.events()[1].name(), "Open Parli");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn events(&self) -> &Vec<Event> {
        &self.events
    }

    /// Get a reference to the tournament's start date.
    ///
    /// # Example
    /// ```no_run
    /// # use debate_stats::Tournament;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let url = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
    /// let tournament = Tournament::from_url(url)?;
    /// assert_eq!(tournament.start_date().to_string(), "2020-11-14");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }
}
