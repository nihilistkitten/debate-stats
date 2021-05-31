//! Utilities related to networking.

use crate::{Error, Result};
use reqwest::blocking::{Client, Response};
use url::Url;

/// Turn a url string into a Url.
pub fn process_url(url: &str) -> Result<Url> {
    Url::parse(url).map_err(|e| Error::url_conversion(e, url))
}

/// Fetch a url.
pub fn fetch_url(_url: Url, _client: Option<Client>) -> Result<Response> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_url_works_base_only() -> Result<()> {
        let test_str = "https://example.org/";
        let url = process_url(test_str)?;
        assert_eq!(url.as_str(), test_str);
        Ok(())
    }

    #[test]
    fn process_url_works_long_url() -> Result<()> {
        let test_str = "https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253";
        let url = process_url(test_str)?;
        assert_eq!(url.as_str(), test_str);
        Ok(())
    }

    #[test]
    fn process_url_error_invalid_url() {
        let invalid_url = "abcdef";
        let err = process_url(invalid_url).unwrap_err();
        assert!(matches!(err, Error::UrlConversion { .. }));
        assert!(err.to_string().contains(invalid_url));
    }
}
