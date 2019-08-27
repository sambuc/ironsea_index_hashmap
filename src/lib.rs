#[macro_use]
extern crate serde_derive;

mod borrowed;
mod owned;

pub use ironsea_table::Table;

pub use borrowed::Index;
pub use owned::IndexOwned;
