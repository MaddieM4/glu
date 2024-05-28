use tempdir::TempDir;
use std::process::Command;

use crate::parse::parse;
use crate::writer::write_files;
mod segment;
mod parse;
mod writer;

fn main() {
    let md_text = "
Hello, world!

```javascript
// some_file_name.js
const foo = 'bar';
```
    ";
    let segments = parse(&md_text);
    let tmp = TempDir::new("glu").expect("Failed to create temp directory");
    write_files(&tmp, &segments).expect("Failed to write files");

    Command::new("tree")
        .arg(tmp.path())
        .spawn()
        .expect("Failed to spawn 'tree' command")
        .wait()
        .expect("Failed to wait for 'tree' to finish");

    Command::new("head")
        .arg(tmp.path().join("filename.txt"))
        .spawn()
        .expect("Failed to spawn 'head' command")
        .wait()
        .expect("Failed to wait for 'head' to finish");
}
