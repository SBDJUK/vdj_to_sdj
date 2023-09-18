use crate::common::{Config,ConvertError,TrackData};

pub fn update_ogg_tag(_filename: &str, _config: &Config, _data: TrackData, _simulate: bool) -> Result<(), ConvertError> {
    Err(ConvertError::UnknownExtension(format!("OGG is not yet supported")))
}