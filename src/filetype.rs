#[derive(PartialEq, Debug, Copy, Clone)]
pub enum FileType {
    Unknown,
    JavaScript,
    Bash,
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
            "bash" => FileType::Bash,
            _ => FileType::Unknown,
        }
    }
}

impl From<FileType> for String {
    fn from(item: FileType) -> String {
        match item {
            FileType::JavaScript => "javascript",
            FileType::Bash => "bash",
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
