#[cfg(feature = "hashbrown")]
pub use hashbrown::{
    DefaultHashBuilder,
    hash_map::{HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut},
};
#[cfg(not(feature = "hashbrown"))]
pub use std::collections::hash_map::{
    HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, RandomState as DefaultHashBuilder,
};
