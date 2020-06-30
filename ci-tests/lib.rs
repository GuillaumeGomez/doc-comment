#[doc_comment::doc_comment(include_str!("../README.md"))]
extern {}

// We can use parenthesis too, but not in rustc 1.44.1...
doc_comment::doctest!{ "../README.md" }