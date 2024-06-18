use regex::Regex;

// ----------------------------------------------------------------------------
// Base FileType
// ----------------------------------------------------------------------------

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum FileType {
    Asm,
    Bash,
    C,
    JavaScript,
    Rust,
    Unknown,
}

impl From<&Option<String>> for FileType {
    fn from(item: &Option<String>) -> FileType {
        match &item {
            Some(s) => FileType::from(&s[..]),
            None => FileType::Unknown,
        }
    }
}
impl From<&str> for FileType {
    fn from(item: &str) -> FileType {
        match item {
            "asm" => FileType::Asm,
            "bash" => FileType::Bash,
            "c" => FileType::C,
            "javascript" => FileType::JavaScript,
            "js" => FileType::JavaScript,
            "rust" => FileType::Rust,
            "rs" => FileType::Rust,
            _ => FileType::Unknown,
        }
    }
}

impl From<FileType> for String {
    fn from(item: FileType) -> String {
        match item {
            FileType::Asm => "asm",
            FileType::Bash => "bash",
            FileType::C => "c",
            FileType::JavaScript => "javascript",
            FileType::Rust => "rust",
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
// Path Detection
// ----------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
pub struct PathDetection {
    pub line_number: usize,
    pub path: String,
}

pub fn detect_path(ft: FileType, lines: &Vec<&str>) -> Option<PathDetection> {
    let pat: &str = match ft {
        FileType::Asm => r";\s*(\w.*\.(s|asm)\b)",
        FileType::Bash => r"#\s*(\w.*\.sh\b)",
        FileType::C => r"//\s*(\w.*\.(c|h)\b)",
        FileType::JavaScript => r"//\s*(\w.*\.js\b)",
        FileType::Rust => r"//\s*(\w.*\.rs\b)",
        FileType::Unknown => return None,
    };
    let re = Regex::new(pat).expect("Failed to compile regex");

    lines
        .iter()
        .take(3)
        .map(|line| re.captures(line))
        .enumerate()
        .find(|(_, cap)| cap.is_some())
        .map(|(count, cap)| PathDetection {
            line_number: count,
            path: cap.unwrap().get(1).unwrap().as_str().to_string(),
        })
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_some(ft: FileType, line_number: usize, path: &str, lines: Vec<&str>) {
        assert_eq!(detect_path(ft, &lines),
            Some(PathDetection {
                line_number: line_number,
                path: path.to_string(),
            }));
    }

    fn check_none(ft: FileType, lines: Vec<&str>) {
        assert_eq!(detect_path(ft, &lines), None);
    }

    #[test]
    fn test_asm() {
        let ft = FileType::Asm;
        check_none(ft, vec![]);
        check_none(ft, vec![
            ".intel_syntax noprefix",
        ]);

        check_some(ft, 0, "foo.asm", vec![
            "; foo.asm",
            "",
            ".intel_syntax noprefix",
        ]);

        check_some(ft, 1, "bar.s", vec![
            ".intel_syntax noprefix",
            "; bar.s",
        ]);
    }

    #[test]
    fn test_bash() {
        let ft = FileType::Bash;
        check_none(ft, vec![]);
        check_none(ft, vec![
            "#!/bin/bash",
            "echo hello world",
        ]);

        check_some(ft, 0, "script.sh", vec![
            "# script.sh",
            "",
            "echo hello world",
        ]);

        check_some(ft, 1, "second_line.sh", vec![
            "#!/bin/bash",
            "# second_line.sh",
            "set -ex",
            "",
            "echo hello world",
        ]);
    }

    #[test]
    fn test_c() {
        let ft = FileType::C;
        check_none(ft, vec![]);
        check_none(ft, vec![
            "#define FOO 1",
        ]);

        check_some(ft, 0, "main.c", vec![
            "// main.c",
            "",
            "#define FOO 1",
        ]);

        check_some(ft, 1, "header.h", vec![
            "// A header file",
            "// header.h",
            "",
            "#define FOO 1",
        ]);
    }

    #[test]
    fn test_rust() {
        let ft = FileType::Rust;
        check_none(ft, vec![
            "fn main() {",
            "    println!('Hello world');",
            "}",
        ]);
        check_some(ft, 0, "foo.rs", vec![
            "// foo.rs",
            "",
            "fn main() {",
            "    println!('Hello world');",
            "}",
        ]);
    }

    #[test]
    fn test_js() {
        let ft = FileType::JavaScript;
        check_none(ft, vec![]);
        check_none(ft, vec![
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);

        check_some(ft, 0, "foo.js", vec![
            "// foo.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);

        check_some(ft, 2, "third_line.js", vec![
            "// First line",
            "// Second line",
            "// third_line.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);

        check_none(ft, vec![
            "// First line",
            "// Second line",
            "// Third line",
            "// fourth_line.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);
    }

    #[test]
    fn test_unknown() {
        let ft = FileType::Unknown;
        check_none(ft, vec![]);
        check_none(ft, vec!["Anything"]);
    }
}
