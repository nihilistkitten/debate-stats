//! Utilities for tests.
#![allow(clippy::module_name_repetitions)]

use std::fmt::Display;

use chrono::NaiveDate;

use crate::{Entry, Event, EventKind, Tournament};

pub struct TestFailure(String);

impl From<String> for TestFailure {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type TestResult = Result<(), TestFailure>;

pub trait Input<C: Default, O> {
    /// the function under test
    fn under_test(self, config: C) -> O;
}

impl<T> Input<(), Self> for T {
    fn under_test(self, _: ()) -> Self {
        self
    }
}

pub trait ExpectedOutput<T>: Sized {
    /// check if the expected output is correct
    fn run(self, output: T) -> TestResult;

    fn input<C: Default, I: Input<C, T>>(self, input: I) -> TestCase<T, C, I, Self> {
        TestCase {
            input,
            output: self,
            config: C::default(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

pub struct TestCase<T, C: Default, I: Input<C, T>, O: ExpectedOutput<T>> {
    input: I,
    config: C,
    output: O,
    phantom_data: std::marker::PhantomData<T>, // we need this to not get a compiler error
}

impl<T, C: Default, I: Input<C, T>, O: ExpectedOutput<T>> TestCase<T, C, I, O> {
    /// configure the test
    #[allow(dead_code)]
    pub fn config(self, config: C) -> Self {
        Self { config, ..self }
    }

    /// run the test, returning err on failure
    pub fn check(self) -> TestResult {
        self.output.run(self.input.under_test(self.config))
    }

    /// run the test, panicing on failure
    pub fn run(self) {
        self.check().unwrap_or_else(|e| panic!("{}", e))
    }
}

/// return `Ok` if left == right, otherwise a return a nicely formatted `TestError`
/// utility for impling `ExpectedOutput`
fn matching<T: Display>(
    expected: &(impl PartialEq<T> + Display),
    observed: &T,
    name: &str,
) -> TestResult {
    if expected == observed {
        Ok(())
    } else {
        Err(format!(
            "\n\n{}s don't match:\n\texpected: {}\n\tobserved: {}\n\n",
            name, expected, observed
        )
        .into())
    }
}

pub struct EntryTest {
    code: String,
    full_name: String,
}

impl ExpectedOutput<&Entry> for EntryTest {
    // this neesd to run on &entry because we only have references when we do the test
    fn run(self, output: &Entry) -> TestResult {
        matching(&self.full_name, &output.full_name(), "entry full name")?;
        matching(&self.code, &output.code(), "event code")?;
        Ok(())
    }
}

impl EntryTest {
    fn new(full_name: &str, code: &str) -> Self {
        Self {
            full_name: full_name.into(),
            code: code.into(),
        }
    }
}

pub struct EventTest {
    name: String,
    abbr: String,
    kind: EventKind,
    entries: Vec<EntryTest>,
}

impl ExpectedOutput<&Event> for EventTest {
    // this neesd to run on &event because we only have references when we do the test
    fn run(self, output: &Event) -> TestResult {
        matching(&self.name, &output.name(), "event name")?;
        matching(&self.abbr, &output.abbr(), "event abbreviation")?;
        matching(&self.kind, output.kind(), "event kind")?;

        for entry in self.entries {
            // first find an event with the same name to test against
            // this is faster and more readable
            let entry_under_test = output
                .entries()
                .iter()
                .find(|&e| e.code() == entry.code)
                .ok_or_else(|| format!("No entry matching {}", entry.code))?;

            // then do the test
            entry.input(entry_under_test).check()?;
        }
        Ok(())
    }
}

impl EventTest {
    fn new(name: &str, abbr: &str, kind: EventKind) -> Self {
        Self {
            name: name.into(),
            abbr: abbr.into(),
            kind,
            entries: Vec::default(),
        }
    }

    // here is a regex to generate entries from tabroom's provided CSV of entries
    // %s/[^,]*,[^,]*,\(.*\),\(.*\)/.entry(EntryTest::new("\1", "\2"))
    fn entry(mut self, entry: EntryTest) -> Self {
        self.entries.push(entry);
        self
    }
}

pub struct TournamentTest {
    name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    events: Vec<EventTest>,
}

impl ExpectedOutput<Tournament> for TournamentTest {
    fn run(self, output: Tournament) -> TestResult {
        matching(&self.name, &output.name(), "tournament name")?;
        matching(
            &self.start_date,
            output.start_date(),
            "tournament start date",
        )?;
        matching(&self.end_date, output.end_date(), "tournament end date")?;

        // for each event, find a matching expected event
        for event in self.events {
            // first find an event with the same name to test against
            // this is faster and more readable
            let event_under_test = output
                .events()
                .iter()
                .find(|&e| e.name() == event.name)
                .ok_or_else(|| format!("No event matching {}", event.name))?;

            // then do the test
            event.input(event_under_test).check()?;
        }

        Ok(())
    }
}

impl TournamentTest {
    fn new(
        name: &str,
        start_year: i32,
        start_month: u32,
        start_day: u32,
        end_year: i32,
        end_month: u32,
        end_day: u32,
    ) -> Self {
        Self {
            name: name.into(),
            start_date: NaiveDate::from_ymd(start_year, start_month, start_day),
            end_date: NaiveDate::from_ymd(end_year, end_month, end_day),
            events: Vec::default(),
        }
    }

    fn event(mut self, event: EventTest) -> Self {
        self.events.push(event);
        self
    }

    /// Generate the NPDI test case.
    #[allow(clippy::clippy::too_many_lines)] // this is just setting up the test case
    pub fn npdi() -> Self {
        Self::new(
            "National Parliamentary Debate Invitational",
            2020,
            11,
            14,
            2020,
            11,
            16,
        )
        .event(
            EventTest::new("Open Parli", "Open", EventKind::Debate)
                .entry(EntryTest::new("Harding & O'Rafferty", "Bishop O'Dowd HO"))
                .entry(EntryTest::new(
                    "Hollinger-Miles & Wolmark",
                    "Bishop O'Dowd HW",
                ))
                .entry(EntryTest::new("Jayasuriya & Barretto", "Bishop O'Dowd JB"))
                .entry(EntryTest::new(
                    "Kuiper Rauch & Elenteny",
                    "Bishop O'Dowd KE",
                ))
                .entry(EntryTest::new("Sundar & Richards", "Bishop O'Dowd SR"))
                .entry(EntryTest::new("Mao & spalding", "Campolindo MS"))
                .entry(EntryTest::new("Perry & Phadnis", "Campolindo PP"))
                .entry(EntryTest::new("Ravikumar & Kenderski", "Campolindo RK"))
                .entry(EntryTest::new("Ryan & Young", "Campolindo RY"))
                .entry(EntryTest::new("Tolajian & Crosby", "Campolindo TC"))
                .entry(EntryTest::new("Robertson & Toh", "Catlin Gabel RT"))
                .entry(EntryTest::new("Chen & Hao", "Crystal Springs Uplands CH"))
                .entry(EntryTest::new("Mills & Gao", "Crystal Springs Uplands MG"))
                .entry(EntryTest::new("Ma & Qin", "Crystal Springs Uplands MQ"))
                .entry(EntryTest::new("Karkhanis & Singh", "Cupertino KS"))
                .entry(EntryTest::new("Mahajan & Kerai", "Cupertino MK"))
                .entry(EntryTest::new("Natarajan & Yee", "Cupertino NY"))
                .entry(EntryTest::new("Chu & Mui", "Dougherty Valley CM"))
                .entry(EntryTest::new("Chakkenchath & Yang", "Dougherty Valley CY"))
                .entry(EntryTest::new("Mangalick & Sriram", "Dougherty Valley MS"))
                .entry(EntryTest::new("Zhang & Ahuja", "Dougherty Valley ZA"))
                .entry(EntryTest::new("Gund-Morrow & Bloch", "Friends Seminary GB"))
                .entry(EntryTest::new("Batriedo & Parikh", "Granite Bay BP"))
                .entry(EntryTest::new("Elahi & Krishnan", "Granite Bay EK"))
                .entry(EntryTest::new("Nagunuri & Sawyer", "Granite Bay NS"))
                .entry(EntryTest::new("Shelat & Kassouni", "Granite Bay SK"))
                .entry(EntryTest::new("Chung & Mejia", "Gunn CM"))
                .entry(EntryTest::new("Li & Agarwal", "Gunn LA"))
                .entry(EntryTest::new("Garg & Shah", "Irvington GS"))
                .entry(EntryTest::new("Kharbanda & Song", "Irvington KS"))
                .entry(EntryTest::new("Mandal & Telkar", "Irvington MT"))
                .entry(EntryTest::new("Parekh & Mehta", "Irvington PM"))
                .entry(EntryTest::new("Shyam Sundar & Zhang", "Irvington SZ"))
                .entry(EntryTest::new("Zhu & Pandey", "Irvington ZP"))
                .entry(EntryTest::new("Modi & Jonnala", "Irvington/JFK-Fremont MJ"))
                .entry(EntryTest::new("Chatwin & Ikegami", "Los Altos CI"))
                .entry(EntryTest::new("Kadiyala & Lingo", "Los Altos KL"))
                .entry(EntryTest::new("Mehta & Colgrove", "Los Altos MC"))
                .entry(EntryTest::new("Shi & Lue", "Los Altos SL"))
                .entry(EntryTest::new("Su & Lay", "Los Altos SuLa"))
                .entry(EntryTest::new(
                    "Tierling & Wong",
                    "Los Altos/Mountain View TW",
                ))
                .entry(EntryTest::new("Bae & Patnaik", "Menlo-Atherton BP"))
                .entry(EntryTest::new("Doran & Chen", "Menlo-Atherton DC"))
                .entry(EntryTest::new("Gupta & Deutch", "Menlo-Atherton GD"))
                .entry(EntryTest::new("Grosso & Goldberg", "Menlo-Atherton GG"))
                .entry(EntryTest::new("Nori & Bodnick", "Menlo-Atherton NB"))
                .entry(EntryTest::new("Parikh-Briggs & Goel", "Menlo-Atherton PG"))
                .entry(EntryTest::new("Liou & Yung", "Mountain View/Los Altos LY"))
                .entry(EntryTest::new("Haimes & Carlin", "New Roads HC"))
                .entry(EntryTest::new("Robinson-Rosendorff & Sims", "New Roads RS"))
                .entry(EntryTest::new("Tran & Tong", "Notre Dame San Jose TT"))
                .entry(EntryTest::new("Lin & Taware", "Papaya Valley LT"))
                .entry(EntryTest::new("Mani & Sharma", "Papaya Valley MS"))
                .entry(EntryTest::new("Shukla & Kudva", "Papaya Valley SK"))
                .entry(EntryTest::new("Thalamati & Prabhune", "Papaya Valley TP"))
                .entry(EntryTest::new(
                    "Choudhury & Manthapuri",
                    "Papaya Valley/Irvington CM",
                ))
                .entry(EntryTest::new("Cheney & Garcia", "Piedmont CG"))
                .entry(EntryTest::new("Del Real & Uriostegui", "Riverside STEM DU"))
                .entry(EntryTest::new("Keshavan & Keshavan", "Riverside STEM KK"))
                .entry(EntryTest::new("Liu & Bensaid", "Riverside STEM LB"))
                .entry(EntryTest::new("Perlstein & Reddy", "Riverside STEM PR"))
                .entry(EntryTest::new("Mogharei & Lewis", "San Ramon Valley ML"))
                .entry(EntryTest::new("Kaushek & Kiang", "Menlo KK"))
                .entry(EntryTest::new("Mishkin & Yoo", "Menlo MY"))
                .entry(EntryTest::new("Nandal & Mangtani", "Menlo NM"))
                .entry(EntryTest::new("Ball & Zhu", "Nueva BZ"))
                .entry(EntryTest::new("Cham & Lehane", "Nueva CL"))
                .entry(EntryTest::new("Descollonges & Xu", "Nueva DX"))
                .entry(EntryTest::new("Jonker & Chatterjee", "Nueva JoCh"))
                .entry(EntryTest::new("Jonker & Sachdev", "Nueva JS"))
                .entry(EntryTest::new("Kuznetsov & Srihari", "Nueva KS"))
                .entry(EntryTest::new("Mathewson & Peasley-Lynch", "Nueva MP"))
                .entry(EntryTest::new("Stoffel & Nickolov", "Nueva SN"))
                .entry(EntryTest::new("Turner & Kan", "Nueva TK"))
                .entry(EntryTest::new("Turner & Sharma", "Nueva TS"))
                .entry(EntryTest::new("Wee & Byun", "Nueva WB"))
                .entry(EntryTest::new("Andrijanic & Suh", "Torrey Pines AS"))
                .entry(EntryTest::new("Bloom & Tisdale", "Valencia BT"))
                .entry(EntryTest::new("Jain & Diop", "Washington JD"))
                .entry(EntryTest::new("Morales & Luo", "Washington ML"))
                .entry(EntryTest::new("Nair & Panda", "Washington NP"))
                .entry(EntryTest::new("Pandhare & Ip", "Washington PI"))
                .entry(EntryTest::new("Sankar & Devadhar", "Washington SaDe"))
                .entry(EntryTest::new("Shah & Shiva", "Washington SS")),
        )
        .event(EventTest::new("JV Parli", "JV", EventKind::Debate))
    }

    /// Generate the Jack Howe test case.
    #[allow(clippy::too_many_lines)] // this is all just writing the test cases
    pub fn jack_howe() -> Self {
        Self::new("Jack Howe Memorial Tournament", 2020, 9, 19, 2020, 9, 22)
            .event(EventTest::new("JV Policy", "JVCX", EventKind::Debate))
            .event(EventTest::new(
                "Novice Dramatic Interpretation",
                "N DI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Extemporaneous",
                "N Ext",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Humorous Interpretation",
                "N HI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Impromptu",
                "N Imp",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Informative",
                "N Inf",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Lincoln-Douglas",
                "N LD",
                EventKind::Debate,
            ))
            .event(EventTest::new(
                "Novice Oratorical Interpretation",
                "N OI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Original Oratory",
                "N OO",
                EventKind::Speech,
            ))
            .event(EventTest::new("Novice Policy", "N CX", EventKind::Debate))
            .event(EventTest::new(
                "Novice Program Oral Interpretation",
                "N POI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Novice Public Forum",
                "N PF",
                EventKind::Debate,
            ))
            .event(EventTest::new(
                "Open Congress",
                "O Con",
                EventKind::Other("congress".into()),
            ))
            .event(EventTest::new(
                "Open Dramatic Interpretation",
                "O DI",
                EventKind::Speech,
            ))
            .event(
                EventTest::new("Open Extemporaneous", "O Ext", EventKind::Speech)
                    .entry(EntryTest::new("Grant Oshita", "HN Grant Oshita"))
                    .entry(EntryTest::new("Ishaq Khan", "HN Ishaq Khan"))
                    .entry(EntryTest::new("Kevin Velasquez", "HN Kevin Velasquez"))
                    .entry(EntryTest::new("Serena Lin", "HN Serena Lin"))
                    .entry(EntryTest::new("Elaine Ma", "HJ Elaine Ma"))
                    .entry(EntryTest::new("Katelyn Cai", "EN Katelyn Cai"))
                    .entry(EntryTest::new(
                        "Pranav Tangallpalli",
                        "EN Pranav Tangallpalli",
                    ))
                    .entry(EntryTest::new("Lauren Adams", "IK Lauren Adams"))
                    .entry(EntryTest::new("Arshon Keyani", "GD Arshon Keyani"))
                    .entry(EntryTest::new("Ashok Ramkumar", "GD Ashok Ramkumar"))
                    .entry(EntryTest::new("Joseph Thomas", "GD Joseph Thomas"))
                    .entry(EntryTest::new("Kush Narang", "GD Kush Narang"))
                    .entry(EntryTest::new("Majid Shabbeer", "GD Majid Shabbeer"))
                    .entry(EntryTest::new("Nihaar Charagulla", "GD Nihaar Charagulla"))
                    .entry(EntryTest::new("Oliver Owen", "GD Oliver Owen"))
                    .entry(EntryTest::new("Owen Thompson", "GD Owen Thompson"))
                    .entry(EntryTest::new("Ryan Alappatt", "GD Ryan Alappatt"))
                    .entry(EntryTest::new("Tyler Pineda", "GD Tyler Pineda"))
                    .entry(EntryTest::new("Justin Marimon", "AE Justin Marimon"))
                    .entry(EntryTest::new("Theodore Gercken", "AW Theodore Gercken"))
                    .entry(EntryTest::new("Dominic Erickson", "IP Dominic Erickson"))
                    .entry(EntryTest::new("Maneh Davityan", "KB Maneh Davityan"))
                    .entry(EntryTest::new("Tran Pham", "IE Tran Pham"))
                    .entry(EntryTest::new("Arnav Soni", "DG Arnav Soni"))
                    .entry(EntryTest::new("Gautham Sudhakar", "DG Gautham Sudhakar"))
                    .entry(EntryTest::new("Krishiv Haranath", "DG Krishiv Haranath"))
                    .entry(EntryTest::new("Risha Chakraborty", "DG Risha Chakraborty"))
                    .entry(EntryTest::new("Sanjana Kollu", "DG Sanjana Kollu"))
                    .entry(EntryTest::new("Tanay Subramanian", "DG Tanay Subramanian"))
                    .entry(EntryTest::new("Austin Hong", "DD Austin Hong"))
                    .entry(EntryTest::new("Daniel Zhao", "DD Daniel Zhao"))
                    .entry(EntryTest::new("Lauren Kim", "DD Lauren Kim"))
                    .entry(EntryTest::new("Astor Redhead", "KT Astor Redhead"))
                    .entry(EntryTest::new("Garrett Fan", "KT Garrett Fan"))
                    .entry(EntryTest::new("Daniel Candia", "LN Daniel Candia"))
                    .entry(EntryTest::new(
                        "Gabriel Frank-McPheter",
                        "LN Gabriel Frank-McPheter",
                    ))
                    .entry(EntryTest::new(
                        "Gabriel Sundaramoorthy",
                        "LN Gabriel Sundaramoorthy",
                    ))
                    .entry(EntryTest::new("Luccia Yacoub", "LN Luccia Yacoub"))
                    .entry(EntryTest::new("Govind Pattathil", "BK Govind Pattathil"))
                    .entry(EntryTest::new(
                        "Venkata Harshitha Pulagam",
                        "BK Venkata Harshitha Pulagam",
                    ))
                    .entry(EntryTest::new("Andrew Lai", "CC Andrew Lai"))
                    .entry(EntryTest::new("Logan Tang", "CC Logan Tang"))
                    .entry(EntryTest::new("Gonzalo Vargas", "GE Gonzalo Vargas"))
                    .entry(EntryTest::new("Florence Zhu", "BC Florence Zhu"))
                    .entry(EntryTest::new("Emma McGregor", "LH Emma McGregor"))
                    .entry(EntryTest::new("Zachary English", "LH Zachary English"))
                    .entry(EntryTest::new("Alex Zhang", "HK Alex Zhang"))
                    .entry(EntryTest::new("Adam Grau", "GF Adam Grau"))
                    .entry(EntryTest::new("Alexander Gerson", "GF Alexander Gerson"))
                    .entry(EntryTest::new("Brian Francisco", "GF Brian Francisco"))
                    .entry(EntryTest::new("Jayden Sengul", "GF Jayden Sengul"))
                    .entry(EntryTest::new("Nathan Wilson", "GF Nathan Wilson"))
                    .entry(EntryTest::new("Nicholas Monsour", "GF Nicholas Monsour"))
                    .entry(EntryTest::new("Robert Safar", "GF Robert Safar"))
                    .entry(EntryTest::new("Vahan Haytayan", "GF Vahan Haytayan"))
                    .entry(EntryTest::new("Giuseppe DiMassa", "CB Giuseppe DiMassa"))
                    .entry(EntryTest::new("Simren Parikh", "CR Simren Parikh"))
                    .entry(EntryTest::new("Arni Kulkarni", "MC Arni Kulkarni"))
                    .entry(EntryTest::new("Moniva Pal", "MC Moniva Pal"))
                    .entry(EntryTest::new("Andrew Schmitz", "FS Andrew Schmitz"))
                    .entry(EntryTest::new("Katie Lee", "FS Katie Lee"))
                    .entry(EntryTest::new("Aaron Ong", "BW Aaron Ong"))
                    .entry(EntryTest::new(
                        "Ethan Yi-Fang Wang",
                        "BW Ethan Yi-Fang Wang",
                    ))
                    .entry(EntryTest::new("Joshua Yue", "BW Joshua Yue"))
                    .entry(EntryTest::new("Lucas Chen", "BW Lucas Chen"))
                    .entry(EntryTest::new("Rahil Siddiqi", "BW Rahil Siddiqi"))
                    .entry(EntryTest::new("Chloe Affaki", "FG Chloe Affaki"))
                    .entry(EntryTest::new(
                        "Harshita Krupadanam",
                        "EZ Harshita Krupadanam",
                    ))
                    .entry(EntryTest::new(
                        "Timothy Schoonover",
                        "EZ Timothy Schoonover",
                    ))
                    .entry(EntryTest::new("River Simard", "OZ River Simard"))
                    .entry(EntryTest::new("Adelaide Parker", "DP Adelaide Parker"))
                    .entry(EntryTest::new("Alicia Du", "DP Alicia Du"))
                    .entry(EntryTest::new("Cara Wilson", "DC Cara Wilson"))
                    .entry(EntryTest::new("Keith Maben", "IJ Keith Maben"))
                    .entry(EntryTest::new(
                        "Christopher Baldwin",
                        "BM Christopher Baldwin",
                    ))
                    .entry(EntryTest::new("Eva Bonanno", "BM Eva Bonanno"))
                    .entry(EntryTest::new("Giuliana Tepedino", "BM Giuliana Tepedino")),
            )
            .event(EventTest::new(
                "Open Humorous Interpretation",
                "O HI",
                EventKind::Speech,
            ))
            .event(EventTest::new("Open Impromptu", "O Imp", EventKind::Speech))
            .event(EventTest::new(
                "Open Informative",
                "O Inf",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Open Lincoln-Douglas - CA",
                "CA LD",
                EventKind::Debate,
            ))
            .event(EventTest::new(
                "Open Lincoln-Douglas - TOC",
                "TOCLD",
                EventKind::Debate,
            ))
            .event(EventTest::new(
                "Open Oratorical Interpretation",
                "O OI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Open Original Oratory",
                "O OO",
                EventKind::Speech,
            ))
            .event(EventTest::new("Open Policy", "O CX", EventKind::Debate))
            .event(EventTest::new(
                "Open Program Oral Interpretation",
                "O POI",
                EventKind::Speech,
            ))
            .event(EventTest::new(
                "Open Public Forum",
                "O PF",
                EventKind::Debate,
            ))
            .event(EventTest::new("Parliamentary", "Parli", EventKind::Debate))
            .event(EventTest::new(
                "World School Debate",
                "WSD",
                EventKind::Other("wsdc".into()),
            ))
    }
}

// TournamentTest::npdi().build(input).run()
