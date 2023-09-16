use mp4ameta::{Data, FreeformIdent};
use base64::{Engine as _, engine::general_purpose};
use crate::common::{ConvertError,get_rating_string};

fn get_base64_padded_playcount(pc: u32) -> String {
    let binding = pc.to_string();
    let int_bytes = binding.as_bytes();
    let buffer_size = std::cmp::max(3, (int_bytes.len() + 2 + 2) / 3 * 3);
    let mut buffer = vec![0u8; buffer_size];
    buffer[..int_bytes.len()].copy_from_slice(int_bytes);
    general_purpose::STANDARD_NO_PAD.encode(&buffer)
}

pub fn update_mp4_tag(filename: &str, playcount: u32, rating: usize, hashtag: &str, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = mp4ameta::Tag::read_from_path(filename).map_err(|e| ConvertError::TagReadError(Box::new(e)))?;
    if playcount > 0 {
        tag.set_data(FreeformIdent::new("com.serato.dj", "playcount"), Data::Utf8(get_base64_padded_playcount(playcount)));
    }
    if rating > 0 {
        tag.set_composer(get_rating_string(rating));
    }
    if hashtag != "" {
        tag.set_grouping(hashtag);
    }
    if !simulate {
        tag.write_to_path(filename).map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    }
    Ok(())
}