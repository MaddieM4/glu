use markdown::{to_mdast, ParseOptions};
use markdown::mdast::{Node,Code};
use crate::segment::Segment;

mod segment;

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
