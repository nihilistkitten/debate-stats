//! For representing a single event.

/// A single event.
#[derive(Debug)]
pub struct Event {
    abbr: String,
    name: String,
    kind: EventKind,
}

#[derive(Clone, Debug, PartialEq)]
/// The kind of event.
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

impl Event {
    pub(crate) const fn new(abbr: String, name: String, kind: EventKind) -> Self {
        Self { abbr, name, kind }
    }

    /// Get a reference to the event's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a reference to the event's abbreviation.
    ///
    /// Think "VPD" instead of "Varsity Parliamentary Debate;" different tournaments obviously have
    /// different schemes.
    #[must_use]
    pub fn abbr(&self) -> &str {
        &self.abbr
    }

    /// Get a reference to the event's [kind](EventKind).
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
}
