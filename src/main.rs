use markdown::{to_mdast, ParseOptions};
use markdown::mdast::{Node,Code};
use std::fmt::{Debug, Formatter};

struct Segment {
    file_type: String,
    file_name: String,
    contents: String,
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

impl Debug for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Segment {{ file_type: {:?}, file_name: {:?}, contents: {:?} }}",
            self.file_type, self.file_name, self.contents)
    }
}

// Recursively explore a Markdown parse tree and find all the code segments
fn find_code_nodes(node: &Node, output_buf: &mut Vec<Code>) {
    match node {
        Node::Code(c) => output_buf.push(c.clone()),
        _ => node.children().unwrap_or(&vec![]).iter().for_each(
            |n| find_code_nodes(n, output_buf)
        ),
    };
}

fn main() {
    // Per docs: cannot fail with MDX off
    let md_text = "
Hello, world!

```javascript
// some_file_name.js
const foo = 'bar';
```
    ";
    let tree = to_mdast(md_text, &ParseOptions::default()).unwrap();
    println!("Tree: {:?}", &tree);

    let mut code_nodes: Vec<Code> = vec![];
    find_code_nodes(&tree, &mut code_nodes);
    println!("Code Nodes: {:?}", code_nodes);

    let segments: Vec<Segment> = code_nodes.iter().map(|c| Segment::from(c)).collect();
    println!("Segments: {:?}", segments);
}
