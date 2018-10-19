use std::convert::AsRef;
use std::convert::From;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::path::Path;

/// A distinguished type for filenames
#[derive(Hash, Ord, Eq)]
pub struct Filename<'a>(&'a str);

impl<'a> PartialEq for Filename<'a> {
    fn eq(&self, other: &Filename<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialOrd for Filename<'a> {
    fn partial_cmp(&self, other: &Filename<'a>) -> Option<Ordering> {
        Some(self.0.cmp(other.0))
    }
}

impl<'a> From<&'a String> for Filename<'a> {
    /// Create a Filename from a String
    fn from(str: &'a String) -> Filename {
        Filename(str.as_str())
    }
}

impl<'a> From<&'a str> for Filename<'a> {
    /// Create a Filename from a String
    fn from(str: &'a str) -> Filename {
        Filename(str)
    }
}

impl<'a> Display for Filename<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Debug for Filename<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> AsRef<Path> for Filename<'a> {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}
