//! For representing a single event.

use std::fmt::Display;

use crate::Entry;

/// A single event.
#[derive(Debug)]
pub struct Event {
    abbr: String,
    name: String,
    kind: EventKind,
    entries: Vec<Entry>,
}

#[derive(Clone, Debug, PartialEq)]
/// The possible kinds of events.
#[allow(clippy::module_name_repetitions)] // this seems warranted here
pub enum EventKind {
    /// A debate event.
    Debate,
    /// A speech event.
    Speech,
    /// Some events categorize congress, worlds, etc. as neither speech nor debate.
    ///
    /// The value is the name the tab api reports for the type of event.
    Other(String),
}

impl From<String> for EventKind {
    fn from(s: String) -> Self {
        match s.as_str() {
            "debate" => Self::Debate,
            "speech" => Self::Speech,
            _ => Self::Other(s),
        }
    }
}

impl Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Debate => "debate",
            Self::Speech => "speech",
            Self::Other(s) => s,
        };
        write!(f, "{}", message)
    }
}

impl Event {
    /// Create a new event.
    ///
    /// The new event has no entries.
    pub(crate) const fn new(abbr: String, name: String, kind: EventKind) -> Self {
        Self {
            abbr,
            name,
            kind,
            entries: vec![],
        }
    }

    /// Push an entry
    pub(crate) fn push_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// The event's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The event's abbreviation.
    ///
    /// Think "VPD" instead of "Varsity Parliamentary Debate;" different tournaments obviously have
    /// different schemes.
    #[must_use]
    pub fn abbr(&self) -> &str {
        &self.abbr
    }

    /// The event's entries.
    #[must_use]
    pub const fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// The kind of event (i.e., the subcategory of forensics).
    ///
    /// For more info see [`EventKind`].
    #[must_use]
    pub const fn kind(&self) -> &EventKind {
        &self.kind
    }

    /// Check whether the event is a debate event.
    #[must_use]
    pub fn is_debate(&self) -> bool {
        self.kind() == &EventKind::Debate
    }

    /// Check whether the event is a speech event.
    #[must_use]
    pub fn is_speech(&self) -> bool {
        self.kind() == &EventKind::Speech
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_kind_from_debate_works() {
        assert_eq!(EventKind::Debate, "debate".to_string().into())
    }

    #[test]
    fn event_kind_from_speech_works() {
        assert_eq!(EventKind::Speech, "speech".to_string().into())
    }

    #[test]
    fn event_kind_from_other_works() {
        let kind = "unknown";
        assert_eq!(EventKind::Other(kind.into()), kind.to_string().into())
    }

    #[test]
    fn event_new_has_no_entries() {
        assert!(Event::new("".into(), "".into(), EventKind::Debate)
            .entries()
            .is_empty());
    }

    #[test]
    fn push_entry_works() {
        let mut event = Event::new("".into(), "".into(), EventKind::Debate);
        event.push_entry(Entry::new("".into(), "".into()));
        assert_eq!(event.entries().len(), 1)
    }
}
