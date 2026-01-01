#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

const HASH_BUF_SIZE: usize = 16;

#[cfg(any(feature = "hashbrown", feature = "std"))]
mod as_ref_hashmap;

pub mod ascii;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap};

mod impl_macro;

pub mod unicode;

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    pub fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[derive(Default)]
    pub struct MockHasher {
        inner: Vec<u8>,
    }

    impl MockHasher {
        #[track_caller]
        pub fn as_str(&self) -> &str {
            str::from_utf8(&self.inner).unwrap()
        }
    }

    impl Hasher for MockHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.inner.extend_from_slice(bytes);
        }

        fn write_u8(&mut self, _: u8) {}

        fn finish(&self) -> u64 {
            0
        }
    }
}
