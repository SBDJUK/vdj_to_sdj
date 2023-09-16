use metaflac::Tag;
use crate::common::{ConvertError,get_rating_string};

pub fn update_flac_tag(filename: &str, playcount: u32, rating: usize, hashtag: &str, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = Tag::read_from_path(filename).map_err(|e| ConvertError::TagReadError(Box::new(e)))?;
    if playcount > 0 {
        let mut pc = playcount.to_string();
        pc.push('\0');
        tag.set_vorbis("serato_playcount".to_owned(), vec!(pc))
    }
    if rating > 0 {
        tag.set_vorbis("composer".to_owned(), vec!(get_rating_string(rating)))
    }
    if hashtag != "" {
        tag.set_vorbis("grouping".to_owned(), vec!(hashtag))
    }
    if !simulate {
        tag.save().map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    }
    Ok(())
}