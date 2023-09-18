use id3::{Tag, TagLike, Version, frame};
use crate::common::{Config,ConvertError,ConfigBlock,get_rating_string,TrackData};

#[derive(Debug, PartialEq)]
pub enum FileType {
    MP3,
    AIFF,
    WAV,
}

const SERATO_PLAYCOUNT_FRAME: &str = "SERATO_PLAYCOUNT";

fn get_tag_field(field: &str) -> &str {
    match field {
        "Album" => "TALB",
        "Comment" => "COMM",
        "Composer" => "TCOM",
        "Genre" => "TCON",
        "Grouping" => "TIT1",
        "Label" => "TPUB",
        "Remixer" => "TPE4",
        "Year" => "TDRC",
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
                    tag.set_text_values(target.to_owned(), [data.to_owned()]);
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

fn read_tag_from_file(filename: &str, id3_type: &FileType) -> Result<Tag, ConvertError> {
    let tag = match id3_type {
        FileType::MP3 => Tag::read_from_path(filename),
        FileType::AIFF => Tag::read_from_aiff_path(filename),
        FileType::WAV => Tag::read_from_wav_path(filename),
    };

    match tag {
        Ok(tag) => Ok(tag),
        Err(id3::Error { kind: id3::ErrorKind::NoTag, .. }) => Ok(id3::Tag::new()),
        Err(e) => Err(ConvertError::TagReadError(Box::new(e))),
    }
}

fn process_playcount(playcount: u32, tag: &mut Tag) -> bool {
    if playcount > 0 {
        tag.remove_extended_text(Some(SERATO_PLAYCOUNT_FRAME), None);
        tag.add_frame(frame::ExtendedText{
            description: SERATO_PLAYCOUNT_FRAME.to_owned(),
            value: format!("{}\0", playcount)
        });
        return true; 
    }
    false
}

pub fn update_id3_tag(filename: &str, id3_type: FileType, config: &Config, data: TrackData, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = read_tag_from_file(filename, &id3_type)?;

    let mut changed = Vec::new();
    changed.push(process_playcount(data.playcount, &mut tag));
    changed.push(process_field(&config.rating, &get_rating_string(data.rating), &mut tag));
    changed.push(process_field(&config.user1, &data.user1, &mut tag));
    changed.push(process_field(&config.user2, &data.user2, &mut tag));

    if simulate || !changed.iter().any(|&change| change) {
        return Ok(());
    }
    
    if id3_type == FileType::MP3 {
        tag.write_to_path(filename, Version::Id3v24).map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    } else if id3_type == FileType::AIFF {
        tag.write_to_aiff_path(filename, Version::Id3v24).map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    } else if id3_type == FileType::WAV {
        tag.write_to_wav_path(filename, Version::Id3v24).map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    }
    Ok(())
}