use metaflac::Tag;
use crate::common::{Config,ConfigBlock,ConvertError,get_rating_string,TrackData};

const SERATO_PLAYCOUNT_FRAME: &str = "serato_playcount";

fn get_tag_field(field: &str) -> &str {
    match field {
        "Album" => "album",
        "Comment" => "comment",
        "Composer" => "composer",
        "Genre" => "genre",
        "Grouping" => "grouping",
        //"Label" => "publisher", // Do not seem to be available in the files tag, must be in the SDJ database?
        //"Remixer" => "remixer",
        //"Year" => "year",
        &_ => "????"
    };
    "????"
}

fn process_field(config: &Option<ConfigBlock>, data: &str, tag: &mut Tag) -> bool {
    match config {
        Some(block) if block.enabled && !data.is_empty() => {
            match get_tag_field(&block.target) {
                "????" => {}
                target => {
                    tag.set_vorbis(target.to_owned(), vec!(data.to_owned()));
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

fn process_playcount(playcount: u32, tag: &mut Tag) -> bool {
    if playcount > 0 {
        let mut pc = playcount.to_string();
        pc.push('\0');
        tag.set_vorbis(SERATO_PLAYCOUNT_FRAME.to_owned(), vec!(pc));
        return true; 
    }
    false
}

pub fn update_flac_tag(filename: &str, config: &Config, data: TrackData, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = Tag::read_from_path(filename).map_err(|e| ConvertError::TagReadError(Box::new(e)))?;

    let mut changed = Vec::new();
    changed.push(process_playcount(data.playcount, &mut tag));
    changed.push(process_field(&config.rating, &get_rating_string(data.rating), &mut tag));
    changed.push(process_field(&config.user1, &data.user1, &mut tag));
    changed.push(process_field(&config.user2, &data.user2, &mut tag));

    if simulate || !changed.iter().any(|&change| change) {
        return Ok(());
    }

    tag.save().map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    Ok(())
}