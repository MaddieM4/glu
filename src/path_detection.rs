use regex::Regex;
use crate::filetype::FileType;

#[derive(PartialEq, Debug)]
pub struct PathDetection {
    pub line_number: usize,
    pub path: String,
}

pub fn detect_path(ft: FileType, lines: &Vec<&str>) -> Option<PathDetection> {
    let pat: &str = match ft {
        FileType::JavaScript => r"//\s*(\w.*\.js\b)",
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
    fn test_unknown() {
        let ft = FileType::Unknown;
        check_none(ft, vec![]);
        check_none(ft, vec!["Anything"]);
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
}
