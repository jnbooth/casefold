use core::char::ToUppercase;
use core::hash::{Hash, Hasher};
use core::iter;
use core::str::Chars;

#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct CaseFold<S: ?Sized>(S);

impl<S: ?Sized> CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    pub fn caseless_iter(&self) -> iter::FlatMap<Chars<'_>, ToUppercase, fn(char) -> ToUppercase> {
        self.0.as_ref().chars().flat_map(char::to_uppercase)
    }
}

impl<S: ?Sized, Rhs: ?Sized> PartialEq<CaseFold<Rhs>> for CaseFold<S>
where
    S: AsRef<str>,
    Rhs: AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        self.caseless_iter().eq(other.caseless_iter())
    }
}

impl<S: ?Sized> Hash for CaseFold<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        use crate::HASH_BUF_SIZE as N;

        let mut buf = [0; N + 4];
        let mut i = 0;
        for c in self.caseless_iter() {
            i += c.encode_utf8(&mut buf[i..]).len();
            if i >= N {
                hasher.write(&buf[..i]);
                i = 0;
            }
        }
        if i != 0 {
            hasher.write(&buf[..i]);
        }
        hasher.write_u8(0xff);
    }
}

crate::impl_casefold!(str);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockHasher, hash};
    const ENCODE: &str =
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";

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
    fn encode() {
        let mut hasher = MockHasher::default();
        CaseFold::borrow(ENCODE).hash(&mut hasher);
        assert_eq!(hasher.as_str(), ENCODE.to_ascii_uppercase());
    }
}
