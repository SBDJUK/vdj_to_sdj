use crate::common::ConvertError;

pub fn update_ogg_tag(_filename: &str, _playcount: u32, _rating: usize, _hashtag: &str, _simulate: bool) -> Result<(), ConvertError> {
    Err(ConvertError::UnknownExtension(format!("OGG is not yet supported")))
}