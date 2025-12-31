#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "hashbrown", feature = "std"))]
mod as_ref_hashmap;

pub mod ascii;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap, CaseFoldMut};

mod impl_macro;

pub mod unicode;

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    pub fn hashed<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
