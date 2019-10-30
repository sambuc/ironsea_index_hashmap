#[macro_use]
extern crate serde_derive;

mod destructured;
mod full_record;

pub use destructured::Index as IndexDestructured;
pub use full_record::Index;
