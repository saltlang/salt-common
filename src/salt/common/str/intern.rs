use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::Values;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::ExactSizeIterator;
use std::iter::FusedIterator;
use std::iter::Iterator;

/// String interning table.
pub struct StrIntern<'s>(HashMap<&'s str, &'s str>);

/// Iterator over strings in a `StrIntern` table.
pub struct Strings<'s>(Values<'s, &'s str, &'s str>);

impl<'s> Clone for Strings<'s> {
    fn clone(&self) -> Strings<'s> {
        Strings(self.0.clone())
    }
}

impl<'s> Debug for Strings<'s> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_list()
            .entries(self.0.clone())
            .finish()
    }
}

impl<'s> Iterator for Strings<'s> {
    type Item = &'s str;

    #[inline]
    fn next(&mut self) -> Option<&'s str> {
        self.0.next().map(|outref: &'s &'s str| *outref )
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'s> ExactSizeIterator for Strings<'s> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'s> FusedIterator for Strings<'s> {}

impl<'s> StrIntern<'s> {
    /// Create a `StrIntern` table.
    pub fn new() -> StrIntern<'s> {
        StrIntern(HashMap::new())
    }

    /// Create a `StrIntern` table with a given capacity.
    pub fn with_capacity(capacity: usize) -> StrIntern<'s> {
        StrIntern(HashMap::with_capacity(capacity))
    }

    /// Intern a given string.  This will return a distinguished
    /// reference to a string equal to str, relative to this StrIntern
    /// structure.
    pub fn intern(&mut self, str: &'s str) -> &'s str {
        match self.0.entry(str) {
            Entry::Occupied(ent) => ent.get(),
            Entry::Vacant(ent) => ent.insert(str)
        }
    }

    /// Reserves capacity for at least additional more elements.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Shrinks the capacity of the underlying map as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    /// Get an iterator for the strings in the table.
    pub fn strings(&self) -> Strings {
        Strings(self.0.values())
    }

    /// Get the number of strings in the table.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn test_ref_equality_match() {
    let a = "hello";
    let refa = &a as *const _;

    assert_eq!(refa, refa)
}

#[test]
fn test_ref_equality_mismatch() {
    let a = "helloa".split_at(5).0;
    let b = "hellob".split_at(5).0;
    let refa = &a as *const _;
    let refb = &b as *const _;

    assert_ne!(refa, refb)
}

#[test]
fn test_str_equality_mismatch() {
    let a = "helloa".split_at(5).0;
    let b = "hellob".split_at(5).0;
    let refa = &a;
    let refb = &b;

    assert_eq!(refa, refb)
}

#[test]
fn test_str_intern_same() {
    let refa = "helloa".split_at(5).0;
    let refb = "hellob".split_at(5).0;
    let mut tab = StrIntern::new();
    let interna = tab.intern(refa);
    let internb = tab.intern(refb);

    assert_eq!(interna, internb)
}

#[test]
fn test_str_intern_different() {
    let a = "hello";
    let b = "bye";
    let refa = &a;
    let refb = &b;
    let mut tab = StrIntern::new();
    let interna = tab.intern(refa);
    let internb = tab.intern(refb);

    assert_ne!(interna, internb)
}
