//! Construct tournaments from tabroom.com.
//!
//! Thanks to tabroom for providing a convenient API!

use crate::util::network;
use crate::{Error, Result, SearchingFor, Tournament};
use reqwest::blocking::Response;
use url::Url;

impl Tournament {
    pub(super) fn from_tabroom_impl(url: Url) -> Result<Self> {
        let api_url = get_api_url(&url)?;
        // TODO: Should we call with a client here?
        let resp = network::fetch_url(api_url, None)?;
        let xml = get_xml_body(resp)?;
        process_api(xml)
    }
}

/// Get the appropriate URL for the tabroom API for this tournament.
///
/// # Errors
/// If the tournament ID can't be resolved from the URL.
fn get_api_url(url: &Url) -> Result<Url> {
    todo!();
}

/// Get the tournament's ID from any associated URL.
fn get_id(url: &Url) -> Result<u32> {
    const ERROR: Error = Error::HtmlParseFailed(SearchingFor::ID);

    url.query_pairs()
        .find(|(k, _)| k == "tourn_id")
        .ok_or(ERROR)?
        .1
        .into_owned()
        .parse()
        .or(Err(ERROR))
}

/// Get the XML API body from the reqwest response.
fn get_xml_body(_response: Response) -> Result<String> {
    todo!();
}

/// Process the XML into a Tournament.
fn process_api(_xml: String) -> Result<Tournament> {
    // Impl: probably this should use serde and quick-xml
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_works() -> Result<()> {
        assert_eq!(
            get_id(
                &(network::process_url(
                    "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253"
                ))?
            )?,
            17253
        );
        Ok(())
    }

    #[test]
    fn get_id_works_pariings() -> Result<()> {
        assert_eq!(
            get_id(
                &(network::process_url(
                    "https://www.tabroom.com/index/tourn/postings/round.mhtml?tourn_id=17253&round_id=622046"
                ))?
            )?,
            17253
        );
        Ok(())
    }

    #[test]
    fn get_id_works_jack_howe() -> Result<()> {
        assert_eq!(
            get_id(
                &(network::process_url(
                    "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=16814"
                ))?
            )?,
            16814
        );
        Ok(())
    }

    #[test]
    fn get_id_errors_base_tabroom() -> Result<()> {
        let err = get_id(&(network::process_url("https://tabroom.com"))?).unwrap_err();
        assert!(matches!(err, Error::HtmlParseFailed(SearchingFor::ID)));
        Ok(())
    }
}
