use core::borrow::Borrow;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FromIterator;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
#[cfg(feature = "hashbrown")]
pub(crate) use hashbrown::DefaultHashBuilder;
#[cfg(feature = "hashbrown")]
use hashbrown::hash_map::{Entry, HashMap};
#[cfg(not(feature = "hashbrown"))]
pub(crate) use std::collections::hash_map::RandomState as DefaultHashBuilder;
#[cfg(not(feature = "hashbrown"))]
use std::collections::hash_map::{Entry, HashMap};

#[repr(transparent)]
pub struct AsRefHashMap<R: ?Sized, K, V, S = DefaultHashBuilder>(HashMap<K, V, S>, PhantomData<R>);

impl<R: ?Sized, K, V, S> PartialEq for AsRefHashMap<R, K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R: ?Sized, K, V, S> Clone for AsRefHashMap<R, K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<R: ?Sized, K, V, S> fmt::Debug for AsRefHashMap<R, K, V, S>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<R: ?Sized, K, V, S> Eq for AsRefHashMap<R, K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

impl<R: ?Sized, K, V, S> Default for AsRefHashMap<R, K, V, S>
where
    S: Default,
{
    fn default() -> Self {
        Self(HashMap::with_hasher(S::default()), PhantomData)
    }
}

impl<R: ?Sized, K, V, S> AsRefHashMap<R, K, V, S>
where
    S: Default + BuildHasher,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(
            HashMap::with_capacity_and_hasher(capacity, S::default()),
            PhantomData,
        )
    }
}

impl<R: ?Sized, K, V, S> AsRefHashMap<R, K, V, S> {
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder), PhantomData)
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(
            HashMap::with_capacity_and_hasher(capacity, hash_builder),
            PhantomData,
        )
    }
}

impl<R: ?Sized, K, V, S> AsRefHashMap<R, K, V, S>
where
    R: Eq + Hash,
    K: Eq + Hash + Borrow<R>,
    S: BuildHasher,
{
    #[inline]
    #[cfg(not(feature = "hashbrown"))]
    pub fn entry<Q>(&mut self, k: Q) -> Entry<'_, K, V>
    where
        Q: Into<K>,
    {
        self.0.entry(k.into())
    }

    #[inline]
    #[cfg(feature = "hashbrown")]
    pub fn entry<Q>(&mut self, k: Q) -> Entry<'_, K, V, S>
    where
        Q: Into<K>,
    {
        self.0.entry(k.into())
    }

    #[inline]
    pub fn insert<Q>(&mut self, k: Q, v: V) -> Option<V>
    where
        Q: Into<K>,
    {
        self.0.insert(k.into(), v)
    }

    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get(k.as_ref())
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get_mut(k.as_ref())
    }

    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get_key_value(k.as_ref())
    }

    #[inline]
    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get_disjoint_mut(ks.map(Q::as_ref))
    }

    #[inline]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.contains_key(k.as_ref())
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.remove(k.as_ref())
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.remove_entry(k.as_ref())
    }
}

impl<R: ?Sized, K, V, S> Deref for AsRefHashMap<R, K, V, S> {
    type Target = HashMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: ?Sized, K, V, S> DerefMut for AsRefHashMap<R, K, V, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R: ?Sized, K, V, S> FromIterator<(K, V)> for AsRefHashMap<R, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter), PhantomData)
    }
}
