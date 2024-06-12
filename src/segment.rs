use markdown::mdast::Code;
use std::fmt::Debug;

// ----------------------------------------------------------------------------
// File type enumeration and conversion
// ----------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
enum FileType {
    Unknown,
    JavaScript,
}

impl From<&Option<String>> for FileType {
    fn from(item: &Option<String>) -> FileType {
        match &item {
            None => FileType::Unknown,
            Some(s) => FileType::from(&s[..]),
        }
    }
}
impl From<&str> for FileType {
    fn from(item: &str) -> FileType {
        match item {
            "javascript" => FileType::JavaScript,
            "js" => FileType::JavaScript,
            _ => FileType::Unknown,
        }
    }
}

impl From<FileType> for String {
    fn from(item: FileType) -> String {
        match item {
            FileType::JavaScript => "javascript",
            FileType::Unknown => "unknown",
        }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(FileType::from("js"), FileType::JavaScript);
        assert_eq!(FileType::from("javascript"), FileType::JavaScript);
        assert_eq!(FileType::from("foo"), FileType::Unknown);
    }

    #[test]
    fn from_option_string() {
        assert_eq!(FileType::from(&Some("js".to_string())), FileType::JavaScript);
        assert_eq!(FileType::from(&Some("foo".to_string())), FileType::Unknown);
        assert_eq!(FileType::from(&None), FileType::Unknown);
    }

    #[test]
    fn to_string() {
        assert_eq!(String::from(FileType::JavaScript), "javascript".to_string());
        assert_eq!(String::from(FileType::Unknown), "unknown".to_string());
    }
}

// ----------------------------------------------------------------------------
// Segment optimization
// ----------------------------------------------------------------------------

struct SegmentOptimizer<'a> {
    lines: Vec<&'a str>,
    min_indent: u8,

    inferred_type: FileType,
    inferred_path: Option<String>,
}

impl <'a> From<&'a Code> for SegmentOptimizer<'a> {
    fn from(c: &'a Code) -> SegmentOptimizer<'a> {
        let lines: Vec<&str> = c.value.lines().collect();
        let itype: FileType = (&c.lang).into();

        SegmentOptimizer {
            lines: lines,
            min_indent: 0,
            inferred_type: itype,
            inferred_path: None,
        }
    }
}

// ----------------------------------------------------------------------------
// Segments
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Segment {
    pub file_type: String,
    pub file_name: String,
    pub contents: String,
}

impl From<&Code> for Segment {
    fn from(item: &Code) -> Segment {
        let so: SegmentOptimizer = item.into();
        return Segment {
            file_type: so.inferred_type.into(),
            file_name: so.inferred_path.unwrap_or("filename.txt".into()),
            contents: so.lines.join("\n"),
        }
    }
}
