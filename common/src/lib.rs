//! Common utilities and re-exports for Advent of Code 2025
//!
//! Just add `use common::prelude::*;` at the top of your solution!

pub mod prelude {
    // Re-export commonly used crates
    pub use itertools::Itertools;
    pub use nom;
    pub use num;

    // Re-export standard library items you'll use constantly
    pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
    pub use std::str::FromStr;
}
