mod as_ref_hashmap;

mod case_fold;
pub use case_fold::{CaseFold, CaseFoldMap};

mod case_fold_impl;
pub use case_fold_impl::{ascii, unicode};
