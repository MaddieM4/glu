use markdown::mdast::Code;
use std::fmt::Debug;
use crate::segment_optimizer::SegmentOptimizer;

#[derive(PartialEq, Debug)]
pub struct Segment {
    pub file_type: String,
    pub file_name: String,
    pub contents: String,
}

impl From<&Code> for Segment {
    fn from(item: &Code) -> Segment {
        let raw_so: SegmentOptimizer = item.into();
        let so = raw_so.optimize();
        return Segment {
            file_type: so.inferred_type.into(),
            file_name: so.inferred_path.unwrap_or("filename.txt".into()),
            contents: so.lines.join("\n") + "\n",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_optimize() {
        let code = Code {
            meta: None,
            position: None,
            value: "// foo.js\n\nlet x = 0;".to_string(),
            lang: Some("js".to_string()),
        };
        let seg: Segment = (&code).into();

        assert_eq!(seg, Segment {
            file_type: "javascript".to_string(),
            file_name: "foo.js".to_string(),
            contents: "let x = 0;\n".to_string(),
        });
    }
}
