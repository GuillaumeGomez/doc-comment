//
// Doc comment
//
// Copyright (c) 2018 Guillaume Gomez
//

// ! The point of this (small) crate is to allow you to add doc comments from macros or
// ! to test external markdown files' code blocks through `rustdoc`.
// !
// ! ## Including file(s) for testing
// !
// ! Let's assume you want to test code examples in your `README.md` file which
// ! looks like this:
// !
// ! ````text
// ! # A crate
// !
// ! Here is a code example:
// !
// ! ```rust
// ! let x = 2;
// ! assert!(x != 0);
// ! ```
// ! ````
// !
// ! You can use the `doc_comment!` macro to test it like this:
// !
// ! ```
// ! #[macro_use]
// ! extern crate doc_comment;
// !
// ! // When running `cargo test`, rustdoc will check this file as well.
// ! doc_comment!(include_str!("../README.md"));
// ! # fn main() {}
// ! ```
// !
// ! Please note that can also use the `doctest!` macro to have a shorter way to test an outer
// ! file:
// !
// ! ```no_run
// ! extern crate doc_comment;
// !
// ! doc_comment::doctest!("../README.md");
// ! # fn main() {}
// ! ```
// !
// ! Please also note that you can use `#[cfg(doctest)]`:
// !
// ! ```no_run
// ! #[cfg(doctest)]
// ! doc_comment::doctest!("../README.md");
// ! # fn main() {}
// ! ```
// !
// ! In this case, the examples in the `README.md` file will only be run on `cargo test`. You
// ! can find more information about `#[cfg(doctest)]` in [this blogpost](https://blog.guillaume-gomez.fr/articles/2020-03-07+cfg%28doctest%29+is+stable+and+you+should+use+it).
// !
// ! ## Generic documentation
// !
// ! Now let's imagine you want to write documentation once for multiple types but
// ! still having examples specific to each type:
// !
// ! ```
// ! // The macro which generates types
// ! macro_rules! gen_types {
// !     ($tyname:ident) => {
// !         /// This is a wonderful generated struct!
// !         ///
// !         /// You can use it as follow:
// !         ///
// !         /// ```
// !         /// let x = FirstOne {
// !         ///     field1: 0,
// !         ///     field2: 0,
// !         ///     field3: 0,
// !         ///     field4: 0,
// !         /// };
// !         ///
// !         /// println!("Created a new instance of FirstOne: {:?}", x);
// !         /// ```
// !         #[derive(Debug)]
// !         pub struct $tyname {
// !             pub field1: u8,
// !             pub field2: u16,
// !             pub field3: u32,
// !             pub field4: u64,
// !         }
// !     }
// ! }
// !
// ! // Now let's actually generate types:
// ! gen_types!(FirstOne);
// ! gen_types!(SecondOne);
// ! gen_types!(Another);
// ! ```
// !
// ! So now we have created three structs with different names, but they all have the exact same
// ! documentation, which is an issue for any structs not called `FirstOne`. That's where
// ! [`doc_comment!`] macro comes in handy!
// !
// ! Let's rewrite the `gen_types!` macro:
// !
// !     // Of course, we need to import the `doc_comment` macro:
// !     #[macro_use]
// !     extern crate doc_comment;
// !
// !     macro_rules! gen_types {
// !         ($tyname:ident) => {
// !             doc_comment! {
// !     concat!("This is a wonderful generated struct!
// !
// !     You can use it as follow:
// !
// !     ```
// !     let x = ", stringify!($tyname), " {
// !         field1: 0,
// !         field2: 0,
// !         field3: 0,
// !         field4: 0,
// !     };
// !
// !     println!(\"Created a new instance of ", stringify!($tyname), ": {:?}\", x);
// !     ```"),
// !                 #[derive(Debug)]
// !                 pub struct $tyname {
// !                     pub field1: u8,
// !                     pub field2: u16,
// !                     pub field3: u32,
// !                     pub field4: u64,
// !                 }
// !             }
// !         }
// !     }
// !
// !     gen_types!(FirstOne);
// !     gen_types!(SecondOne);
// !     gen_types!(Another);
// !     # fn main() {}
// !
// ! Now each struct has doc which match itself!

extern crate proc_macro;

use proc_macro::token_stream::IntoIter as ProcIter;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::fs;
use std::iter::Peekable;
use std::path::Path;
use std::str::FromStr;

