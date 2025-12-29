use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use super::as_ref_hashmap::AsRefHashMap;
use super::case_fold_impl::{ascii, unicode};

pub enum CaseFold<'a, S: 'a + ToOwned + ?Sized> {
    Ascii(<S as ToOwned>::Owned),
    BorrowedAscii(&'a S),
    Unicode(<S as ToOwned>::Owned),
    BorrowedUnicode(&'a S),
}

impl<'a, S: 'a + ToOwned + ?Sized> Clone for CaseFold<'a, S> {
    fn clone(&self) -> Self {
        match self {
            Self::BorrowedAscii(b) => Self::BorrowedAscii(b),
            Self::Ascii(o) => Self::Ascii(S::to_owned(o.borrow())),
            Self::BorrowedUnicode(b) => Self::BorrowedUnicode(b),
            Self::Unicode(o) => Self::Unicode(S::to_owned(o.borrow())),
        }
    }
}

impl<'a, S: 'a + ToOwned + ?Sized> Deref for CaseFold<'a, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ascii(s) | Self::Unicode(s) => s.borrow(),
            Self::BorrowedAscii(s) | Self::BorrowedUnicode(s) => s,
        }
    }
}

impl<'a, T, S: 'a + ToOwned + AsRef<T> + ?Sized> AsRef<T> for CaseFold<'a, S> {
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}

impl<'a, S: ToOwned + AsRef<str> + ?Sized> CaseFold<'a, S> {
    pub fn borrowed(s: &'a S) -> Self {
        if s.as_ref().is_ascii() {
            Self::BorrowedAscii(s)
        } else {
            Self::BorrowedUnicode(s)
        }
    }
}

impl<S: ?Sized + ToOwned + AsRef<str>> CaseFold<'_, S> {
    pub fn new(s: S::Owned) -> Self {
        if s.borrow().as_ref().is_ascii() {
            Self::Ascii(s)
        } else {
            Self::Unicode(s)
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        (**self).as_ref()
    }

    #[inline]
    fn as_unicode(&self) -> &unicode::CaseFold<str> {
        self.as_str().into()
    }

    #[inline]
    fn as_ascii(&self) -> Option<&ascii::CaseFold<str>> {
        Some(
            match self {
                Self::Ascii(s) => s.borrow(),
                Self::BorrowedAscii(s) => s,
                Self::Unicode(_) | Self::BorrowedUnicode(_) => return None,
            }
            .as_ref()
            .into(),
        )
    }
}

impl<S, Rhs> PartialEq<CaseFold<'_, Rhs>> for CaseFold<'_, S>
where
    S: ?Sized + ToOwned + AsRef<str>,
    Rhs: ?Sized + ToOwned + AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        if let Some(x) = self.as_ascii()
            && let Some(y) = other.as_ascii()
        {
            x == y
        } else {
            self.as_unicode() == other.as_unicode()
        }
    }
}

impl<S: ?Sized + AsRef<str> + ToOwned> Eq for CaseFold<'_, S> {}

impl<S: ?Sized + AsRef<str> + ToOwned> Hash for CaseFold<'_, S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_unicode().hash(hasher);
    }
}

impl<S, Rhs> PartialOrd<CaseFold<'_, Rhs>> for CaseFold<'_, S>
where
    S: ?Sized + AsRef<str> + ToOwned,
    Rhs: ?Sized + AsRef<str> + ToOwned,
{
    #[inline]
    fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<Ordering> {
        if let Some(x) = self.as_ascii()
            && let Some(y) = other.as_ascii()
        {
            x.partial_cmp(y)
        } else {
            self.as_unicode().partial_cmp(other.as_unicode())
        }
    }
}

impl<S: ?Sized + AsRef<str> + ToOwned> Ord for CaseFold<'_, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(x) = self.as_ascii()
            && let Some(y) = other.as_ascii()
        {
            x.cmp(y)
        } else {
            self.as_unicode().cmp(other.as_unicode())
        }
    }
}

impl<S: ?Sized + AsRef<str> + ToOwned> fmt::Debug for CaseFold<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: ?Sized + AsRef<str> + ToOwned> fmt::Display for CaseFold<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: AsRef<str> + ToOwned> Borrow<unicode::CaseFold<str>> for CaseFold<'_, S> {
    fn borrow(&self) -> &unicode::CaseFold<str> {
        self.as_unicode()
    }
}

pub type CaseFoldMap<K, V, S = RandomState> =
    AsRefHashMap<unicode::CaseFold<str>, CaseFold<'static, K>, V, S>;
