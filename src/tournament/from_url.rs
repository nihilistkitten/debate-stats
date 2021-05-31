//! Generate tournaments from URLs.

use std::convert::{TryFrom, TryInto};

use crate::util::network;
use crate::{Error, Result, Tournament};
use network::process_url;
use url::Url;

mod tabroom;

impl Tournament {
    pub(super) fn from_url_impl(url_str: &str) -> Result<Self> {
        let url = process_url(url_str)?;
        match (&url).try_into()? {
            TabHost::Tabroom => Self::from_tabroom(&url),
        }
    }

    fn from_tabroom(url: &Url) -> Result<Self> {
        Self::from_tabroom_impl(url)
    }
}

/// The supported hosting websites.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum TabHost {
    /// tabroom.com
    Tabroom,
}

/// Determine the hosting website.
impl TryFrom<&Url> for TabHost {
    type Error = Error;

    fn try_from(value: &Url) -> Result<Self> {
        value.host_str().map_or(
            Err(Error::unsupported_host("unable to determine host")),
            |host| match host {
                "tabroom.com" | "www.tabroom.com" => Ok(Self::Tabroom),
                _ => Err(Error::unsupported_host(host)),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tabhost_from_str(url: &str) -> Result<TabHost> {
        (&process_url(url)?).try_into()
    }

    #[test]
    fn tabhost_try_from_tabroom_works() -> Result<()> {
        let result = get_tabhost_from_str("https://tabroom.com")?;
        assert_eq!(result, TabHost::Tabroom);
        Ok(())
    }

    #[test]
    fn tabhost_try_from_www_tabroom_works() -> Result<()> {
        let result = get_tabhost_from_str("https://www.tabroom.com")?;
        dbg!(&result);
        assert_eq!(result, TabHost::Tabroom);
        Ok(())
    }

    #[test]
    fn tabhost_try_from_tabroom_works_real_tournament() -> Result<()> {
        let result =
            get_tabhost_from_str("https://www.tabroom.com/index/tourn/index.mhtml?tourn_id=17253")?;
        assert_eq!(result, TabHost::Tabroom);
        Ok(())
    }

    // It's better to fail early, so we want to make sure we don't support Joy yet.
    #[test]
    fn tabhost_try_from_joy_works() {
        let result = get_tabhost_from_str("https://joyoftournaments.com");
        assert!(matches!(result.unwrap_err(), Error::UnsupportedHost(_)))
    }

    #[test]
    fn tabhost_try_from_example_org_works() {
        let result = get_tabhost_from_str("https://example.org");
        assert!(matches!(result.unwrap_err(), Error::UnsupportedHost(_)))
    }
}