fn include_file(ident: &str, path: &Path, includes: &mut String) -> String {
    let full_path = if !path.is_absolute() {
        let p = Path::new(file!());
        p.parent().unwrap().join(&path)
    } else {
        path.to_path_buf()
    };
    // This part is to trigger recompilation in case the file has been updated!
    includes.push_str(&format!(
        "const _: &'static str = {}!(\"{}\");",
        ident,
        path.display()
    ));
    match fs::read_to_string(&full_path) {
        Ok(s) => {
            // Not the best way but whatever...
            s.replace("\\", "\\\\").replace("\"", "\\\"")
        }
        Err(e) => panic!("Failed to read `{}`: {}", full_path.display(), e),
    }
}

fn parse_macro_call(
    ident: String,
    attrs: &mut Peekable<ProcIter>,
    out: &mut String,
    includes: &mut String,
) {
    if ident != "include_str" {
        panic!(
            "Unsupported macro call `{}` in proc_macro (only `include_str` is currently supported)",
            ident
        );
    }
    // First we remove the "!" token
    attrs.next();
    match attrs.next() {
        Some(TokenTree::Group(g)) => {
            for token in g.stream().into_iter() {
                match token {
                    TokenTree::Literal(l) => {
                        let l_s = l.to_string();
                        if !l_s.starts_with('"') {
                            panic!("`{}` should be a string literal!", l_s);
                        }
                        let l_s = &l_s[1..l_s.len() - 1];
                        let path = Path::new(&l_s);
                        out.push_str(&include_file(&ident, &path, includes));
                    }
                    TokenTree::Punct(p) if p.to_string() == "," => {}
                    x => panic!("Unexpected item `{}` in macro call `{}`", x, ident),
                }
            }
        }
        Some(x) => panic!("Unexpected `{}` in macro `{}`", x, ident),
        None => panic!("Expected item in macro `{}`, found nothing...", ident),
    }
}

fn parse_attr(attrs: TokenStream, includes: &mut String, is_inner: bool) -> String {
    let mut out = format!("#{}[doc = \"", if is_inner { "!" } else { "" });
    let mut attrs = attrs.into_iter().peekable();
    loop {
        let attr = match attrs.next() {
            Some(a) => a,
            None => break,
        };
        match attr {
            TokenTree::Literal(l) => {
                let print = l.to_string();
                if print.starts_with("b") {
                    out.push_str(&print[2..print.len() - 1]);
                } else if print.starts_with('\'') {
                    out.push_str(&print[1..print.len() - 1]);
                } else if print.starts_with('"') {
                    out.push_str(&print[1..print.len() - 1]);
                } else {
                    out.push_str(&print);
                }
            }
            TokenTree::Punct(p) if p.to_string() == "," => {}
            TokenTree::Ident(i) if attrs.peek().map(|a| a.to_string() == "!") == Some(true) => {
                parse_macro_call(i.to_string(), &mut attrs, &mut out, includes);
            }
            x => {
                panic!("Only literals are expected, found: {:?}", x);
            }
        }
    }
    out.push_str("\"]");
    out
}

fn parse_item(mut parts: Peekable<ProcIter>, includes: &mut String) -> String {
    let mut out = String::new();
    loop {
        let attr = match parts.next() {
            Some(a) => a,
            None => break,
        };
        match attr {
            TokenTree::Group(g) => {
                out.push_str(match g.delimiter() {
                    Delimiter::Parenthesis => " (",
                    Delimiter::Brace => " {",
                    Delimiter::Bracket => " [",
                    Delimiter::None => " ",
                });
                out.push_str(&parse_item(g.stream().into_iter().peekable(), includes));
                out.push_str(match g.delimiter() {
                    Delimiter::Parenthesis => ")",
                    Delimiter::Brace => "}",
                    Delimiter::Bracket => "]",
                    Delimiter::None => "",
                });
            }
            TokenTree::Punct(x) if x.to_string() == "#" => match parts.peek() {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                    let stream = g.stream();
                    let mut sub_parts = stream.into_iter();
                    match sub_parts.next() {
                        Some(TokenTree::Ident(i)) if i.to_string() == "doc_comment" => {}
                        _ => {
                            out.push_str(" ");
                            out.push_str(&x.to_string());
                        }
                    }
                    match sub_parts.next() {
                        Some(TokenTree::Group(g)) => {
                            out.push_str(" ");
                            out.push_str(&parse_attr(g.stream(), includes, false));
                            parts.next();
                        }
                        _ => {
                            out.push_str(" ");
                            out.push_str(&x.to_string());
                        }
                    }
                }
                _ => {
                    out.push_str(" ");
                    out.push_str(&x.to_string());
                }
            },
            x => {
                if !out.ends_with("#") {
                    out.push_str(" ");
                }
                out.push_str(&x.to_string());
            }
        }
    }
    out
}

