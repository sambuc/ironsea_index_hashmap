#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Iron Sea - Index Hash Map
//!
//! A simple hash map index for the Iron Sea toolkit, based on
//! [`std`]`::`[`collections`]`::`[`HashMap`].
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html
//! [`collections`]: https://doc.rust-lang.org/std/collections/index.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//!
//! ## Iron Sea: Database Toolkit
//! **Iron Sea** provides a set of database engine bricks, which can be
//! combined and applied on arbitrary data structures.
//!
//! Unlike a traditional database, it does not assume a specific
//! physical structure for the tables nor the records, but relies on the
//! developer to provide a set of extractor functions which are used by
//! the specific indices provided.
//!
//! This enables the index implementations to be agnostic from the
//! underlying data structure, and re-used.
//!

mod destructured;
mod full_record;

pub use destructured::Index as IndexDestructured;
pub use full_record::Index;
