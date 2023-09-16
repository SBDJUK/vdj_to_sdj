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
use crate::common::ConvertError;

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

fn update_tag(filename: &str, playcount: u32, rating: usize, hashtag: &str, simulate: bool) -> Result<(), ConvertError> {
    let ext_str = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or_else(|| ConvertError::UnknownExtension("Unable to get extension".to_string()))?;

    match &*ext_str {
        "mp3" => tag_id3::update_id3_tag(filename, tag_id3::FileType::MP3, playcount, rating, hashtag, simulate),
        "wav" => tag_id3::update_id3_tag(filename, tag_id3::FileType::WAV, playcount, rating, hashtag, simulate),
        "aif" | "aiff" | "aifc" => tag_id3::update_id3_tag(filename, tag_id3::FileType::AIFF, playcount, rating, hashtag, simulate),
        "m4a" | "mp4" | "aac" => tag_mp4::update_mp4_tag(filename, playcount, rating, hashtag, simulate),
        "flac" => tag_flac::update_flac_tag(filename, playcount, rating, hashtag, simulate),
        "ogg" => tag_ogg::update_ogg_tag(filename, playcount, rating, hashtag, simulate),
        _ => Err(ConvertError::UnknownExtension(format!("Unsupported file extension: {}", ext_str))),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    info!("Reading database");
    let xml_content = std::fs::read_to_string(args.database).expect("Could not read database file");
    info!("Parsing database");
    let database: VirtualDJDatabase = from_str(&xml_content)?;
    info!("Database read completed");
    for song in database.songs {
        let playcount = song.infos.as_ref().and_then(|i| i.play_count).unwrap_or(0);
        let rating = song.tags.as_ref().and_then(|t| t.stars).unwrap_or(0);
        let rating_usize: usize = std::cmp::min(5, std::cmp::max(0, rating)).try_into().unwrap();
        let hashtag = song.tags.as_ref().and_then(|t| t.user1.clone()).unwrap_or_else(|| "".to_string());                     
        if rating_usize > 0 || playcount > 0 || hashtag !="" {
            info!("File Path: {}  =>  PlayCount: {}  Stars: {}  Hashtag: {}", song.file_path, playcount, rating_usize, &hashtag);
            if let Err(err) = update_tag(&song.file_path, playcount, rating_usize, &hashtag, args.simulate) {
                error!("Failed to update tag: {:?}", err);
            }
        }
    }
    Ok(())   
}
