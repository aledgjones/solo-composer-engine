pub mod barline;
pub mod time_signature;

use barline::Barline;
use time_signature::TimeSignature;

#[derive(Debug, Serialize)]
pub enum EntryContent {
    Barline(Barline),
    TimeSignature(TimeSignature),
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub tick: u32,
    pub key: String,
    pub content: EntryContent,
}
