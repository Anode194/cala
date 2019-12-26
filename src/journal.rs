//! API for printing out to journal.  Enable with the `journal` feature.
//!
//! # Usage
//! ```rust
//! use cala::journal;
//! 
//! journal::fix!("FIXME: {}", 12);
//! journal::dev!("Test: {}", 40);
//! journal::out!("Result: {}", 4.4);
//! ```

pub use devout::dev;
pub use devout::fix;
pub use devout::out;
