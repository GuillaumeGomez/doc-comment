//
// Doc comment
//
// Copyright (c) 2018 Guillaume Gomez
//

#![no_std]

//! The point of this (small) crate is to allow you to add doc comments from macros.
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

#[macro_export]
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}
