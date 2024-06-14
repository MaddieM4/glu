use regex::Regex;
use markdown::mdast::Code;
use crate::filetype::FileType;

#[derive(PartialEq, Debug)]
pub struct SegmentOptimizer<'a> {
    pub lines: Vec<&'a str>,
    pub inferred_type: FileType,
    pub inferred_path: Option<String>,
}

impl <'a> From<&'a Code> for SegmentOptimizer<'a> {
    fn from(c: &'a Code) -> SegmentOptimizer<'a> {
        let lines: Vec<&str> = c.value.lines().collect();
        let itype: FileType = (&c.lang).into();

        SegmentOptimizer {
            lines: lines,
            inferred_type: itype,
            inferred_path: None,
        }
    }
}

// ----------------------------------------------------------------------------
// Internal optimizers
// ----------------------------------------------------------------------------

fn find_min_indent(lines: &Vec<&str>) -> usize {
    lines
        .iter()
        .filter(|s| **s != "")
        .map(|s| 
            s.chars().take_while(|ch| ch.is_whitespace()).count()
        )
        .min()
        .unwrap_or_default()
}

fn trim_indent<'a>(line: &'a str, amount: usize) -> &'a str {
    let pat = format!(r"^\s{{0,{}}}(.*)", amount);
    let re = Regex::new(&pat).unwrap();
    let m = re.captures(line).unwrap().get(1).unwrap();
    m.as_str()
}

fn fix_indents<'a>(lines: Vec<&'a str>) -> Vec<&'a str> {
    let min_indent = find_min_indent(&lines);
    lines.iter().map(|s| trim_indent(s, min_indent)).collect()
}

fn trim_empty_lines<'a>(lines: Vec<&'a str>) -> Vec<&'a str> {
    if lines.is_empty() {
        return lines;
    }

    let mut start: usize = 0;
    let mut end: usize = lines.len()-1;
    while start <= end && lines[start] == "" {
        start = start + 1;
    }
    while end >= start && lines[end] == "" {
        end = end - 1;
    }

    return lines[start..end+1].to_vec();
}

// ----------------------------------------------------------------------------
// API for multi-strategy optimization
// ----------------------------------------------------------------------------

fn opt_once<'a>(so: &SegmentOptimizer<'a>) -> SegmentOptimizer<'a> {
    // TODO: Maybe avoid some clones?
    SegmentOptimizer {
        lines: trim_empty_lines(fix_indents(so.lines.clone())),
        inferred_type: so.inferred_type,
        inferred_path: so.inferred_path.clone(),
    }
}

// Optimize until settled
fn optimize<'a>(so: SegmentOptimizer<'a>) -> SegmentOptimizer<'a> {
    let mut prev = so;
    let mut current = opt_once(&prev);
    while current != prev {
        prev = current;
        current = opt_once(&prev);
    }
    return current;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_code_empty() {
        let code = Code {
            meta: None,
            position: None,
            value: "".to_string(),
            lang: None,
        };
        let so: SegmentOptimizer = (&code).into();
        assert_eq!(so, SegmentOptimizer {
            lines: vec![],
            inferred_type: FileType::Unknown,
            inferred_path: None,
        });
    }


    #[test]
    fn test_from_code_populated() {
        let code = Code {
            meta: None,
            position: None,
            value: "// foo.js\n\nlet x = 0;".to_string(),
            lang: Some("js".to_string()),
        };
        let so: SegmentOptimizer = (&code).into();
        assert_eq!(so, SegmentOptimizer {
            lines: vec!["// foo.js", "", "let x = 0;"],
            inferred_type: FileType::JavaScript,
            inferred_path: None,
        });
    }

    #[test]
    fn test_find_min_indent() {
        assert_eq!(find_min_indent(&vec![]), 0);
        assert_eq!(find_min_indent(&vec!["Hello world"]), 0);
        assert_eq!(find_min_indent(&vec!["   Hello world"]), 3);
        assert_eq!(find_min_indent(&vec!["No space", " One space", "  Two space"]), 0);
        assert_eq!(find_min_indent(&vec!["  Ignore", "", "  empty lines"]), 2);
    }

    #[test]
    fn test_trim_indent() {
        assert_eq!(trim_indent("", 5), "");
        assert_eq!(trim_indent("foo", 5), "foo");
        assert_eq!(trim_indent("   Three spaces", 0), "   Three spaces");
        assert_eq!(trim_indent("   Three spaces", 1), "  Three spaces");
        assert_eq!(trim_indent("   Three spaces", 2), " Three spaces");
        assert_eq!(trim_indent("   Three spaces", 3), "Three spaces");
        assert_eq!(trim_indent("   Three spaces", 4), "Three spaces");
    }

    #[test]
    fn test_fix_indents() {
        let lines = vec![
            "    // foo.js",
            "",
            "    function foo() {",
            "        console.log(100);",
            "        ",
            "    }",
        ];

        assert_eq!(fix_indents(lines), vec![
            "// foo.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);
    }


    #[test]
    fn test_trim_empty_lines() {
        let lines = vec![
            "",
            "",
            "// foo.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
            "",
            "",
            "",
            "",
        ];

        assert_eq!(trim_empty_lines(lines), vec![
            "// foo.js",
            "",
            "function foo() {",
            "    console.log(100);",
            "    ",
            "}",
        ]);
    }

    #[test]
    fn test_optimize() {
        let so = SegmentOptimizer {
            lines: vec![
                "",
                "",
                "    // foo.js",
                "",
                "    function foo() {",
                "        console.log(100);",
                "        ",
                "    }",
                "    ",
                "",
                "",
                "",
            ],
            inferred_type: FileType::JavaScript,
            inferred_path: None,
        };

        assert_eq!(optimize(so), SegmentOptimizer {
            lines: vec![
                "// foo.js",
                "",
                "function foo() {",
                "    console.log(100);",
                "    ",
                "}",
            ],
            inferred_type: FileType::JavaScript,
            inferred_path: None,
        });
    }
}
