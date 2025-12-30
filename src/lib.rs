#![cfg_attr(not(feature = "std"), no_std)]

mod as_ref_hashmap;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap, CaseFoldMut};

mod case_fold_impl;
pub use case_fold_impl::{ascii, unicode};
