use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "Vec<T>")]
pub struct NonEmptyVec<T>(Vec<T>);
impl<T> NonEmptyVec<T> {
    pub fn new(value: Vec<T>) -> Option<Self> {
        Self::try_from(value).ok()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
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
    type Error = String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("sequence must contain at least one element".to_string())
        } else {
            Ok(Self(value))
        }
    }
}
impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
