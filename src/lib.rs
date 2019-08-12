//
// Doc comment
//
// Copyright (c) 2018 Guillaume Gomez
//

#![cfg_attr(feature = "no_core", feature(no_core))]
#![cfg_attr(feature = "no_core", no_core)]
#![cfg_attr(not(feature = "no_core"), no_std)]

//! The point of this (small) crate is to allow you to add doc comments from macros or
//! to test external markdown files' code blocks through `rustdoc`.
//!
//! It's especially useful when generating types with macros. For example:
//!
//! ```
//! // The macro which generates types
//! macro_rules! gen_types {
//!     ($tyname:ident) => {
//!         /// This is a wonderful generated struct!
//!         ///
//!         /// You can use it as follow:
//!         ///
//!         /// ```
//!         /// let x = FirstOne {
//!         ///     field1: 0,
//!         ///     field2: 0,
//!         ///     field3: 0,
//!         ///     field4: 0,
//!         /// };
//!         ///
//!         /// println!("Created a new instance of FirstOne: {:?}", x);
//!         /// ```
//!         #[derive(Debug)]
//!         pub struct $tyname {
//!             pub field1: u8,
//!             pub field2: u16,
//!             pub field3: u32,
//!             pub field4: u64,
//!         }
//!     }
//! }
//!
//! // Now let's actually generate types:
//! gen_types!(FirstOne);
//! gen_types!(SecondOne);
//! gen_types!(Another);
//! ```
//!
//! So now we have created three structs with different names, but they all have the exact same
//! documentation, which is an issue for any structs not called `FirstOne`. That's where
//! [`doc_comment!`] macro comes in handy!
//!
//! Let's rewrite the `gen_types!` macro:
//!
//!     // Of course, we need to import the `doc_comment` macro:
//!     #[macro_use]
//!     extern crate doc_comment;
//!
//!     macro_rules! gen_types {
//!         ($tyname:ident) => {
//!             doc_comment! {
//!     concat!("This is a wonderful generated struct!
//!
//!     You can use it as follow:
//!
//!     ```
//!     let x = ", stringify!($tyname), " {
//!         field1: 0,
//!         field2: 0,
//!         field3: 0,
//!         field4: 0,
//!     };
//!
//!     println!(\"Created a new instance of ", stringify!($tyname), ": {:?}\", x);
//!     ```"),
//!                 #[derive(Debug)]
//!                 pub struct $tyname {
//!                     pub field1: u8,
//!                     pub field2: u16,
//!                     pub field3: u32,
//!                     pub field4: u64,
//!                 }
//!             }
//!         }
//!     }
//!
//!     gen_types!(FirstOne);
//!     gen_types!(SecondOne);
//!     gen_types!(Another);
//!     # fn main() {}
//!
//! Now each struct has doc which match itself!
//!
//! Now let's assume you want to test code examples in your `README.md` file which
//! looks like this:
//!
//! ````text
//! # A crate
//!
//! Here is a code example:
//!
//! ```rust
//! let x = 2;
//! assert!(x != 0);
//! ```
//! ````
//!
//! You can use the `doc_comment!` macro to test it like this:
//!
//! ```
//! #[macro_use]
//! extern crate doc_comment;
//!
//! // When running `cargo test`, rustdoc will check this file as well.
//! doc_comment!(include_str!("../README.md"));
//! # fn main() {}
//! ```
//!
//! Please note that can also use the `doctest!` macro to have a shorter way to test an outer
//! file:
//!
//! ```
//! #[macro_use]
//! extern crate doc_comment;
//!
//! doctest!("../README.md");
//! # fn main() {}
//! ```

