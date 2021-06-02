//! Process the tabroom API into a tournament.

use super::orm;
use crate::{Event, Result, Tournament};
use quick_xml::de;

/// A result returned by tournament deserialization
type DeResult = std::result::Result<orm::TournamentResults, de::DeError>;

/// Process the XML into a Tournament.
pub(super) fn process_api(xml: &str) -> Result<Tournament> {
    process_api_impl(xml, de::from_str)
}

fn process_api_impl(xml: &str, deserializer: impl FnOnce(&str) -> DeResult) -> Result<Tournament> {
    // this takes the generic parameter so it can be used for inspect_serde in the TestCase struct
    let api = deserializer(xml)?;

    Ok(Tournament {
        name: api.tourn.tourn_name,
        events: api
            .events
            .into_iter()
            .map(|e| Event::new(e.abbr, e.event_name, e.event_type.into()))
            .collect(),
        start_date: api.tourn.start_date,
        end_date: api.tourn.end_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::{Input, TournamentTestCase};

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

    impl Input<Tournament, bool> for XmlInput {
        fn under_test(self, inspect_serde: bool) -> Result<Tournament> {
            if inspect_serde {
                process_api_impl(&self.0, deserialize_and_inspect_error)
            } else {
                process_api(&self.0)
            }
        }
    }

    impl TournamentTestCase<XmlInput, bool> {
        #[allow(dead_code)]
        /// Turn this on if you want to see where the serde error happened
        fn inspect_serde(self) -> Self {
            self.config(true)
        }
    }

    #[test]
    fn process_api_works_npdi() -> Result<()> {
        TournamentTestCase::new(XmlInput::from_file_name("npdi"))
            .npdi()
            .run()
    }

    #[test]
    fn process_api_works_jack_howe() -> Result<()> {
        TournamentTestCase::new(XmlInput::from_file_name("jack-howe"))
            .jack_howe()
            .run()
    }
}
