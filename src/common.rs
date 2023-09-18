use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct PlayCount {
    pub enabled: bool,
}

#[derive(Debug,Deserialize)]
pub struct ConfigBlock {
    pub enabled: bool,
    pub target: String,
    pub overwrite: bool
}

#[derive(Debug,Deserialize)]
pub struct Config {
    pub playcount: Option<PlayCount>,
    pub rating: Option<ConfigBlock>,
    pub user1: Option<ConfigBlock>,
    pub user2: Option<ConfigBlock>
}

#[derive(Default)]
pub struct TrackData {
    pub playcount: u32,
    pub rating: usize,
    pub user1: String,
    pub user2: String
}

#[derive(Debug)]
pub enum ConvertError {
    UnknownExtension(String),
    TagReadError(Box<dyn std::error::Error + Send + Sync>),
    TagWriteError(Box<dyn std::error::Error + Send + Sync>)
}

pub fn get_rating_string(rating: usize) -> String {
    '★'.to_string().repeat(rating)
}