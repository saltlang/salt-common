use core::clone::Clone;
use std::convert::AsRef;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;

/// A distinguished type for symbols.  These are implemented as
/// references to interned strings, making comparison very easy.
#[derive(Copy, Eq, Ord)]
pub struct Symbol<'a>(&'a str);

/// Context for creating Symbols.
pub trait SymbolCtx<'a> {
    /// Convert `fname` into a corresponding `Symbol`.
    fn symbol(&mut self, fname: &'a str) -> Symbol<'a>;
}

impl<'a> Clone for Symbol<'a> {
    fn clone(&self) -> Symbol<'a> {
        Symbol(self.0)
    }
}

impl<'a> Hash for Symbol<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(((self.0 as *const _) as *const u8) as usize);
    }
}

impl<'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Symbol<'a>) -> bool {
        self.0 as *const _ == other.0 as *const _
    }
}

impl<'a> PartialOrd for Symbol<'a> {
    fn partial_cmp(&self, other: &Symbol<'a>) -> Option<Ordering> {
        let a = ((self.0 as *const _) as *const u8) as usize;
        let b = ((other.0 as *const _) as *const u8) as usize;

        Some(a.cmp(&b))
    }
}

impl<'a> Display for Symbol<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Debug for Symbol<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> AsRef<Path> for Symbol<'a> {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

#[test]
fn test_ref_equality_mismatch() {
    let a = "helloa".split_at(5).0;
    let b = "hellob".split_at(5).0;
    let fa = Symbol(a);
    let fb = Symbol(b);

    assert_ne!(fa, fb)
}

#[test]
fn test_ref_equality_same() {
    let a = "hello";
    let fa = Symbol(a);
    let fb = Symbol(a);

    assert_eq!(fa, fb)
}
