# doc-comment [![][img_travis-ci]][travis-ci] [![][img_crates]][crates] [![][img_doc]][doc]

[img_travis-ci]: https://api.travis-ci.org/GuillaumeGomez/doc-comment.png?branch=master
[travis-ci]: https://travis-ci.org/GuillaumeGomez/doc-comment

[img_crates]: https://img.shields.io/crates/v/doc-comment.svg
[crates]: https://crates.io/crates/doc-comment

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

Write doc comments from macros.

## Usage example

````rust
// Of course, we need to import the `doc_comment` macro:
#[macro_use]
extern crate doc_comment;

// If you want to test examples in your README file.
doctest!("../README.md");

// If you want to document an item:
doc_comment!(concat!("fooo", "or not foo"), pub struct Foo {});
````

For more information, take a look at the [documentation][doc].

[doc]: https://docs.rs/doc-comment/

## Sponsor

<a href="https://g.readmeads.com/readmeads-doc-comment" target="_blank">
    <img src="https://assets.readmeads.com/advertisers/readmeads/doc-comment.png?" />
</a>

`doc-comment` is proudly supported by [ReadMeAds](https://g.readmeads.com/readmeads-doc-comment). ReadMeAds is the best way to start passively monetizing the open source work that you do.  
