//! Utilities related to networking.

use crate::Result;
use reqwest::blocking::{Client, Response};
use url::Url;

/// Turn a url string into a Url.
pub fn process_url(_url_str: &str) -> Result<Url> {
    todo!();
}

/// Fetch a url.
pub fn fetch_url(_url: Url, _client: Option<Client>) -> Result<Response> {
    todo!();
}
