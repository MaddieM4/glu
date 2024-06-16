# GLU - Decompose Markdown files into directories

Sometimes you'll encounter an article online with a bunch of helpful examples,
or you're trying to _write_ such an article without it being full of errors,
and it would be very nice for the article to be the "upstream source of truth"
from which an entire directory of files can be built.

Well, that's what GLU is for. You give it a Markdown file, it builds a temp
directory with the files from the Markdown. This directory is cleaned up when
the GLU process exits. In the meantime, you can check that everything is as it
should be, run any commands you want, etc.

### How does that work?

GLU looks for code blocks, like this one:

```javascript
// hello.js

console.log("Hello, world!");
```

You'll notice that I specified a language for the code block (in Github syntax)
and a filename _within_ the code block, in a comment. Different languages have
different comment syntax, which is why it's important to provide both the
language and the filename comment. Given both, GLU can figure out where your
file should be unpacked to.

If GLU isn't smart enough to figure out the correct filename, it currently
defaults to `filename.txt`, which will be overwritten repeatedly if there are
multiple code blocks attributed to `filename.txt`.

Finally, GLU does a bit of cleanup of your code blocks to trim out the filename
comment, empty trailing and starting lines, and excessive whitespace. Some of
those features will probably be configurable eventually. But for that `hello.js`
example, you'd find a file that just contained the `console.log` line.

### How do I run this?

```bash
# examples.sh

# Unpack to a temp directory and open a bash shell.
# Files will be deleted when shell is closed.
glu path/to/markdown/file.md

# Bash is just the default, the command to run is an optional second arg.
glu README.md tree
```

### Project goals

 * [ ] Improve command support to be more shell-like
 * [ ] Comment syntax for lots more languages
 * [ ] Support URLs for downloading a source file
 * [ ] Support Accept header on my personal website to deliver Markdown when requested
 * [ ] Polish up package metadata and make available via `cargo install`
 * [ ] Make sure we have a LICENSE
 * [ ] Support HTML input (as long as it conforms to certain expectations)

### Contributing

Clone the directory, tinker, make a PR. If you've used Rust and Cargo before,
it shouldn't be difficult or surprising. If it is, it's probably for reasons
I'd have a hard time anticipating.

Be aware that `src/path_detection.rs` is probably the thing most people will
care about contributing to, to improve language support. I'll accept these PRs
very gratefully, but only if they include relevant tests! There are examples
you can use as a starting point, so that's not as hard as it sounds.
