use markdown::mdast::Code;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Segment {
    pub file_type: String,
    pub file_name: String,
    pub contents: String,
}

impl From<&Code> for Segment {
    fn from(item: &Code) -> Segment {
        let ftype = match &item.lang {
            Some(lang) => lang.clone(),
            None => "Unknown".to_string(),
        };

        return Segment {
            file_type: ftype,
            file_name: "Not implemented".to_string(),
            contents: item.value.clone(),
        }
    }
}
