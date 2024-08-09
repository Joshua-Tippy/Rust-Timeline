#[derive(serde::Deserialize, Debug, Clone)]
pub struct HistoricalTimeline {
    pub HistoricalTimeline: TimelineData,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TimelineData {
    pub title: String,
    pub description: String,
    pub events: Vec<Event>,
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Event{
    pub event_id: i32,
    pub title: String,
    pub date_int: Option<i32>,
    pub date: String,
    #[serde(deserialize_with = "deserialize_date_precision")]
    pub date_precision: DateFormat,
    pub tags: Vec<String>,
    pub description: String,
    pub scripture_references: Vec<String>,
}


impl HistoricalTimeline {
    fn finalize(&mut self) {
        for event in self.HistoricalTimeline.events.iter_mut() {
            event.date_precision.finalize(event.date.clone());
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]

pub enum DateFormat {
    //#[serde(rename = "YYYY")]
    YYYY(String),                    // YYYY
    MM_YYYY(String),                 // MM-YYYY
    DD_MM_YYYY(String),              // DD-MM-YYYY
    DD_MM_YYYYzHH_MM_SS(String),     // DD-MM-YYYYzHH-MM-SS
}

pub enum Direction {
    Forward, Reverse
}

//use std::error::Error;
impl DateFormat {
    pub fn new(date: String) -> Result<DateFormat, FormatError > {
        match date.len() {
            1..=4 => Ok(DateFormat::YYYY(date)),
            6..=7 => Ok(DateFormat::MM_YYYY(date)),
            9..=10 => Ok(DateFormat::DD_MM_YYYY(date)),
            11..=19 => Ok(DateFormat::DD_MM_YYYYzHH_MM_SS(date)),
            _ => Err(FormatError::InvalidFormat("Invalid Date Format: ".to_string() + &date)),
        }
        //DateFormat::YYYY(format)
    }
    pub fn finalize(&mut self, date: String) -> Result<DateFormat, FormatError > {
        match self {
            DateFormat::YYYY(s) => Ok(DateFormat::YYYY(date)),
            DateFormat::MM_YYYY(s) => Ok(DateFormat::MM_YYYY(date)),
            DateFormat::DD_MM_YYYY(s) => Ok(DateFormat::DD_MM_YYYY(date)),
            DateFormat::DD_MM_YYYYzHH_MM_SS(s) => Ok(DateFormat::DD_MM_YYYYzHH_MM_SS(date)),
            _ => Err(FormatError::InvalidFormat("Invalid Date Format: ".to_string() + &date)),
        }
        //DateFormat::YYYY(format)
    }
}


use serde::{Deserialize, Deserializer};
use serde_yaml;
fn deserialize_date_precision<'de, D>(deserializer: D) -> Result<DateFormat, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "YYYY" => Ok(DateFormat::YYYY(s)),
        "MM-YYYY" => Ok(DateFormat::MM_YYYY(s)),
        "DD-MM-YYYY" => Ok(DateFormat::DD_MM_YYYY(s)),
        "DD-MM-YYYYzHH-MM-SS" => Ok(DateFormat::DD_MM_YYYYzHH_MM_SS(s)),
        _ => Err(serde::de::Error::unknown_variant(&s, &["YYYY", "MM-YYYY", "DD-MM-YYYY", "DD-MM-YYYYzHH-MM-SS"])),
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(parse(try_from_str = parse_date_range))]
    start_date_end_date: (DateFormat, DateFormat),

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "t", long = "tags", default_value = "*")]
    tags: Vec<String>,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    direction: bool,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}


// Custom parsing function for the date_range
fn parse_date_range(s: &str) -> Result<(DateFormat, DateFormat), &'static str> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Please provide start and end date formats separated by a comma.");
    }

    let start_date = DateFormat::new(parts[0].to_string());
    let end_date = DateFormat::new(parts[1].to_string());
    Ok((start_date.unwrap(), end_date.unwrap()))
}



use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid date format provided: {0}")]
    InvalidFormat(String),
}
