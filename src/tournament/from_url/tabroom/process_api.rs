//! Process the tabroom API into a tournament.

use std::collections::HashMap;

use super::orm;
use crate::{Entry, Event, Result, Tournament};
use quick_xml::de;

/// A result returned by tournament deserialization
type DeResult = std::result::Result<orm::TournamentResults, de::DeError>;

/// Process the XML into a Tournament.
pub(super) fn process_api(xml: &str) -> Result<Tournament> {
    process_api_impl(xml, de::from_str)
}

/// An object that will later need to be appended
struct AssociatedId<T> {
    assoeciated_id: i32,
    inner: T,
}

/// A ballot. The ids of the two teams.
struct Ballot<'a>(Option<&'a Entry>, Option<&'a Entry>);

// this takes the generic parameter so it can be used for inspect_serde in the TestCase struct
fn process_api_impl(xml: &str, deserializer: impl FnOnce(&str) -> DeResult) -> Result<Tournament> {
    let api = deserializer(xml)?;

    let mut events = process_events(api.events);
    let entries = process_entries(api.entries);
    let _ballots = process_ballots(api.ballots, &entries);

    for (_, entry) in entries {
        events
            .get_mut(&entry.assoeciated_id)
            .expect("tab gives valid event ids")
            .push_entry(entry.inner);
    }

    Ok(Tournament {
        name: api.tourn.tourn_name,
        events: events.into_iter().map(|(_, e)| e).collect(),
        start_date: api.tourn.start_date,
        end_date: api.tourn.end_date,
    })
}

fn process_events(api_events: Vec<orm::Event>) -> HashMap<i32, Event> {
    api_events
        .into_iter()
        .map(|e| (e.id, Event::new(e.abbr, e.event_name, e.event_type.into())))
        .collect()
}

fn process_entries(api_entries: Vec<orm::Entry>) -> HashMap<i32, AssociatedId<Entry>> {
    api_entries
        .into_iter()
        .map(|e| {
            (
                e.id,
                AssociatedId {
                    inner: Entry::new(e.code, e.full_name),
                    assoeciated_id: e.event,
                },
            )
        })
        .collect()
}

fn process_ballots(
    api_ballots: Vec<orm::Ballot>,
    entries: &HashMap<i32, AssociatedId<Entry>>,
) -> HashMap<i32, Ballot<'_>> {
    let mut ballots = HashMap::default();
    for ballot in api_ballots {
        if let Some(entry) = entries.get(&ballot.entry).map(|e| &e.inner) {
            ballots
                .entry(ballot.id)
                .and_modify(|Ballot(_, e)| {
                    *e = Some(entry);
                })
                .or_insert_with(|| Ballot(Some(entry), None));
        }
    }
    ballots
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::{ExpectedOutput, Input, TournamentTest};

    /// This uses `serde_path_to_error` to figure out where in the serialized data the error is.
    fn deserialize_and_inspect_error(xml: &str) -> DeResult {
        // create the deserializer
        let d = &mut de::Deserializer::from_reader(xml.as_bytes());

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

    #[derive(Default)]
    struct XmlInput(String);

    impl From<String> for XmlInput {
        fn from(xml: String) -> Self {
            Self(xml)
        }
    }

    impl XmlInput {
        fn from_file_name(name: &str) -> Self {
            std::fs::read_to_string(format!(
                "{}/resources/test/{}.xml",
                env!("CARGO_MANIFEST_DIR"),
                name
            ))
            .expect("The file name must be valid.")
            .into()
        }
    }

    impl Input<bool, Tournament> for XmlInput {
        fn under_test(self, inspect_serde: bool) -> Tournament {
            if inspect_serde {
                process_api_impl(&self.0, deserialize_and_inspect_error)
            } else {
                process_api(&self.0)
            }
            .expect("unable to parse the api")
        }
    }

    #[test]
    fn process_api_works_npdi() {
        TournamentTest::npdi()
            .input(XmlInput::from_file_name("npdi"))
            .run()
    }

    #[test]
    fn process_api_works_jack_howe() {
        TournamentTest::jack_howe()
            .input(XmlInput::from_file_name("jack-howe"))
            .run()
    }
}
