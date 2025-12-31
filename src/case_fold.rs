use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use crate::{ascii, unicode};

#[derive(Copy, Clone)]
pub struct CaseFold<S> {
    inner: S,
    ascii: bool,
}

impl<S> Deref for CaseFold<S> {
    type Target = S;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S> DerefMut for CaseFold<S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, S> AsRef<T> for CaseFold<S>
where
    S: AsRef<T>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<S> CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    pub fn new(s: S) -> Self {
        Self {
            ascii: s.as_ref().is_ascii(),
            inner: s,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> CaseFoldMut<'_, S> {
        self.ascii = false; // because destructors are not guaranteed to run
        CaseFoldMut { fold: self }
    }

    #[inline]
    fn as_str(&self) -> &str {
        self.inner.as_ref()
    }

    #[inline]
    fn as_ascii(&self) -> &ascii::CaseFold<str> {
        ascii::CaseFold::borrow(self.inner.as_ref())
    }

    #[inline]
    fn as_unicode(&self) -> &unicode::CaseFold<str> {
        unicode::CaseFold::borrow(self.inner.as_ref())
    }
}

impl<'a> CaseFold<&'a str> {
    #[inline]
    pub const fn const_new(s: &'a str) -> Self {
        Self {
            ascii: s.is_ascii(),
            inner: s,
        }
    }
}

impl CaseFold<String> {
    #[inline]
    pub const fn const_new(s: String) -> Self {
        Self {
            ascii: s.as_str().is_ascii(),
            inner: s,
        }
    }
}

impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
where
    S: AsRef<str>,
    Rhs: AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        if self.ascii && other.ascii {
            self.as_ascii() == other.as_ascii()
        } else {
            self.as_unicode() == other.as_unicode()
        }
    }
}

impl<S> Eq for CaseFold<S> where S: AsRef<str> {}

impl<S> Hash for CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        if self.ascii {
            for byte in self.inner.as_ref().as_bytes() {
                hasher.write_u8(byte.to_ascii_uppercase());
            }
            hasher.write_u8(0xff);
        } else {
            self.as_unicode().hash(hasher);
        }
    }
}

impl<S, Rhs> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
where
    S: AsRef<str>,
    Rhs: AsRef<str>,
{
    #[inline]
    fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<Ordering> {
        if self.ascii && other.ascii {
            self.as_ascii().partial_cmp(other.as_ascii())
        } else {
            self.as_unicode().partial_cmp(other.as_unicode())
        }
    }
}

impl<S> Ord for CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ascii && other.ascii {
            self.as_ascii().cmp(other.as_ascii())
        } else {
            self.as_unicode().cmp(other.as_unicode())
        }
    }
}

impl<S> fmt::Debug for CaseFold<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S> fmt::Display for CaseFold<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S> Borrow<unicode::CaseFold<str>> for CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn borrow(&self) -> &unicode::CaseFold<str> {
        self.as_unicode()
    }
}

#[cfg(any(feature = "hashbrown", feature = "std"))]
pub type CaseFoldMap<K, V, S = crate::as_ref_hashmap::DefaultHashBuilder> =
    crate::as_ref_hashmap::AsRefHashMap<unicode::CaseFold<str>, CaseFold<K>, V, S>;

pub struct CaseFoldMut<'a, S>
where
    S: AsRef<str>,
{
    fold: &'a mut CaseFold<S>,
}

impl<S> Deref for CaseFoldMut<'_, S>
where
    S: AsRef<str>,
{
    type Target = S;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.fold.inner
    }
}

impl<S> DerefMut for CaseFoldMut<'_, S>
where
    S: AsRef<str>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fold.inner
    }
}

impl<S> Drop for CaseFoldMut<'_, S>
where
    S: AsRef<str>,
{
    fn drop(&mut self) {
        self.fold.ascii = self.fold.inner.as_ref().is_ascii();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::hashed;

    #[test]
    fn eq() {
        assert_eq!(CaseFold::new("Maße"), CaseFold::new("MASSE"));
        assert_eq!(CaseFold::new("στιγμας"), CaseFold::new("στιγμασ"));
        assert_ne!(CaseFold::new("στιγμας"), CaseFold::new("στιγμαα"));
    }

    #[test]
    fn cmp() {
        assert!(CaseFold::new("a") < CaseFold::new("B"));
        assert!(CaseFold::new("A") < CaseFold::new("b"));
    }

    #[test]
    fn hash() {
        assert_eq!(
            hashed(&CaseFold::new("Maße")),
            hashed(&CaseFold::new("MASSE"))
        );
        assert_ne!(
            hashed(&CaseFold::new("Maße")),
            hashed(&CaseFold::new("MASE"))
        );
    }

    #[test]
    fn prefix_free() {
        assert_ne!(
            hashed(&(CaseFold::new("foo"), CaseFold::new("bar"))),
            hashed(&(CaseFold::new("foob"), CaseFold::new("ar")))
        );
    }

    #[test]
    fn is_ascii() {
        assert!(CaseFold::new("fOObaR").ascii);
        assert!(!CaseFold::new("Maße").ascii);
    }

    #[test]
    fn hash_across_ascii() {
        assert_eq!(
            hashed(&CaseFold::new("fOObaR")),
            hashed(&unicode::CaseFold::new("fOObar"))
        );
    }

    #[test]
    fn hash_across_unicode() {
        assert_eq!(
            hashed(&CaseFold::new("Maße")),
            hashed(&unicode::CaseFold::new("MASSE"))
        );
    }
}
