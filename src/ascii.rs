use core::hash::{Hash, Hasher};
use core::{iter, slice};

#[derive(Copy, Clone, Debug, Default)]
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

impl<S: ?Sized> CaseFold<S>
where
    S: AsRef<[u8]>,
{
    #[inline]
    fn caseless_iter(&self) -> iter::Map<slice::Iter<'_, u8>, fn(&u8) -> u8> {
        self.0.as_ref().iter().map(u8::to_ascii_lowercase)
    }
}

impl<S: ?Sized, Rhs: ?Sized> PartialEq<CaseFold<Rhs>> for CaseFold<S>
where
    S: AsRef<[u8]>,
    Rhs: AsRef<[u8]>,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
    }
}

impl<S: ?Sized> Hash for CaseFold<S>
where
    S: AsRef<[u8]>,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let mut iter = EncodeIter {
            inner: self.0.as_ref().iter(),
            buf: [0; 16],
        };
        while let encoded = iter.next()
            && !encoded.is_empty()
        {
            hasher.write(encoded);
        }
        hasher.write_u8(0xff);
    }
}

struct EncodeIter<'a, const N: usize> {
    inner: slice::Iter<'a, u8>,
    buf: [u8; N],
}

impl<const N: usize> EncodeIter<'_, N> {
    pub fn next(&mut self) -> &[u8] {
        let mut i = 0;
        for byte in &mut self.inner {
            self.buf[i] = byte.to_ascii_lowercase();
            if i == N - 1 {
                break;
            }
            i += 1;
        }
        &self.buf[..i]
    }
}

crate::impl_casefold!([u8]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::hashed;

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
    fn hash() {
        assert_eq!(
            hashed(&CaseFold::new("fOObAr")),
            hashed(&CaseFold::new("FOOBAR"))
        );
        assert_ne!(
            hashed(&CaseFold::new("fOObAr")),
            hashed(&CaseFold::new("fOObAa"))
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
    fn encode() {
        let a = "fOObAr";
        let mut buf = Vec::new();
        let mut iter = EncodeIter {
            inner: a.as_bytes().iter(),
            buf: [0; 16],
        };
        while let encoded = iter.next()
            && !encoded.is_empty()
        {
            buf.extend_from_slice(encoded);
        }
        assert_eq!(str::from_utf8(&buf).unwrap(), "foobar");
    }
}
