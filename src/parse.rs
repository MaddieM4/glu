use markdown::{to_mdast, ParseOptions};
use markdown::mdast::{Node,Code};
use crate::segment::Segment;

// This wraps up the higher-level process of generating segments from MD text.
//
// There's probably a lot of room to improve performance by eliminating copy
// operations in memory - the sloppiness is because I'm still learning Rust.
// I promise it itches me, but this will probably always be adequate.
// Realistically the bottleneck will be file IO.
pub fn parse(md_text: &str) -> Vec<Segment> {
    // Per docs: cannot fail with MDX off
    let tree = to_mdast(md_text, &ParseOptions::default()).unwrap();

    // Build a buffer of Code structs
    let mut code_nodes: Vec<Code> = vec![];
    find_code_nodes(&tree, &mut code_nodes);

    // Convert them all!
    return code_nodes.iter().map(|c| Segment::from(c)).collect();
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
