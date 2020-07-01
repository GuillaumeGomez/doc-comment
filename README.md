# doc-comment [![][img_travis-ci]][travis-ci] [![][img_crates]][crates] [![][img_doc]][doc]

[img_travis-ci]: https://api.travis-ci.org/GuillaumeGomez/doc-comment.png?branch=master
[travis-ci]: https://travis-ci.org/GuillaumeGomez/doc-comment

[img_crates]: https://img.shields.io/crates/v/doc-comment.svg
[crates]: https://crates.io/crates/doc-comment

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

Write doc comments from macros.

For now, please keep using version `0.3.*` as the `0.4` is incomplete and is waiting for https://github.com/rust-lang/rust/issues/47809 to get stabilized.

## Usage example

````rust,no_run,edition2018
extern crate doc_comment;

// If you want to test examples in your README file.
doc_comment::doctest!("../README.md");
// If you want to "name" your tests.
doc_comment::doctest!("../README.md", readme);

// If you want to test your README file ONLY on "cargo test":
#[cfg(doctest)]
doc_comment::doctest!("../README.md");

// If you want to document an item:
#[doc_comment::doc_comment("fooo", "or not foo")]
pub struct Foo {
    #[doc_comment("a field!")]
    field: i32,
}
````

## proc-macro

From the version `0.4`, this crate will use `proc-macro` instead of `macros`, meaning that the minimum rust version will also greatly increase. If you don't want the `proc-macro` then use the `0.3.*` versions!

For more information, take a look at the [documentation][doc].

[doc]: https://docs.rs/doc-comment/
