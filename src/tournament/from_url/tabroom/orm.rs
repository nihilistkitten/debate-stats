//! A (very crude) ORM for mapping the tabroom API.

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Tourn {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "TOURNNAME")]
    pub tourn_name: String,
    #[serde(rename = "STARTDATE", with = "tabroom_dates")]
    pub start_date: NaiveDate,
    #[serde(rename = "ENDDATE")]
    pub end_date: String,
    #[serde(rename = "DOWNLOADSITE")]
    pub download_site: String,
}

mod tabroom_dates {
    // https://serde.rs/custom-date-format.html
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%m/%d/%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Event {
    #[serde(rename = "ABBR")]
    pub abbr: String,
    #[serde(rename = "EVENTNAME")]
    pub event_name: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "JUDGE_GROUP")]
    pub judge_group: i32,
    #[serde(rename = "TYPE")]
    pub event_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Entry {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SCHOOL")]
    pub school: i32,
    #[serde(rename = "EVENT")]
    pub event: i32,
    #[serde(rename = "RATING")]
    pub rating: i32,
    #[serde(rename = "CODE")]
    pub code: String,
    #[serde(rename = "FULLNAME")]
    pub full_name: String,
    #[serde(rename = "DROPPED")]
    pub dropped: bool,
    #[serde(rename = "WAITLIST")]
    pub waitlist: bool,
    #[serde(rename = "ADA")]
    pub ada: bool,
    #[serde(rename = "TUBDISABILITY")]
    pub tubdisability: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Judge {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "DOWNLOADRECORD")]
    pub download_record: bool,
    #[serde(rename = "SCHOOL")]
    pub school: i32,
    #[serde(rename = "FIRST")]
    pub first: String,
    #[serde(rename = "LAST")]
    pub last: String,
    #[serde(rename = "PERSON")]
    pub person: i32,
    #[serde(rename = "OBLIGATION")]
    pub obligation: i32,
    #[serde(rename = "HIRED")]
    pub hired: bool,
    #[serde(rename = "TABRATING")]
    pub tab_rating: i32,
    #[serde(rename = "STOPSCHEDULING")]
    pub stop_scheduling: bool,
    #[serde(rename = "ADA")]
    pub ada: bool,
    #[serde(rename = "DIVERSE")]
    pub diverse: bool,
    #[serde(rename = "NOTES")]
    pub notes: bool,
    #[serde(rename = "EMAIL")]
    pub email: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct School {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "DOWNLOADRECORD")]
    pub download_record: bool,
    #[serde(rename = "CODE")]
    pub code: String,
    #[serde(rename = "SCHOOLNAME")]
    pub school_name: String,
    #[serde(rename = "COACHES")]
    pub coaches: i32,
    #[serde(rename = "CHAPTER")]
    pub chapter: bool,
    #[serde(rename = "NSDA")]
    pub nsda: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct Round {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "EVENT")]
    pub event: i32,
    #[serde(rename = "TIMESLOT")]
    pub time_slot: i32,
    #[serde(rename = "TB_SET")]
    pub tiebreak_set: i32,
    #[serde(rename = "RD_NAME")]
    pub rd_name: i32,
    #[serde(rename = "LABEL")]
    pub label: String,
    #[serde(rename = "FLIGHTING")]
    pub flighting: i32,
    #[serde(rename = "JUDGESPERPANEL")]
    pub judges_per_panel: i32,
    #[serde(rename = "JUDGEPLACESCHEME")]
    pub judge_place_scheme: bool,
    #[serde(rename = "PAIRINGSCHEME")]
    pub pairing_scheme: String,
    #[serde(rename = "RUNOFF")]
    pub runoff: bool,
    #[serde(rename = "TOPIC")]
    pub topic: bool,
    #[serde(rename = "CREATEDOFFLINE")]
    pub created_offline: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Panel {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "ROUND")]
    pub round: i32,
    #[serde(rename = "ROOM")]
    pub room: i32,
    #[serde(rename = "FLIGHT")]
    pub flight: i32,
    #[serde(rename = "BYE")]
    pub bye: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Ballot {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "JUDGE")]
    pub judge: i32,
    #[serde(rename = "PANEL")]
    pub panel: i32,
    #[serde(rename = "ENTRY")]
    pub entry: i32,
    #[serde(rename = "SIDE")]
    pub side: i32,
    #[serde(rename = "ROOM")]
    pub room: i32,
    #[serde(rename = "BYE")]
    pub bye: bool,
    #[serde(rename = "NOSHOW")]
    pub no_show: bool,
    #[serde(rename = "CHAIR")]
    pub chair: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct BallotScore {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "BALLOT")]
    pub ballot: i32,
    #[serde(rename = "RECIPIENT")]
    pub recipient: i32,
    #[serde(rename = "SCORE_ID")]
    pub score_id: String,
    #[serde(rename = "SPEECH")]
    pub speech: i32,
    #[serde(rename = "SCORE")]
    pub score: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Tiebreak {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
    #[serde(rename = "DROPS")]
    pub drops: i32,
    #[serde(rename = "FOROPPONENT")]
    pub for_opponent: bool,
    #[serde(rename = "LABEL")]
    pub label: String,
    #[serde(rename = "TAG")]
    pub tag: String,
    #[serde(rename = "TB_SET")]
    pub tb_set: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct TiebreakSet {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SCOREFOR")]
    pub scorefor: String,
    #[serde(rename = "TBSET_NAME")]
    pub tbset_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub(super) struct TimeSlot {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "TIMESLOTNAME")]
    pub time_slot_name: String,
    #[serde(rename = "END")]
    pub end: String,
    #[serde(rename = "START")]
    pub start: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Room {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "BUILDING")]
    pub building: i32,
    #[serde(rename = "ROOMNAME")]
    pub roomname: String,
    #[serde(rename = "QUALITY")]
    pub quality: i32,
    #[serde(rename = "CAPACITY")]
    pub capacity: i32,
    #[serde(rename = "INACTIVE")]
    pub inactive: bool,
    #[serde(rename = "NOTES")]
    pub notes: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct TournamentResults {
    #[serde(rename = "TOURN")]
    pub tourn: Tourn,
    #[serde(rename = "EVENT")]
    pub events: Vec<Event>,
    #[serde(rename = "ENTRY")]
    pub entries: Vec<Entry>,
    #[serde(rename = "JUDGE")]
    pub judges: Vec<Judge>,
    #[serde(rename = "SCHOOL")]
    pub schools: Vec<School>,
    #[serde(rename = "ROUND")]
    pub rounds: Vec<Round>,
    #[serde(rename = "PANEL")]
    pub panels: Vec<Panel>,
    #[serde(rename = "BALLOT")]
    pub ballots: Vec<Ballot>,
    #[serde(rename = "BALLOT_SCORE")]
    pub ballot_scores: Vec<BallotScore>,
    #[serde(rename = "TIEBREAK")]
    pub tiebreaks: Vec<Tiebreak>,
    #[serde(rename = "TIEBREAK_SET")]
    pub tiebreak_sets: Vec<TiebreakSet>,
    #[serde(rename = "TIMESLOT")]
    pub time_slots: Vec<TimeSlot>,
    #[serde(rename = "ROOM")]
    pub rooms: Vec<Room>,
}
