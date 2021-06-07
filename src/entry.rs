//! For representing a single entry.

/// A single entry.
#[derive(Debug)]
pub struct Entry {
    code: String,
    full_name: String,
}

impl Entry {
    /// Create a new entry.
    #[allow(clippy::missing_const_for_fn, clippy::must_use_candidate)] // these won't be true in the future
    pub(crate) fn new(code: String, full_name: String) -> Self {
        Self { code, full_name }
    }

    /// The entry's full name.
    ///
    /// This is the "Entry" column on tab's Entries display.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// The entry's code.
    ///
    /// This is the "Code" column on tab's Entries display.
    #[must_use]
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}
