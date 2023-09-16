use id3::{TagLike, Version, frame};
use crate::common::{ConvertError,get_rating_string};

#[derive(Debug, PartialEq)]
pub enum FileType {
    MP3,
    AIFF,
    WAV,
}

pub fn update_id3_tag(filename: &str, id3_type: FileType, playcount: u32, rating: usize, hashtag: &str, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = if id3_type == FileType::MP3 {
        match id3::Tag::read_from_path(filename) {
            Ok(tag) => tag,
            Err(id3::Error{kind: id3::ErrorKind::NoTag, ..}) => id3::Tag::new(),
            Err(e) => {
                return Err(ConvertError::TagReadError(Box::new(e)));
            }
        }
    } else if id3_type == FileType::AIFF {
        match id3::Tag::read_from_aiff_path(filename) {
            Ok(tag) => tag,
            Err(id3::Error{kind: id3::ErrorKind::NoTag, ..}) => id3::Tag::new(),
            Err(e) => {
                return Err(ConvertError::TagReadError(Box::new(e)));
            }
        }
    } else if id3_type == FileType::WAV {
        match id3::Tag::read_from_wav_path(filename) {
            Ok(tag) => tag,
            Err(id3::Error{kind: id3::ErrorKind::NoTag, ..}) => id3::Tag::new(),
            Err(e) => {
                return Err(ConvertError::TagReadError(Box::new(e)));
            }
        }
    } else {
        return Err(ConvertError::UnknownID3File);
    };

    if playcount > 0 {
        tag.remove_extended_text(Some("SERATO_PLAYCOUNT"), None);
        tag.add_frame(frame::ExtendedText{
            description: "SERATO_PLAYCOUNT".to_owned(),
            value: format!("{}\0", playcount)
        });    
    }
    if rating > 0 {
        tag.set_text_values("TCOM",[get_rating_string(rating)]);
    }
    if hashtag != "" {
        tag.set_text_values("TIT1",[hashtag]);
    }
    if simulate {
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