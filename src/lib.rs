//! A library for doing stats on competitive debate tournaments.
#![doc(html_root_url = "https://docs.rs/debate-stats/0.0.1")]
#![warn(rust_2018_idioms, missing_docs, missing_debug_implementations)]
#![deny(
    broken_intra_doc_links,
    invalid_codeblock_attributes,
    private_intra_doc_links
)]

mod entry;
mod error;
mod event;
mod tournament;
mod util;

pub use entry::Entry;
pub use error::{Error, Result, SearchingFor};
pub use event::{Event, EventKind};
pub use tournament::Tournament;
