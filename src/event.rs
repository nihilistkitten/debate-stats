//! For representing a single event.

/// A single event.
#[derive(Debug)]
pub struct Event {
    name: String,
}

impl Event {
    pub(crate) const fn new(name: String) -> Self {
        Self { name }
    }

    /// Get a reference to the event's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