fn check_if_is_inner(item: &mut Peekable<ProcIter>) -> bool {
    match item.peek() {
        Some(TokenTree::Ident(i)) => {
            i.to_string().is_empty() || format!("{:?}", i.span()) == "#0 bytes(0..0)"
        }
        _ => false,
    }
}

/// ```edition2018,no_run
/// use doc_comment::doc_comment as dc;
///
/// #[dc("the foo function!")]
/// pub fn foo() {}
///
/// #[dc("enum ", "time!")]
/// pub enum Foo {
///     #[doc_comment("variant ", 1)]
///     A,
///     #[doc_comment("variant ", 2)]
///     B,
/// }
/// ```
///
/// Unfortunately, due to current rust limitations, you can't use it as an inner attribute yet (you
/// can check the issue [here](https://github.com/rust-lang/rust/issues/41430)):
///
/// ```ignore
/// pub mod bar {
///     #![dc("yolo")] // not working!
/// }
/// ```
#[proc_macro_attribute]
pub fn doc_comment(attrs: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro::token_stream::IntoIter;
    use std::iter::FromIterator;
    let mut includes = String::new();
    let mut item = item.into_iter().peekable();
    let is_inner = check_if_is_inner(&mut item);
    let attr = TokenStream::from_str(&parse_attr(attrs, &mut includes, is_inner))
        .unwrap()
        .into_iter();
    if !is_inner {
        let item = TokenStream::from_str(&parse_item(item, &mut includes))
            .unwrap()
            .into_iter();
        let includes: IntoIter = TokenStream::from_str(&includes).unwrap().into_iter();
        TokenStream::from_iter(attr.chain(item).chain(includes))
    } else {
        loop {
            match item.next() {
                Some(TokenTree::Group(g)) => {
                    let tokens: IntoIter = g.stream().into_iter();
                    let includes: IntoIter = TokenStream::from_str(&includes).unwrap().into_iter();

                    return TokenStream::from_iter(attr.chain(tokens).chain(includes));
                }
                Some(_) => {}
                None => {
                    let includes: IntoIter = TokenStream::from_str(&includes).unwrap().into_iter();
                    // Weird case... It would meant that we can't find a "TokenTree::Group" where
                    // the inner attribute would be located...
                    return TokenStream::from_iter(attr.chain(includes));
                }
            }
        }
    }
}

/// This proc macro provides a simpler way to test an outer markdown file.
///
/// # Example
///
/// ```edition2018,no_run
/// doc_comment::doctest!("../README.md");
/// // It is the equivalent of:
/// #[doc_comment::doc_comment(include_str!("../README.md"))]
/// extern {}
///
/// // If you want to have a name for your tests:
/// doc_comment::doctest!("../README.md", another);
/// # fn main() {}
/// ```
#[proc_macro]
pub fn doctest(item: TokenStream) -> TokenStream {
    let mut parts = item.into_iter();
    let mut file_path = None;
    let mut test_name = None;

    loop {
        match parts.next() {
            Some(TokenTree::Literal(l)) => {
                let l_s = l.to_string();
                if !l_s.starts_with('"') {
                    panic!(
                        "First parameter of doctest should be a string literal, found `{}`",
                        l_s
                    );
                } else if file_path.is_some() {
                    panic!(
                        "Second parameter of doctest should be a ident, found a literal: `{}`",
                        l_s
                    );
                }
                file_path = Some(l_s[1..l_s.len() - 1].to_owned());
            }
            Some(TokenTree::Punct(p)) if p.to_string() == "," => {}
            Some(TokenTree::Ident(i)) => {
                let i_s = i.to_string();
                if file_path.is_none() {
                    panic!(
                        "First parameter of doctest should be a string literal, found ident `{}`",
                        i_s
                    );
                }
                test_name = Some(i_s);
            }
            Some(t) => panic!("Unexpected token `{}`", t),
            None => break,
        }
    }
    if file_path.is_none() && test_name.is_none() {
        panic!("doctest expects at least one parameter");
    }
    let mut includes = String::new();
    let content = include_file(
        "include_str",
        &Path::new(file_path.as_ref().unwrap()),
        &mut includes,
    );
    let item = match test_name {
        Some(t) => format!("mod {} {{}}", t),
        None => "extern {}".to_owned(),
    };
    format!("#[doc = \"{}\"]\n{}\n{}", content, item, includes)
        .parse()
        .unwrap()
}
