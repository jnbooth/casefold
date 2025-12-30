use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use super::as_ref_hashmap::{AsRefHashMap, DefaultHashBuilder};
use super::case_fold_impl::{ascii, unicode};

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

impl<T, S: AsRef<T>> AsRef<T> for CaseFold<S> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<S: AsRef<str>> CaseFold<S> {
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
        self.as_str().into()
    }

    #[inline]
    fn as_unicode(&self) -> &unicode::CaseFold<str> {
        self.as_str().into()
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

impl<S: AsRef<str>> Eq for CaseFold<S> {}

impl<S: AsRef<str>> Hash for CaseFold<S> {
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

impl<S: AsRef<str>> Ord for CaseFold<S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ascii && other.ascii {
            self.as_ascii().cmp(other.as_ascii())
        } else {
            self.as_unicode().cmp(other.as_unicode())
        }
    }
}

impl<S: AsRef<str>> fmt::Debug for CaseFold<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: AsRef<str>> fmt::Display for CaseFold<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: AsRef<str>> Borrow<unicode::CaseFold<str>> for CaseFold<S> {
    #[inline]
    fn borrow(&self) -> &unicode::CaseFold<str> {
        self.as_unicode()
    }
}

pub type CaseFoldMap<K, V, S = DefaultHashBuilder> =
    AsRefHashMap<unicode::CaseFold<str>, CaseFold<K>, V, S>;

pub struct CaseFoldMut<'a, S: AsRef<str>> {
    fold: &'a mut CaseFold<S>,
}

impl<S: AsRef<str>> Deref for CaseFoldMut<'_, S> {
    type Target = S;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.fold.inner
    }
}

impl<S: AsRef<str>> DerefMut for CaseFoldMut<'_, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fold.inner
    }
}

impl<S: AsRef<str>> Drop for CaseFoldMut<'_, S> {
    fn drop(&mut self) {
        self.fold.ascii = self.fold.inner.as_ref().is_ascii();
    }
}
