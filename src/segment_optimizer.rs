use markdown::mdast::Code;
use crate::filetype::FileType;

pub struct SegmentOptimizer<'a> {
    pub lines: Vec<&'a str>,
    min_indent: u8,

    pub inferred_type: FileType,
    pub inferred_path: Option<String>,
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

