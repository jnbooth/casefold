use core::hash::{Hash, Hasher};
use core::{iter, slice};
#[cfg(feature = "std")]
use std::borrow::Borrow;

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct CaseFold<S: ?Sized>(S);

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

impl<S> CaseFold<S>
where
    S: ?Sized,
{
    /// Convert a `[u8]` or `str` reference into a `CaseFold` reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use casefold::ascii::CaseFold;
    ///
    /// let s: &CaseFold<str> = CaseFold::borrow("s");
    /// ```
    #[inline]
    pub const fn borrow(s: &S) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(core::ptr::from_ref::<S>(s) as *const Self) }
    }
}

impl<S> CaseFold<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    #[inline]
    fn caseless_iter(&self) -> iter::Map<slice::Iter<'_, u8>, fn(&u8) -> u8> {
        self.0.as_ref().iter().map(u8::to_ascii_uppercase)
    }
}

impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
where
    S: AsRef<[u8]> + ?Sized,
    Rhs: AsRef<[u8]> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
    }
}

impl<S> Hash for CaseFold<S>
where
    S: AsRef<[u8]> + ?Sized,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        use crate::HASH_BUF_SIZE as N;

        let mut buf = [0; N];
        let mut i = 0;
        for byte in self.as_ref() {
            buf[i] = byte.to_ascii_uppercase();
            i += 1;
            if i == N {
                hasher.write(&buf);
                i = 0;
            }
        }
        if i != 0 {
            hasher.write(&buf[..i]);
        }
        hasher.write_u8(0xff);
    }
}

crate::impl_casefold!([u8]);

#[cfg(any(feature = "hashbrown", feature = "std"))]
/// Case-insensitive wrapper around a [`HashMap`](crate::map::HashMap), using ASCII case folding.
#[repr(transparent)]
pub struct CaseFoldMap<K, V, S = crate::map::DefaultHashBuilder>(
    crate::map::HashMap<CaseFold<K>, V, S>,
);

#[cfg(any(feature = "hashbrown", feature = "std"))]
crate::impl_casefoldmap!([u8]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockHasher, hash};
    const ENCODE: &str =
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";

    #[test]
    fn eq() {
        assert_eq!(CaseFold::new("fOObAr"), CaseFold::new("FOOBAR"));
        assert_eq!(CaseFold::new("fOObAr"), CaseFold::new("foobar"));
        assert_ne!(CaseFold::new("fOObAr"), CaseFold::new("fOObAra"));
    }

    #[test]
    fn cmp() {
        assert!(CaseFold::new("a") < CaseFold::new("B"));
        assert!(CaseFold::new("A") < CaseFold::new("b"));
    }

    #[test]
    fn hash_eq() {
        assert_eq!(
            hash(&CaseFold::new("fOObAr")),
            hash(&CaseFold::new("FOOBAR"))
        );
        assert_ne!(
            hash(&CaseFold::new("fOObAr")),
            hash(&CaseFold::new("fOObAa"))
        );
    }

    #[test]
    fn hash_prefix_free() {
        assert_ne!(
            hash(&(CaseFold::new("foo"), CaseFold::new("bar"))),
            hash(&(CaseFold::new("foob"), CaseFold::new("ar")))
        );
    }

    #[test]
    fn encode() {
        let mut hasher = MockHasher::default();
        CaseFold::new(ENCODE).hash(&mut hasher);
        assert_eq!(hasher.as_str(), ENCODE.to_ascii_uppercase());
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash(b: &mut test::Bencher) {
        let s = CaseFold::new(ENCODE);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_hash_unicase(b: &mut test::Bencher) {
        let s = unicase::Ascii::new(ENCODE);
        let mut hasher = std::hash::DefaultHasher::new();
        b.iter(|| test::black_box(s).hash(&mut hasher));
    }
}
