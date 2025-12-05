use std::fmt::{self, Display};

pub mod template;

// Use this file to add helper functions and additional modules.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate<T> {
    pub left: T,
    pub top: T,
}

impl<T: Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.top)
    }
}
