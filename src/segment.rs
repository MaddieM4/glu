use markdown::mdast::Code;
use std::fmt::Debug;
use crate::segment_optimizer::SegmentOptimizer;

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
