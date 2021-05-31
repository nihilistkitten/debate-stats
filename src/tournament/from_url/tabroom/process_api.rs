//! Process the tabroom API into a tournament.

use super::orm;
use crate::{Result, Tournament};
use quick_xml::de::from_str;

/// Process the XML into a Tournament.
pub(super) fn process_api(xml: &str) -> Result<Tournament> {
    let _tournament_results = deserialize(xml);
    todo!();
}

fn deserialize(xml: &str) -> Result<orm::TournamentResults> {
    Ok(from_str(xml)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_works() -> Result<()> {
        let xml = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/npdi.xml"
        ));
        let tournament = deserialize(xml)?;
        assert_eq!(
            tournament.tourn.tourn_name,
            "National Parliamentary Debate Invitational"
        );
        Ok(())
    }
}
