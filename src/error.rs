//! For handling errors.

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

/// A [`Result`](StdResult) alias where the failure case is an [`Error`].
pub type Result<T> = StdResult<T, Error>;

/// Enumerates all possible errors returned by the library.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An invalid URL was passed.
    UrlConversion {
        /// The URL that failed to convert.
        invalid_url: String,
        /// The source error from the [`url`](url) library.
        source: url::ParseError,
    },

    /// The URL was from an unsupported tournament hosting platform.
    ///
    /// Only certain tournament hosts are supported; see
    /// [`Tournament::from_url`](crate::Tournament::from_url)
    UnsupportedHost(String),

    /// A tournament HTML page failed to parse.
    HtmlParseFailed(
        /// The tournament data we were unable to parse from the tournament page.
        SearchingFor,
    ),

    /// Wraps [`reqwest::Error`] with the URL that couldn't be accessed correctly.
    HttpRequest(reqwest::Error),
}

/// The types of tournament data that we can fail to parse.
#[derive(Debug)]
#[non_exhaustive]
pub enum SearchingFor {
    /// The tournament's ID number.
    ID,
}

impl Display for SearchingFor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            Self::ID => "ID".to_string(),
        };
        write!(f, "{}", message)
    }
}

impl Error {
    /// Generate an `Error::UrlConversion` from a `url::ParseError`.
    pub(crate) fn url_conversion(source: url::ParseError, invalid_url: &str) -> Self {
        Self::UrlConversion {
            invalid_url: invalid_url.to_string(),
            source,
        }
    }

    /// Generate an `Error::UnsupportedHost` from the host as a &str.
    pub(crate) fn unsupported_host(host: &str) -> Self {
        Self::UnsupportedHost(host.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            Self::UrlConversion {
                source,
                invalid_url,
            } => {
                format!("unable to convert {} to a url: {}", invalid_url, source)
            }
            Self::UnsupportedHost(host) => {
                format!("we don't currently scrape that tournament host: {}", host)
            }
            Self::HtmlParseFailed(inner) => format!("unable to find the tournament's {}", inner),
            Self::HttpRequest(source) => {
                format!(
                    "unable to scrape url '{}': {}",
                    source.url().map_or(
                        "internal error: no url found".to_string(),
                        url::Url::to_string
                    ),
                    source
                )
            }
        };
        write!(f, "{}", message)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::UrlConversion { source, .. } => Some(source),
            Self::HttpRequest(source) => Some(source),
            Self::UnsupportedHost(_) | Self::HtmlParseFailed(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Self::HttpRequest(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_conversion_display_works() {
        let source = url::Url::parse("abcdef").expect_err("the url is invalid");
        assert_eq!(
            format!(
                "{}",
                Error::UrlConversion {
                    source,
                    invalid_url: "abcdef".to_string()
                }
            ),
            format!("unable to convert abcdef to a url: {}", source)
        );
    }
}
