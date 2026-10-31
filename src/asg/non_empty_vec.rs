use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "Vec<T>")]
pub struct NonEmptyVec<T>(Vec<T>);
impl<T> NonEmptyVec<T> {
    pub fn new(first: T) -> Self {
        Self(vec![first])
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }
}
impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = EmptySequence;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptySequence)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug)]
pub struct EmptySequence;
impl fmt::Display for EmptySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("sequence must contain at least one element")
    }
}
impl Error for EmptySequence {}
