//! Construct tournaments from tabroom.com.
//!
//! Thanks to tabroom for providing a convenient API!

use crate::util::network;
use crate::{Result, Tournament};
use reqwest::blocking::Response;
use url::Url;

impl Tournament {
    pub(super) fn from_tabroom_impl(url: Url) -> Result<Self> {
        let api_url = get_api_url(url)?;
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
///
/// Note that we might need to take a raw &str here to do more customized processing of the URL
/// than the url crate allows. If so, that will need to propogate up the call chain.
fn get_api_url(_url: Url) -> Result<Url> {
    todo!();
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
