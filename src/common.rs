#[derive(Debug)]
pub enum ConvertError {
    UnknownExtension(String),
    UnknownID3File,
    TagReadError(Box<dyn std::error::Error + Send + Sync>),
    TagWriteError(Box<dyn std::error::Error + Send + Sync>),
}

pub fn get_rating_string(rating: usize) -> String {
    '★'.to_string().repeat(rating)
}