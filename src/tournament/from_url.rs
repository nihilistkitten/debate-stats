//! Generate tournaments from URLs.

use std::convert::{TryFrom, TryInto};

use crate::util::network;
use crate::{Error, Result, Tournament};
use url::Url;

mod tabroom;

impl Tournament {
    pub(super) fn from_url_impl(url_str: &str) -> Result<Self> {
        let url = network::process_url(url_str)?;
        match (&url).try_into()? {
            TabHost::_Tabroom => Self::from_tabroom(url),
        }
    }

    fn from_tabroom(url: Url) -> Result<Self> {
        Self::from_tabroom_impl(url)
    }
}

/// The supported hosting websites.
#[non_exhaustive]
enum TabHost {
    _Tabroom,
}

/// Determine the hosting website.
impl TryFrom<&Url> for TabHost {
    type Error = Error;

    fn try_from(_value: &Url) -> Result<Self> {
        todo!()
    }
}
