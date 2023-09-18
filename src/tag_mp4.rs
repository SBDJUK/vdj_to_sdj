use mp4ameta::{Data, FreeformIdent, Tag};
use base64::{Engine as _, engine::general_purpose};
use crate::common::{Config,ConfigBlock,ConvertError,get_rating_string,TrackData};

const SERATO_VENDOR: &str = "com.serato.dj";
const SERATO_PLAYCOUNT_FRAME: &str = "playcount";

fn get_base64_padded_playcount(pc: u32) -> String {
    let binding = pc.to_string();
    let int_bytes = binding.as_bytes();
    let buffer_size = std::cmp::max(3, (int_bytes.len() + 2 + 2) / 3 * 3);
    let mut buffer = vec![0u8; buffer_size];
    buffer[..int_bytes.len()].copy_from_slice(int_bytes);
    general_purpose::STANDARD_NO_PAD.encode(&buffer)
}

fn get_set_tag_function(field: &str) -> fn(&mut Tag, &str) -> bool {
    match field {
        "Album" => set_album,
        "Comment" => set_comment,
        "Composer" => set_composer,
        "Genre" => set_genre,
        "Grouping" => set_grouping,
        //"Label" => "TPUB",  // Do not seem to be available in the files tag, must be in the SDJ database?
        //"Remixer" => "TPE4",
        //"Year" => "TDRC",
        _ => set_default,
    }
}

fn set_album(tag: &mut Tag, value: &str) -> bool {
    tag.set_album(value);
    true
}

fn set_comment(tag: &mut Tag, value: &str)  -> bool {
    tag.set_comment(value);
    true
}

fn set_composer(tag: &mut Tag, value: &str) -> bool {
    tag.set_composer(value);
    true
}

fn set_genre(tag: &mut Tag, value: &str) -> bool {
    tag.set_genre(value);
    true
}

fn set_grouping(tag: &mut Tag, value: &str) -> bool {
    tag.set_grouping(value);
    true
}

fn set_default(_tag: &mut Tag, _value: &str) -> bool {
    false
}

fn process_field(config: &Option<ConfigBlock>, data: &str, tag: &mut Tag) -> bool {
    match config {
        Some(block) if block.enabled && !data.is_empty() => {
            let setter = get_set_tag_function(&block.target);
            if setter(tag, data) {
                return true;
            }
        }
        _ => {}
    }
    false
}

fn process_playcount(playcount: u32, tag: &mut Tag) -> bool {
    if playcount > 0 {
        tag.set_data(FreeformIdent::new(SERATO_VENDOR, SERATO_PLAYCOUNT_FRAME), Data::Utf8(get_base64_padded_playcount(playcount)));
        return true; 
    }
    false
}

pub fn update_mp4_tag(filename: &str, config: &Config, data: TrackData, simulate: bool) -> Result<(), ConvertError> {
    let mut tag = Tag::read_from_path(filename).map_err(|e| ConvertError::TagReadError(Box::new(e)))?;

    let mut changed = Vec::new();
    changed.push(process_playcount(data.playcount, &mut tag));
    changed.push(process_field(&config.rating, &get_rating_string(data.rating), &mut tag));
    changed.push(process_field(&config.user1, &data.user1, &mut tag));
    changed.push(process_field(&config.user2, &data.user2, &mut tag));

    if simulate || !changed.iter().any(|&change| change) {
        return Ok(());
    }

    tag.write_to_path(filename).map_err(|e| ConvertError::TagWriteError(Box::new(e)))?;
    Ok(())
}