mod common;
mod tag_id3;
mod tag_mp4;
mod tag_flac;
mod tag_ogg;

extern crate serde;
extern crate serde_xml_rs;
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::path::Path;
use log::{info,error};
use clap::Parser;
use crate::common::{Config,ConvertError,TrackData};
use std::fs::File;
use std::io::Read;
use crate::serde::de::Error;

#[derive(Debug, Deserialize)]
struct Song {
    #[serde(rename = "FilePath")]
    file_path: String,
    #[serde(rename = "Tags")]
    tags: Option<Tags>,
    #[serde(rename = "Infos")]
    infos: Option<Infos>,
}

#[derive(Debug, Deserialize)]
struct Tags {
    #[serde(rename = "Stars")]
    stars: Option<u32>,
    #[serde(rename = "User1")]
    user1: Option<String>,
    #[serde(rename = "User2")]
    user2: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Infos {
    #[serde(rename = "PlayCount")]
    play_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct VirtualDJDatabase {
    #[serde(rename = "Song")]
    songs: Vec<Song>,
}

#[derive(Parser, Debug)]
#[command(
    author = "SBDJ",
    version = "0.1",
    about = "Convert some fields from VirtualDJ database XML into Serato compatible metadata tags",
    long_about = None)]
struct Args {
    #[arg(short = 'd', long = "database")]
    database: String,

    #[arg(short = 's', long = "simulate")]
    simulate: bool,
}

fn update_tag(filename: &str, config: &Config, data: TrackData, simulate: bool) -> Result<(), ConvertError> {
    let ext_str = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or_else(|| ConvertError::UnknownExtension("Unable to get extension".to_string()))?;

    match &*ext_str {
        "mp3" => tag_id3::update_id3_tag(filename, tag_id3::FileType::MP3, config, data, simulate),
        "wav" => tag_id3::update_id3_tag(filename, tag_id3::FileType::WAV, config, data, simulate),
        "aif" | "aiff" | "aifc" => tag_id3::update_id3_tag(filename, tag_id3::FileType::AIFF, config, data, simulate),
        "m4a" | "mp4" | "aac" => tag_mp4::update_mp4_tag(filename, config, data, simulate),
        "flac" => tag_flac::update_flac_tag(filename, config, data, simulate),
        "ogg" => tag_ogg::update_ogg_tag(filename, config, data, simulate),
        _ => Err(ConvertError::UnknownExtension(format!("Unsupported file extension: {}", ext_str))),
    }
}

fn read_config(filename: &str) -> Result<Config, toml::de::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(toml::de::Error::custom(format!("Failed to open file: {}", e))),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => return Err(toml::de::Error::custom(format!("Failed to read file: {}", e))),
    };

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let config = read_config("config.toml").unwrap();

    info!("Reading database");
    let xml_content = std::fs::read_to_string(args.database).expect("Could not read database file");
    info!("Parsing database");
    let database: VirtualDJDatabase = from_str(&xml_content)?;
    info!("Database read completed");
    for song in database.songs {
        let mut data = TrackData::default();
        data.playcount = song.infos.as_ref().and_then(|i| i.play_count).unwrap_or(0);
        let rating = song.tags.as_ref().and_then(|t| t.stars).unwrap_or(0);
        data.rating = std::cmp::min(5, std::cmp::max(0, rating)).try_into().unwrap();
        data.user1 = song.tags.as_ref().and_then(|t| t.user1.clone()).unwrap_or_else(|| "".to_string());                     
        data.user2 = song.tags.as_ref().and_then(|t| t.user2.clone()).unwrap_or_else(|| "".to_string());                     
        if data.rating > 0 || data.playcount > 0 || data.user1 !="" || data.user2 !="" {
            info!("File Path: {}  =>  PlayCount: {}  Rating: {}  User1: {}  User2: {}", song.file_path, data.playcount, data.rating, data.user1, data.user2);
            if let Err(err) = update_tag(&song.file_path, &config, data, args.simulate) {
                error!("Failed to update tag: {:?}", err);
            }
        }
    }
    Ok(())   
}
