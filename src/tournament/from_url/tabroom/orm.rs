//! A (very crude) ORM for mapping the tabroom API.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Tourn {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "TOURNNAME")]
    pub(super) tourn_name: String,
    #[serde(rename = "STARTDATE")]
    pub(super) start_date: String,
    #[serde(rename = "ENDDATE")]
    pub(super) end_date: String,
    #[serde(rename = "DOWNLOADSITE")]
    pub(super) download_site: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Event {
    #[serde(rename = "ABBR")]
    pub(super) abbr: String,
    #[serde(rename = "EVENTNAME")]
    pub(super) event_name: String,
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "JUDGE_GROUP")]
    pub(super) judge_group: i32,
    #[serde(rename = "TYPE")]
    pub(super) event_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Entry {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "SCHOOL")]
    pub(super) school: i32,
    #[serde(rename = "EVENT")]
    pub(super) event: i32,
    #[serde(rename = "RATING")]
    pub(super) rating: i32,
    #[serde(rename = "CODE")]
    pub(super) code: String,
    #[serde(rename = "FULLNAME")]
    pub(super) full_name: String,
    #[serde(rename = "DROPPED")]
    pub(super) dropped: bool,
    #[serde(rename = "WAITLIST")]
    pub(super) waitlist: bool,
    #[serde(rename = "ADA")]
    pub(super) ada: bool,
    #[serde(rename = "TUBDISABILITY")]
    pub(super) tubdisability: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Judge {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "DOWNLOADRECORD")]
    pub(super) download_record: bool,
    #[serde(rename = "SCHOOL")]
    pub(super) school: i32,
    #[serde(rename = "FIRST")]
    pub(super) first: String,
    #[serde(rename = "LAST")]
    pub(super) last: String,
    #[serde(rename = "PERSON")]
    pub(super) person: i32,
    #[serde(rename = "OBLIGATION")]
    pub(super) obligation: i32,
    #[serde(rename = "HIRED")]
    pub(super) hired: bool,
    #[serde(rename = "TABRATING")]
    pub(super) tab_rating: i32,
    #[serde(rename = "STOPSCHEDULING")]
    pub(super) stop_scheduling: bool,
    #[serde(rename = "ADA")]
    pub(super) ada: bool,
    #[serde(rename = "DIVERSE")]
    pub(super) diverse: bool,
    #[serde(rename = "NOTES")]
    pub(super) notes: bool,
    #[serde(rename = "EMAIL")]
    pub(super) email: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct School {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "DOWNLOADRECORD")]
    pub(super) download_record: bool,
    #[serde(rename = "CODE")]
    pub(super) code: String,
    #[serde(rename = "SCHOOLNAME")]
    pub(super) school_name: String,
    #[serde(rename = "COACHES")]
    pub(super) coaches: i32,
    #[serde(rename = "CHAPTER")]
    pub(super) chapter: bool,
    #[serde(rename = "NSDA")]
    pub(super) nsda: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Round {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "EVENT")]
    pub(super) event: i32,
    #[serde(rename = "TIMESLOT")]
    pub(super) time_slot: i32,
    #[serde(rename = "TB_SET")]
    pub(super) tiebreak_set: i32,
    #[serde(rename = "RD_NAME")]
    pub(super) rd_name: i32,
    #[serde(rename = "LABEL")]
    pub(super) label: String,
    #[serde(rename = "FLIGHTING")]
    pub(super) flighting: bool,
    #[serde(rename = "JUDGESPERPANEL")]
    pub(super) judges_per_panel: i32,
    #[serde(rename = "JUDGEPLACESCHEME")]
    pub(super) judge_place_scheme: bool,
    #[serde(rename = "PAIRINGSCHEME")]
    pub(super) pairing_scheme: String,
    #[serde(rename = "RUNOFF")]
    pub(super) runoff: bool,
    #[serde(rename = "TOPIC")]
    pub(super) topic: bool,
    #[serde(rename = "CREATEDOFFLINE")]
    pub(super) created_offline: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Panel {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "ROUND")]
    pub(super) round: i32,
    #[serde(rename = "ROOM")]
    pub(super) room: i32,
    #[serde(rename = "FLIGHT")]
    pub(super) flight: i32,
    #[serde(rename = "BYE")]
    pub(super) bye: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Ballot {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "JUDGE")]
    pub(super) judge: i32,
    #[serde(rename = "PANEL")]
    pub(super) panel: i32,
    #[serde(rename = "ENTRY")]
    pub(super) entry: i32,
    #[serde(rename = "SIDE")]
    pub(super) side: i32,
    #[serde(rename = "ROOM")]
    pub(super) room: i32,
    #[serde(rename = "BYE")]
    pub(super) bye: bool,
    #[serde(rename = "NOSHOW")]
    pub(super) no_show: bool,
    #[serde(rename = "CHAIR")]
    pub(super) chair: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct BallotScore {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "BALLOT")]
    pub(super) ballot: i32,
    #[serde(rename = "RECIPIENT")]
    pub(super) recipient: i32,
    #[serde(rename = "SCORE_ID")]
    pub(super) score_id: String,
    #[serde(rename = "SPEECH")]
    pub(super) speech: i32,
    #[serde(rename = "SCORE")]
    pub(super) score: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Tiebreak {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "SortOrder")]
    pub(super) sort_order: i32,
    #[serde(rename = "DROPS")]
    pub(super) drops: i32,
    #[serde(rename = "FOROPPONENT")]
    pub(super) for_opponent: bool,
    #[serde(rename = "LABEL")]
    pub(super) label: String,
    #[serde(rename = "TAG")]
    pub(super) tag: String,
    #[serde(rename = "TB_SET")]
    pub(super) tb_set: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct TiebreakSet {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "SCOREFOR")]
    pub(super) scorefor: String,
    #[serde(rename = "TBSET_NAME")]
    pub(super) tbset_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub(super) struct TimeSlot {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "TIMESLOTNAME")]
    pub(super) time_slot_name: String,
    #[serde(rename = "END")]
    pub(super) end: String,
    #[serde(rename = "START")]
    pub(super) start: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Room {
    #[serde(rename = "ID")]
    pub(super) id: i32,
    #[serde(rename = "BUILDING")]
    pub(super) building: i32,
    #[serde(rename = "ROOMNAME")]
    pub(super) roomname: String,
    #[serde(rename = "QUALITY")]
    pub(super) quality: i32,
    #[serde(rename = "CAPACITY")]
    pub(super) capacity: i32,
    #[serde(rename = "INACTIVE")]
    pub(super) inactive: bool,
    #[serde(rename = "NOTES")]
    pub(super) notes: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct TournamentResults {
    #[serde(rename = "TOURN")]
    pub(super) tourn: Tourn,
    #[serde(rename = "EVENT")]
    pub(super) events: Vec<Event>,
    #[serde(rename = "ENTRY")]
    pub(super) entries: Vec<Entry>,
    #[serde(rename = "JUDGE")]
    pub(super) judges: Vec<Judge>,
    #[serde(rename = "SCHOOL")]
    pub(super) schools: Vec<School>,
    #[serde(rename = "ROUND")]
    pub(super) rounds: Vec<Round>,
    #[serde(rename = "PANEL")]
    pub(super) panels: Vec<Panel>,
    #[serde(rename = "BALLOT")]
    pub(super) ballots: Vec<Ballot>,
    #[serde(rename = "BALLOT_SCORE")]
    pub(super) ballot_scores: Vec<BallotScore>,
    #[serde(rename = "TIEBREAK")]
    pub(super) tiebreaks: Vec<Tiebreak>,
    #[serde(rename = "TIEBREAK_SET")]
    pub(super) tiebreak_sets: Vec<TiebreakSet>,
    #[serde(rename = "TIMESLOT")]
    pub(super) time_slots: Vec<TimeSlot>,
    #[serde(rename = "ROOM")]
    pub(super) rooms: Vec<Room>,
}