/// This macro can be used to generate documentation upon a type/item (or just to test outer
/// markdown file code examples).
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate doc_comment;
///
/// // If you just want to test an outer markdown file:
/// doc_comment!(include_str!("../README.md"));
///
/// // If you want to document an item:
/// doc_comment!("fooo", pub struct Foo {});
/// doc_comment!("fooo?", struct Fooo(usize));
/// # fn main() {}
/// ```
///
/// # More advanced usage
///
/// It's also possible to use this macro to add documentation on types' fields as well,
/// but then you'll have to declare the full type inside the macro:
///
/// ```
/// #[macro_use]
/// extern crate doc_comment;
///
/// doc_comment!(
///     pub struct Bar { // no doc comment on the type itself since it's optional
///         "field", // <- the comma here is mandatory
///         pub field: usize,
///         "hidden field", // <- still mandatory
///         private_field: usize, // commas are mandatory after fields declaration as well
///     }
/// );
/// 
/// doc_comment!(
///     "This is an enum!", // <- comma is mandatory
///     enum Enum {
///         "first variant", // <- the comma here is mandatory
///         #[cfg(windows)]
///         Variant(usize),
///         "second variant", // still mandatory
///         SecondVariant,
///         "foo",
///         Third {foo: usize},
///         NoDocComment, // Doc comments are optional
///     }
/// );
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! doc_comment {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
    // enum types
    ($($x:expr,)? $visibility:vis enum $ty_name:ident {
        $(
          $($y:expr,)? // the doc comment if any
          $(#[$meta:meta])? // metadata
          $field:ident // variant name
          $(($($field_ty:ty $(,)?)+))? // newtype-like variant such as Variant(usize)
          $({
              $($field_name2:ident: $field_ty2:ty $(,)?)+
            })?, // struct-like variant such as Variant { foo: usize }
        )*
    }) => {
        $(#[doc = $x])?
        $visibility enum $ty_name {
            $($(#[doc = $y])?
              $(#[$meta])?
              $field $(($($field_ty,)+))? $({$($field_name2: $field_ty2 ,)+})?,)*
        }
    };
    // struct types
    ($($x:expr,)? $visibility:vis struct $ty_name:ident {
        $(
          $($y:expr,)? // the doc comment if any
          $(#[$meta:meta])? // metadata
          $visibility_i:vis $field:ident: $typ:ty ,)*
    }) => {
        $(#[doc = $x])?
        $visibility struct $ty_name {
            $($(#[doc = $y])?
              $(#[$meta])?
              $visibility_i $field: $typ,)*
        }
    };
    // union types
    ($($x:expr,)? $visibility:vis union $ty_name:ident {
        $(
          $($y:expr,)? // the doc comment if any
          $(#[$meta:meta])? // metadata
          $visibility_i:vis $field:ident: $typ:ty ,)*
    }) => {
        $(#[doc = $x])?
        $visibility struct $ty_name {
            $($(#[doc = $y])?
              $(#[$meta])?
              $visibility_i $field: $typ,)*
        }
    };
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

/// This macro provides a simpler way to test an outer markdown file.
///
/// # Example
///
/// ```
/// extern crate doc_comment;
///
/// // The two next lines are doing exactly the same thing:
/// doc_comment::doc_comment!(include_str!("../README.md"));
/// doc_comment::doctest!("../README.md");
///
/// // If you want to have a name for your tests:
/// doc_comment::doctest!("../README.md", another);
/// # fn main() {}
/// ```
#[cfg(not(feature = "old_macros"))]
#[macro_export]
macro_rules! doctest {
    ($x:expr) => {
        doc_comment::doc_comment!(include_str!($x));
    };
    ($x:expr, $y:ident) => {
        doc_comment::doc_comment!(include_str!($x), mod $y {});
    };
}

/// This macro provides a simpler way to test an outer markdown file.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate doc_comment;
///
/// // The two next lines are doing exactly the same thing:
/// doc_comment!(include_str!("../README.md"));
/// doctest!("../README.md");
///
/// // If you want to have a name for your tests:
/// doctest!("../README.md", another);
/// # fn main() {}
/// ```
#[cfg(feature = "old_macros")]
#[macro_export]
macro_rules! doctest {
    ($x:expr) => {
        doc_comment!(include_str!($x));
    };
    ($x:expr, $y:ident) => {
        doc_comment!(include_str!($x), mod $y {});
    };
}
