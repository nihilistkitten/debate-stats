//! Construct tournaments from tabroom.com.
//!
//! Thanks to tabroom for providing a convenient API!

mod orm;
mod process_api;

use crate::util::network;
use crate::{Error, Result, SearchingFor, Tournament};
use network::{fetch_url, process_url};
use process_api::process_api;
use reqwest::blocking::Response;
use url::Url;

impl Tournament {
    pub(super) fn from_tabroom_impl(url: &Url) -> Result<Self> {
        let api_url = get_api_url(url)?;
        // TODO: Should we call with a client here?
        let resp = fetch_url(api_url, None)?;
        let xml = get_xml_body(resp)?;
        process_api(&xml)
    }
}

/// Get the appropriate URL for the tabroom API for this tournament.
///
/// # Errors
/// If the tournament ID can't be resolved from the URL.
fn get_api_url(url: &Url) -> Result<Url> {
    let id = get_id(url)?;
    let api_url_str = format!(
        "https://www.tabroom.com/api/tourn_published.mhtml?tourn_id={}",
        id
    );
    process_url(&api_url_str)
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
fn get_xml_body(response: Response) -> Result<String> {
    dbg!(response.status());
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(Error::HttpRequest(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_api_url_works() -> Result<()> {
        assert_eq!(
            get_api_url(
                &(process_url("https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253"))?
            )?
            .as_str(),
            "https://www.tabroom.com/api/tourn_published.mhtml?tourn_id=17253"
        );
        Ok(())
    }

    #[test]
    fn get_id_works() -> Result<()> {
        assert_eq!(
            get_id(
                &(process_url("https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253"))?
            )?,
            17253
        );
        Ok(())
    }

    #[test]
    fn get_id_works_pariings() -> Result<()> {
        assert_eq!(
            get_id(
                &(process_url(
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
                &(process_url("https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=16814"))?
            )?,
            16814
        );
        Ok(())
    }

    #[test]
    fn get_id_errors_base_tabroom() -> Result<()> {
        let err = get_id(&(process_url("https://tabroom.com"))?).unwrap_err();
        assert!(matches!(err, Error::HtmlParseFailed(SearchingFor::ID)));
        Ok(())
    }
}
