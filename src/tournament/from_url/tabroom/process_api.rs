//! Process the tabroom API into a tournament.

use super::orm;
use crate::{Event, Result, Tournament};
use quick_xml::de;

/// Process the XML into a Tournament.
pub(super) fn process_api(xml: &str) -> Result<Tournament> {
    process_api_impl(xml, de::from_str)
}

fn process_api_impl(
    xml: &str,
    deserializer: impl FnOnce(&str) -> std::result::Result<orm::TournamentResults, de::DeError>,
) -> Result<Tournament> {
    // this takes the generic parameter so it can be used for inspect_serde in the TestCase struct
    let tournament_results = deserializer(xml)?;
    Ok(Tournament {
        name: tournament_results.tourn.tourn_name,
        events: tournament_results
            .events
            .into_iter()
            .map(|e| Event::new(e.event_name))
            .collect(),
        start_date: tournament_results.tourn.start_date,
    })
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    /// Create a test case out of a tournament.
    ///
    /// This will take as many expected values as you want, and only run tests on the ones it's
    /// given.
    #[derive(Default)]
    struct TestCase {
        inspect_serde: bool,

        /// The tournament's XML file.
        xml: String,
        /// The name we expect
        expected_name: Option<String>,
        /// The names of events we expect
        expected_event_names: Option<Vec<String>>,
        /// The start date we expect
        expected_start_date: Option<NaiveDate>,
    }

    impl TestCase {
        /// Create a new test case
        fn new(xml: String) -> Self {
            Self {
                xml,
                ..Self::default()
            }
        }

        /// Create a new test case from the name of the file's XML file in the resources/test dir
        fn from_file_name(name: &str) -> Self {
            let xml = std::fs::read_to_string(format!(
                "{}/resources/test/{}.xml",
                env!("CARGO_MANIFEST_DIR"),
                name
            ))
            .expect("The file name must be valid.");
            Self::new(xml)
        }

        /// Run the test case
        fn run(self) -> Result<()> {
            let tournament = {
                if self.inspect_serde {
                    process_api_impl(&self.xml, deserialize_and_inspect_error)?
                } else {
                    process_api(&self.xml)?
                }
            };
            if let Some(name) = self.expected_name {
                assert_eq!(name, tournament.name(), "Tournament names don't match.");
            }
            if let Some(event_names) = self.expected_event_names {
                assert_eq!(
                    tournament
                        .events()
                        .iter()
                        .map(Event::name)
                        .collect::<Vec<_>>(),
                    event_names,
                    "Event names don't match."
                )
            }
            if let Some(start_date) = self.expected_start_date {
                assert_eq!(
                    tournament.start_date(),
                    &start_date,
                    "Start dates don't match."
                )
            }
            Ok(())
        }

        /// Turn on serde error introspection. This will print the path at which the API failed to
        /// parse, if it fails to parse.
        #[allow(dead_code)] // this is only for when we're actuvely debugging
        #[allow(clippy::clippy::missing_const_for_fn)] // this is a false positive
        fn inspect_serde(self) -> Self {
            Self {
                inspect_serde: true,
                ..self
            }
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

        /// Add the expected start date
        fn start_date(self, year: i32, month: u32, day: u32) -> Self {
            Self {
                expected_start_date: Some(NaiveDate::from_ymd(year, month, day)),
                ..self
            }
        }
    }

    /// This uses `serde_path_to_error` to figure out where in the serialized data the error is.
    fn deserialize_and_inspect_error(
        xml: &str,
    ) -> std::result::Result<orm::TournamentResults, de::DeError> {
        // create the deserialized
        let d = &mut quick_xml::de::Deserializer::from_reader(xml.as_bytes());

        // let serde_path_to_error do its thing
        // the type annotation is necessary
        let result: std::result::Result<orm::TournamentResults, _> =
            serde_path_to_error::deserialize(d);
        // either return the result or print the failed path
        result.or_else(|e| {
            dbg!(e.path().to_string());
            panic!("An API parsing error occured; the path should be printed above.")
        })
    }

    #[test]
    fn process_api_works_npdi() -> Result<()> {
        TestCase::from_file_name("npdi")
            .name("National Parliamentary Debate Invitational")
            .events(vec!["JV Parli", "Open Parli"])
            .start_date(2020, 11, 14)
            .run()
    }

    #[test]
    fn process_api_works_jack_howe() -> Result<()> {
        TestCase::from_file_name("jack-howe")
            .name("Jack Howe Memorial Tournament")
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
            .start_date(2020, 9, 19)
            .run()
    }
}
