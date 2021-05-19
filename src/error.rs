//! For handling errors.

use std::result::Result as StdResult;

/// A [`Result`](StdResult) alias where the failure case is an [`Error`].
pub type Result<T> = StdResult<T, Error>;

/// Enumerates all possible errors returned by the library.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An invalid URL was passed.
    UrlConversion,
}
