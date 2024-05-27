use crate::parse::parse;
mod segment;
mod parse;

fn main() {
    let md_text = "
Hello, world!

```javascript
// some_file_name.js
const foo = 'bar';
```
    ";
    let segments = parse(&md_text);
    println!("Segments: {:?}", segments);
}
