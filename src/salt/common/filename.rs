use core::clone::Clone;
use std::convert::AsRef;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::fmt::Result;
use std::path::Path;

/// A distinguished type for filenames.  These are implemented as
/// references to interned strings, making comparison very easy.
#[derive(Copy, Eq, Ord)]
pub struct Filename<'a>(&'a str);

/// Context for creating Filenames.
pub trait FilenameCtx<'a> {
    /// Convert `fname` into a corresponding `Filename`.
    fn filename(&mut self, fname: &'a str) -> Filename<'a>;
}

impl<'a> Clone for Filename<'a> {
    fn clone(&self) -> Filename<'a> {
        Filename(self.0)
    }
}

impl<'a> Hash for Filename<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(((self.0 as *const _) as *const u8) as usize);
    }
}

impl<'a> PartialEq for Filename<'a> {
    fn eq(&self, other: &Filename<'a>) -> bool {
        self.0 as *const _ == other.0 as *const _
    }
}

impl<'a> PartialOrd for Filename<'a> {
    fn partial_cmp(&self, other: &Filename<'a>) -> Option<Ordering> {
        let a = ((self.0 as *const _) as *const u8) as usize;
        let b = ((other.0 as *const _) as *const u8) as usize;

        Some(a.cmp(&b))
    }
}

impl<'a> Display for Filename<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Debug for Filename<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.0 as *const _)
    }
}

impl<'a> AsRef<Path> for Filename<'a> {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

#[test]
fn test_ref_equality_mismatch() {
    let a = "helloa".split_at(5).0;
    let b = "hellob".split_at(5).0;
    let fa = Filename(a);
    let fb = Filename(b);

    assert_ne!(fa, fb)
}

#[test]
fn test_ref_equality_same() {
    let a = "hello";
    let fa = Filename(a);
    let fb = Filename(a);

    assert_eq!(fa, fb)
}
