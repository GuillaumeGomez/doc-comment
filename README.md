# doc-comment [![][img_travis-ci]][travis-ci] [![][img_crates]][crates] [![][img_doc]][doc]

[img_travis-ci]: https://api.travis-ci.org/GuillaumeGomez/doc-comment.png?branch=master
[travis-ci]: https://travis-ci.org/GuillaumeGomez/doc-comment

[img_crates]: https://img.shields.io/crates/v/doc-comment.svg
[crates]: https://crates.io/crates/doc-comment

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

Write doc comments from macros.

## Usage example

     // Of course, we need to import the `doc_comment` macro:
     #[macro_use]
     extern crate doc_comment;

     macro_rules! gen_types {
         ($tyname:ident) => {
             doc_comment! {
     concat!("This is a wonderful generated struct!

     You can use it as follow:

     ```
     let x = ", stringify!($tyname), " {
         field1: 0,
         field2: 0,
         field3: 0,
         field4: 0,
     };

     println!(\"Created a new instance of ", stringify!($tyname), ": {:?}\", x);
     ```"),
                 #[derive(Debug)]
                 pub struct $tyname {
                     pub field1: u8,
                     pub field2: u16,
                     pub field3: u32,
                     pub field4: u64,
                 }
             }
         }
     }

     gen_types!(FirstOne);
     gen_types!(SecondOne);
     gen_types!(Another);

For more information, take a look at the [documentation][doc].

[doc]: https://docs.rs/doc-comment/