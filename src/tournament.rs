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
    end_date: NaiveDate,
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

    /// The tournament's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The tournament's events.
    ///
    /// Note that we make no ordering guarantee; events may appear in any order.
    ///
    /// # Example
    /// ```no_run
    /// # use debate_stats::Tournament;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let url = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
    /// let tournament = Tournament::from_url(url)?;
    /// assert!(tournament.events().iter().any(|e| e.name() == "JV Parli"));
    /// assert!(tournament.events().iter().any(|e| e.name() == "Open Parli"));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    /// Note the use of [`Iterator::any`]; this is because of the lack of ordering guarantees.
    #[must_use]
    pub const fn events(&self) -> &Vec<Event> {
        &self.events
    }

    /// The tournament's start date.
    ///
    /// This and the end date are [`NaiveDate`]s because tabroom doesn't provide time zone data.
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

    /// The tournament's end date.
    ///
    /// Note that this is one day after the last day of the tournament; i.e., a day later than the
    /// end of the range displayed in the "Tournament" section under "Dates & Deadlines" in the tab
    /// sidebar.
    ///
    /// # Example
    /// ```no_run
    /// # use debate_stats::Tournament;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let url = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
    /// let tournament = Tournament::from_url(url)?;
    /// assert_eq!(tournament.end_date().to_string(), "2020-11-16"); // the tournament ended 11/15
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::{ExpectedOutput, Input, TournamentTest};

    struct UrlInput(String);

    impl From<&str> for UrlInput {
        fn from(url: &str) -> Self {
            Self(url.into())
        }
    }

    impl Input<(), Tournament> for UrlInput {
        fn under_test(self, _: ()) -> Tournament {
            Tournament::from_url(&self.0)
                .unwrap_or_else(|e| panic!("tournament construction failed: {:?}", e))
        }
    }

    #[test]
    #[ignore = "the tab api randomly 404s"]
    fn from_url_works_npdi() {
        TournamentTest::npdi()
            .input(UrlInput::from(
                "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253",
            ))
            .run()
    }

    #[test]
    #[ignore = "the tab api randomly 404s"]
    fn from_url_works_jack_howe() {
        TournamentTest::jack_howe()
            .input(UrlInput::from(
                "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=16814",
            ))
            .run()
    }
}
