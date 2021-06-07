//! For representing a single entry.

/// A single entry.
#[derive(Debug)]
pub struct Entry {
    code: String,
    full_name: String,
}

impl Entry {
    /// Create a new entry.
    #[allow(clippy::clippy::missing_const_for_fn, clippy::clippy::must_use_candidate)] // these won't be true in the future
    pub fn new(code: String, full_name: String) -> Self {
        Self { code, full_name }
    }

    /// Get a reference to the entry's code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get a reference to the entry's full name.
    #[must_use]
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}
