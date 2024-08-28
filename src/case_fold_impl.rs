use super::as_ref_hashmap::AsRefHashMap;

macro_rules! impl_ci {
    ($t:ty) => {
        impl<S: ?Sized + AsRef<$t>> Eq for CaseFold<S> {}

        impl<S, Rhs> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
        where
            S: ?Sized + AsRef<$t>,
            Rhs: ?Sized + AsRef<$t>,
        {
            #[inline]
            fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<std::cmp::Ordering> {
                Some(self.caseless_iter().cmp(other.caseless_iter()))
            }
        }

        impl<S: ?Sized + AsRef<$t>> Ord for CaseFold<S> {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.caseless_iter().cmp(other.caseless_iter())
            }
        }

        impl<S> CaseFold<S> {
            pub const fn new(s: S) -> Self {
                Self(s)
            }

            pub fn unfold(self) -> S {
                self.0
            }
        }

        impl<S: ?Sized> CaseFold<S> {
            pub const fn borrow(s: &S) -> &Self {
                // SAFETY: #[repr(transparent)]
                unsafe { &*(s as *const S as *const Self) }
            }
        }

        impl<'a, S: ?Sized> From<&CaseFold<&'a S>> for &'a CaseFold<S> {
            fn from(value: &CaseFold<&'a S>) -> Self {
                value.0.into()
            }
        }

        impl<S: ?Sized + AsRef<str>> CaseFold<S> {
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

        impl<'a, S: ?Sized> From<&'a S> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &'a S) -> Self {
                CaseFold::borrow(value)
            }
        }

        impl<S: AsRef<$t>> std::borrow::Borrow<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn borrow(&self) -> &CaseFold<$t> {
                self.0.as_ref().into()
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

        impl<'a, S: ?Sized + AsRef<str>> Display for CaseFold<S> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.0.as_ref().fmt(f)
            }
        }

        pub type CaseFoldMap<K, V, S = RandomState> =
            super::AsRefHashMap<CaseFold<$t>, CaseFold<K>, V, S>;
    };
}

pub mod ascii {
    use std::collections::hash_map::RandomState;
    use std::fmt::{self, Display, Formatter};
    use std::hash::{Hash, Hasher};
    use std::{iter, slice};

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

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
        }
    }

    impl AsRef<CaseFold<[u8]>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().into()
        }
    }

    impl AsRef<CaseFold<[u8]>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().into()
        }
    }

    impl_ci!([u8]);
}

pub mod unicode {
    use std::char::ToLowercase;
    use std::collections::hash_map::RandomState;
    use std::fmt::{self, Display, Formatter};
    use std::hash::{Hash, Hasher};
    use std::iter;
    use std::str::Chars;

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

    impl<S: ?Sized + AsRef<str>> CaseFold<S> {
        #[inline]
        fn caseless_iter(&self) -> iter::FlatMap<Chars, ToLowercase, fn(char) -> ToLowercase> {
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
            for c in self.caseless_iter() {
                hasher.write_u32(u32::from(c));
            }
        }
    }

    impl AsRef<CaseFold<str>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.into()
        }
    }

    impl AsRef<CaseFold<str>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.as_str().into()
        }
    }

    impl_ci!(str);
}
