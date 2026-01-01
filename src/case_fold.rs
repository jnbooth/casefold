use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

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
            self.as_ascii().hash(hasher);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::hash;

    const ENCODE_ASCII: &str =
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";

    const ENCODE_UTF8: &str =
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyß";

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
    fn hash_eq() {
        assert_eq!(hash(&CaseFold::new("Maße")), hash(&CaseFold::new("MASSE")));
        assert_ne!(hash(&CaseFold::new("Maße")), hash(&CaseFold::new("MASE")));
    }

    #[test]
    fn hash_prefix_free() {
        assert_ne!(
            hash(&(CaseFold::new("foo"), CaseFold::new("bar"))),
            hash(&(CaseFold::new("foob"), CaseFold::new("ar")))
        );
    }

    #[test]
    fn is_ascii() {
        assert!(CaseFold::new("fOObaR").ascii);
        assert!(!CaseFold::new("Maße").ascii);
    }

    #[test]
    fn hash_ascii_consistency() {
        assert_eq!(
            hash(&ascii::CaseFold::new(ENCODE_ASCII)),
            hash(&unicode::CaseFold::new(ENCODE_ASCII))
        );
    }

    #[test]
    fn hash_across_ascii() {
        assert_eq!(
            hash(&CaseFold::new(ENCODE_ASCII)),
            hash(&ascii::CaseFold::new(ENCODE_ASCII))
        );
    }

    #[test]
    fn hash_across_unicode() {
        assert_eq!(
            hash(&CaseFold::new(ENCODE_UTF8)),
            hash(&unicode::CaseFold::new(ENCODE_UTF8))
        );
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash_ascii(b: &mut test::Bencher) {
        let s = CaseFold::new(ENCODE_ASCII);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash_ascii_unicase(b: &mut test::Bencher) {
        let s = unicase::UniCase::new(ENCODE_ASCII);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash_unicode(b: &mut test::Bencher) {
        let s = CaseFold::new(ENCODE_UTF8);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash_unicode_unicase(b: &mut test::Bencher) {
        let s = unicase::UniCase::new(ENCODE_UTF8);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }
}
