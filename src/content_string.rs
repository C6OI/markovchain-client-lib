use serde::{Serialize, Serializer};
use thiserror::Error;

/// A string that is guaranteed to have length between 1 and 2000 (inclusive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentString(String);

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ContentStringError {
    #[error("Text is empty")]
    Empty,
    #[error("Text is too long: {length} characters (max {max})")]
    TooLong { length: usize, max: usize },
}

impl ContentString {
    const MAX_LEN: usize = 2000;

    /// Try to construct a `ContentString` from a `String`, validating its length.
    ///
    /// # Errors
    /// Will return `Err` if the string is empty or too long
    pub fn new(s: String) -> Result<Self, ContentStringError> {
        let len = s.len();

        if len == 0 {
            Err(ContentStringError::Empty)
        } else if len > Self::MAX_LEN {
            Err(ContentStringError::TooLong { length: len, max: Self::MAX_LEN })
        } else {
            Ok(Self(s))
        }
    }

    #[must_use] 
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use] 
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for ContentString {
    type Error = ContentStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ContentString {
    type Error = ContentStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl Serialize for ContentString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let error = ContentString::new(String::new()).unwrap_err();
        assert_eq!(error, ContentStringError::Empty);
    }

    #[test]
    fn too_long() {
        let error = ContentString::new("x".repeat(2500)).unwrap_err();
        assert_eq!(error, ContentStringError::TooLong { length: 2500, max: ContentString::MAX_LEN });
    }

    #[test]
    fn normal() {
        let content_string = ContentString::new("Just a normal string".into()).unwrap();
        assert_eq!(content_string.0, "Just a normal string");
    }
}
