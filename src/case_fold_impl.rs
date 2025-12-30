macro_rules! impl_ci {
    ($t:ty) => {
        use core::borrow::Borrow;
        use core::cmp::Ordering;
        use core::ops::{Deref, DerefMut};
        use core::str::FromStr;
        use core::{fmt, ptr};

        use crate::as_ref_hashmap::{AsRefHashMap, DefaultHashBuilder};

        impl<S: ?Sized + AsRef<$t>> Eq for CaseFold<S> {}

        impl<S, Rhs> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
        where
            S: ?Sized + AsRef<$t>,
            Rhs: ?Sized + AsRef<$t>,
        {
            #[inline]
            fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<Ordering> {
                Some(self.caseless_iter().cmp(other.caseless_iter()))
            }
        }

        impl<S: ?Sized + AsRef<$t>> Ord for CaseFold<S> {
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

        impl<S> Deref for CaseFold<S> {
            type Target = S;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<S> DerefMut for CaseFold<S> {
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

        impl<S: ?Sized + AsRef<str>> CaseFold<S> {
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

        impl<S: FromStr> FromStr for CaseFold<S> {
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

        impl<S: AsRef<$t>> AsRef<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self.0.as_ref().into()
            }
        }

        impl<S: AsRef<$t>> AsRef<$t> for CaseFold<S> {
            #[inline]
            fn as_ref(&self) -> &$t {
                self.0.as_ref()
            }
        }

        impl<'a, S: ?Sized + AsRef<str>> fmt::Display for CaseFold<S> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.as_ref().fmt(f)
            }
        }

        pub type CaseFoldMap<K, V, S = DefaultHashBuilder> =
            AsRefHashMap<CaseFold<$t>, CaseFold<K>, V, S>;
    };
}

pub mod ascii {
    use core::hash::{Hash, Hasher};
    use core::{iter, slice};

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

    #[cfg(feature = "std")]
    impl ToOwned for CaseFold<[u8]> {
        type Owned = CaseFold<Vec<u8>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            CaseFold(self.0.to_owned())
        }
    }

    #[cfg(feature = "std")]
    impl Borrow<CaseFold<str>> for CaseFold<String> {
        #[inline]
        fn borrow(&self) -> &CaseFold<str> {
            CaseFold::borrow(&self.0)
        }
    }

    #[cfg(feature = "std")]
    impl ToOwned for CaseFold<str> {
        type Owned = CaseFold<String>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            CaseFold(self.0.to_owned())
        }
    }

    impl<S: ?Sized + AsRef<[u8]>> CaseFold<S> {
        #[inline]
        fn caseless_iter(&self) -> iter::Map<slice::Iter<'_, u8>, fn(&u8) -> u8> {
            self.0.as_ref().iter().map(u8::to_ascii_lowercase)
        }
    }

    impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
    where
        S: ?Sized + AsRef<[u8]>,
        Rhs: ?Sized + AsRef<[u8]>,
    {
        #[inline]
        fn eq(&self, other: &CaseFold<Rhs>) -> bool {
            self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
        }
    }

    impl<S: ?Sized + AsRef<[u8]>> Hash for CaseFold<S> {
        #[inline]
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            for byte in self.caseless_iter() {
                hasher.write_u8(byte);
            }
            hasher.write_u8(0xff);
        }
    }

    impl AsRef<CaseFold<[u8]>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().into()
        }
    }

    #[cfg(feature = "std")]
    impl AsRef<CaseFold<[u8]>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().into()
        }
    }

    impl_ci!([u8]);
}

pub mod unicode {
    use core::char::ToLowercase;
    use core::hash::{Hash, Hasher};
    use core::iter;
    use core::str::Chars;

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

    #[cfg(feature = "std")]
    impl ToOwned for CaseFold<str> {
        type Owned = CaseFold<String>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            CaseFold(self.0.to_owned())
        }
    }

    impl<S: ?Sized + AsRef<str>> CaseFold<S> {
        #[inline]
        fn caseless_iter(&self) -> iter::FlatMap<Chars<'_>, ToLowercase, fn(char) -> ToLowercase> {
            self.0.as_ref().chars().flat_map(char::to_lowercase)
        }
    }

    impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
    where
        S: ?Sized + AsRef<str>,
        Rhs: ?Sized + AsRef<str>,
    {
        #[inline]
        fn eq(&self, other: &CaseFold<Rhs>) -> bool {
            self.caseless_iter().eq(other.caseless_iter())
        }
    }

    impl<S: ?Sized + AsRef<str>> Hash for CaseFold<S> {
        #[inline]
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            let mut buf = [0; 4];
            for c in self.caseless_iter() {
                for &byte in c.encode_utf8(&mut buf).as_bytes() {
                    hasher.write_u8(byte);
                }
            }
            hasher.write_u8(0xff);
        }
    }

    impl AsRef<CaseFold<str>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.into()
        }
    }

    #[cfg(feature = "std")]
    impl AsRef<CaseFold<str>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.as_str().into()
        }
    }

    impl_ci!(str);
}
