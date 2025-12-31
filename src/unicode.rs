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
        let mut buf = [0; 4];
        for c in self.caseless_iter() {
            for &byte in c.encode_utf8(&mut buf).as_bytes() {
                hasher.write_u8(byte);
            }
        }
        hasher.write_u8(0xff);
    }
}

crate::impl_casefold!(str);

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
}
