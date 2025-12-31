#[macro_export]
macro_rules! impl_casefold {
    ($t:ty) => {
        use core::borrow::Borrow;
        use core::cmp::Ordering;
        use core::ops::{Deref, DerefMut};
        use core::str::FromStr;
        use core::{fmt, ptr};

        impl<S: ?Sized> Eq for CaseFold<S> where S: AsRef<$t> {}

        impl<S: ?Sized, Rhs: ?Sized> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
        where
            S: AsRef<$t>,
            Rhs: AsRef<$t>,
        {
            #[inline]
            fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<Ordering> {
                Some(self.caseless_iter().cmp(other.caseless_iter()))
            }
        }

        impl<S: ?Sized> Ord for CaseFold<S>
        where
            S: AsRef<$t>,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.caseless_iter().cmp(other.caseless_iter())
            }
        }

        impl<S> CaseFold<S> {
            #[inline]
            pub const fn new(s: S) -> Self {
                Self(s)
            }

            #[inline]
            pub fn into_inner(self) -> S {
                self.0
            }
        }

        impl<S: ?Sized> CaseFold<S> {
            #[inline]
            pub const fn borrow(s: &S) -> &Self {
                // SAFETY: #[repr(transparent)]
                unsafe { &*(ptr::from_ref::<S>(s) as *const Self) }
            }
        }

        impl<S: ?Sized> Deref for CaseFold<S> {
            type Target = S;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<S: ?Sized> DerefMut for CaseFold<S> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<'a, S: ?Sized> From<&CaseFold<&'a S>> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &CaseFold<&'a S>) -> Self {
                value.0.into()
            }
        }

        impl<S: ?Sized> CaseFold<S>
        where
            S: AsRef<str>,
        {
            #[inline]
            pub fn as_str(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl<S> From<S> for CaseFold<S> {
            #[inline]
            fn from(value: S) -> Self {
                CaseFold::new(value)
            }
        }

        impl<S> FromStr for CaseFold<S>
        where
            S: FromStr,
        {
            type Err = <S as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse().map(CaseFold)
            }
        }

        impl<'a, S: ?Sized> From<&'a S> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &'a S) -> Self {
                CaseFold::borrow(value)
            }
        }

        impl<S: AsRef<$t>> Borrow<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn borrow(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.0.as_ref())
            }
        }

        impl AsRef<CaseFold<$t>> for CaseFold<$t> {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self
            }
        }

        impl<S> AsRef<CaseFold<$t>> for CaseFold<S>
        where
            S: AsRef<$t>,
        {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self.0.as_ref().into()
            }
        }

        impl<S: ?Sized> AsRef<$t> for CaseFold<S>
        where
            S: AsRef<$t>,
        {
            #[inline]
            fn as_ref(&self) -> &$t {
                self.0.as_ref()
            }
        }

        impl<'a, S: ?Sized> fmt::Display for CaseFold<S>
        where
            S: AsRef<str>,
        {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.as_ref().fmt(f)
            }
        }

        impl AsRef<CaseFold<$t>> for str {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.as_ref())
            }
        }

        impl AsRef<CaseFold<$t>> for String {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.as_ref())
            }
        }

        #[cfg(feature = "std")]
        impl ToOwned for CaseFold<$t> {
            type Owned = CaseFold<<$t as ToOwned>::Owned>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                CaseFold(self.0.to_owned())
            }
        }

        #[cfg(any(feature = "hashbrown", feature = "std"))]
        pub type CaseFoldMap<K, V, S = $crate::as_ref_hashmap::DefaultHashBuilder> =
            $crate::as_ref_hashmap::AsRefHashMap<CaseFold<$t>, CaseFold<K>, V, S>;
    };
}
