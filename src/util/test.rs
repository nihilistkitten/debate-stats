//! Utilities for tests.

use crate::{Event, Result, Tournament};

use std::collections::HashSet;

use chrono::NaiveDate;

/// an input to a test
///
/// C is a struct for configuring the test
pub trait Input<O, C: Default> {
    /// the function under test
    fn under_test(self, config: C) -> Result<O>;
}

/// Create a test case out of a tournament.
///
/// This will take as many expected values as you want, and only run tests on the ones it's
/// given.
#[derive(Default)]
pub struct TournamentTestCase<T, C: Default>
where
    T: Input<Tournament, C> + Default,
{
    /// The test input
    input: T,

    /// The test config
    config: C,

    expected_name: Option<String>,
    expected_event_names: Option<HashSet<String>>,
    expected_event_abbrs: Option<HashSet<String>>,
    expected_debate_events: Option<usize>,
    expected_speech_events: Option<usize>,
    expected_start_date: Option<NaiveDate>,
    expected_end_date: Option<NaiveDate>,
}

impl<C: Default, T: Input<Tournament, C> + Default> TournamentTestCase<T, C> {
    /// Run the NPDI test case
    pub fn npdi(self) -> Self {
        self.name("National Parliamentary Debate Invitational")
            .events(vec!["JV Parli", "Open Parli"])
            .event_abbrs(vec!["JV", "Open"])
            .event_kind_counts(2, 0)
            .start_date(2020, 11, 14)
            .end_date(2020, 11, 16)
    }

    /// Run the Jack Howe test case
    pub fn jack_howe(self) -> Self {
        self.name("Jack Howe Memorial Tournament")
            .events(vec![
                "JV Policy",
                "Novice Dramatic Interpretation",
                "Novice Extemporaneous",
                "Novice Humorous Interpretation",
                "Novice Impromptu",
                "Novice Informative",
                "Novice Lincoln-Douglas",
                "Novice Oratorical Interpretation",
                "Novice Original Oratory",
                "Novice Policy",
                "Novice Program Oral Interpretation",
                "Novice Public Forum",
                "Open Congress",
                "Open Dramatic Interpretation",
                "Open Extemporaneous",
                "Open Humorous Interpretation",
                "Open Impromptu",
                "Open Informative",
                "Open Lincoln-Douglas - CA",
                "Open Lincoln-Douglas - TOC",
                "Open Oratorical Interpretation",
                "Open Original Oratory",
                "Open Policy",
                "Open Program Oral Interpretation",
                "Open Public Forum",
                "Parliamentary",
                "World School Debate",
            ])
            .event_abbrs(vec![
                "JVCX", "N DI", "N Ext", "N HI", "N Imp", "N Inf", "N LD", "N OI", "N OO", "N CX",
                "N POI", "N PF", "O Con", "O DI", "O Ext", "O HI", "O Imp", "O Inf", "CA LD",
                "TOCLD", "O OI", "O OO", "O CX", "O POI", "O PF", "Parli", "WSD",
            ])
            .event_kind_counts(9, 16) // congress and worlds are considered separate
            .start_date(2020, 9, 19)
            .end_date(2020, 9, 22)
    }

    /// Create a new test case
    pub fn new(input: T) -> Self {
        Self {
            input,
            ..Self::default()
        }
    }

    /// Configure the test
    pub fn config(self, config: C) -> Self {
        Self { config, ..self }
    }

    /// Run the test case
    pub fn run(self) -> Result<()> {
        let tournament = { self.input.under_test(self.config)? };

        if let Some(name) = self.expected_name {
            assert_eq!(name, tournament.name(), "Tournament names don't match.");
        }

        if let Some(event_names) = self.expected_event_names {
            // Collect to a hashset because we're ok with reordering
            assert_eq!(
                tournament
                    .events()
                    .iter()
                    .map(Event::name)
                    .map(String::from)
                    .collect::<HashSet<_>>(),
                event_names,
                "Event names don't match."
            )
        }

        if let Some(event_abbrs) = self.expected_event_abbrs {
            assert_eq!(
                tournament
                    .events()
                    .iter()
                    .map(Event::abbr)
                    .map(String::from)
                    .collect::<HashSet<_>>(),
                event_abbrs,
                "Event abbreviations don't match."
            );
        }

        if let Some(num_debate_events) = self.expected_debate_events {
            assert_eq!(
                num_debate_events,
                // for some reason the compiler makes us write a closure here instead of
                // Event::is_debate, with a very weird error message I don't want to pick
                // through
                tournament.events().iter().filter(|e| e.is_debate()).count(),
                "The number of debate events don't match."
            )
        }

        if let Some(num_speech_events) = self.expected_speech_events {
            assert_eq!(
                num_speech_events,
                // ditto, see above
                tournament.events().iter().filter(|e| e.is_speech()).count(),
                "The number of speech events don't match."
            )
        }

        if let Some(start_date) = self.expected_start_date {
            assert_eq!(
                tournament.start_date(),
                &start_date,
                "Start dates don't match."
            )
        }

        if let Some(end_date) = self.expected_end_date {
            assert_eq!(tournament.end_date(), &end_date, "End dates don't match.")
        }

        Ok(())
    }

    /// Add an expected name
    fn name(self, name: &str) -> Self {
        Self {
            expected_name: Some(name.into()),
            ..self
        }
    }

    /// Add names of expected events
    fn events(self, events: Vec<&str>) -> Self {
        Self {
            expected_event_names: Some(events.into_iter().map(String::from).collect()),
            ..self
        }
    }

    /// Add abbreviations of expected events
    fn event_abbrs(self, abbrs: Vec<&str>) -> Self {
        Self {
            expected_event_abbrs: Some(abbrs.into_iter().map(String::from).collect()),
            ..self
        }
    }

    /// Add expected event kind counts
    ///
    /// We test the counts instead of associating them with specific events because
    ///   a) it makes writing tests a lot easier
    ///   b) in the aggregate it's still highly likely it catches errors
    ///   c) it's nontrivial to write an impl of these tests which associates the kind with the
    ///      event
    #[allow(clippy::missing_const_for_fn)] // this is a false positive
    fn event_kind_counts(self, debate: usize, speech: usize) -> Self {
        Self {
            expected_debate_events: Some(debate),
            expected_speech_events: Some(speech),
            ..self
        }
    }

    /// Add the expected start date
    fn start_date(self, year: i32, month: u32, day: u32) -> Self {
        Self {
            expected_start_date: Some(NaiveDate::from_ymd(year, month, day)),
            ..self
        }
    }

    /// Add the expected end date
    fn end_date(self, year: i32, month: u32, day: u32) -> Self {
        Self {
            expected_end_date: Some(NaiveDate::from_ymd(year, month, day)),
            ..self
        }
    }
}
