#[macro_export]
macro_rules! impl_casefold {
    ($t:ty) => {
        impl<S> Eq for CaseFold<S> where S: AsRef<$t> + ?Sized {}

        impl<S, Rhs> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
        where
            S: AsRef<$t> + ?Sized,
            Rhs: AsRef<$t> + ?Sized,
        {
            #[inline]
            fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<core::cmp::Ordering> {
                Some(self.caseless_iter().cmp(other.caseless_iter()))
            }
        }

        impl<S> Ord for CaseFold<S>
        where
            S: AsRef<$t> + ?Sized,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.caseless_iter().cmp(other.caseless_iter())
            }
        }

        impl<S> CaseFold<S> {
            /// Construct a new `CaseFold`.
            #[inline]
            pub const fn new(s: S) -> Self {
                Self(s)
            }

            /// Consume this `CaseFold` and get the inner value.
            #[inline]
            pub fn into_inner(self) -> S {
                self.0
            }
        }

        impl<S: ?Sized> core::ops::Deref for CaseFold<S> {
            type Target = S;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<S: ?Sized> core::ops::DerefMut for CaseFold<S> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<'a, S: ?Sized> From<&CaseFold<&'a S>> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &CaseFold<&'a S>) -> Self {
                value.0.into()
            }
        }

        impl<S> CaseFold<S>
        where
            S: AsRef<str> + ?Sized,
        {
            /// Borrows the inner value as a `str`.
            #[inline]
            pub fn as_str(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl<S> From<S> for CaseFold<S> {
            #[inline]
            fn from(value: S) -> Self {
                CaseFold::new(value)
            }
        }

        impl<S> core::str::FromStr for CaseFold<S>
        where
            S: core::str::FromStr,
        {
            type Err = <S as core::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse().map(CaseFold)
            }
        }

        impl<'a, S: ?Sized> From<&'a S> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &'a S) -> Self {
                CaseFold::borrow(value)
            }
        }

        impl<S: AsRef<$t>> core::borrow::Borrow<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn borrow(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.0.as_ref())
            }
        }

        impl AsRef<CaseFold<$t>> for CaseFold<$t> {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self
            }
        }

        impl<S> AsRef<CaseFold<$t>> for CaseFold<S>
        where
            S: AsRef<$t>,
        {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self.0.as_ref().into()
            }
        }

        impl<S> AsRef<$t> for CaseFold<S>
        where
            S: AsRef<$t> + ?Sized,
        {
            #[inline]
            fn as_ref(&self) -> &$t {
                self.0.as_ref()
            }
        }

        impl<'a, S> core::fmt::Debug for CaseFold<S>
        where
            S: AsRef<$t> + ?Sized,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.as_ref().fmt(f)
            }
        }

        impl<'a, S: ?Sized> core::fmt::Display for CaseFold<S>
        where
            S: AsRef<str>,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.as_ref().fmt(f)
            }
        }

        impl AsRef<CaseFold<$t>> for str {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.as_ref())
            }
        }

        impl AsRef<CaseFold<$t>> for String {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                CaseFold::borrow(self.as_ref())
            }
        }

        #[cfg(feature = "std")]
        impl ToOwned for CaseFold<$t> {
            type Owned = CaseFold<<$t as ToOwned>::Owned>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                CaseFold(self.0.to_owned())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_casefoldmap {
    ($t:ty) => {
        impl<K, V, S> Clone for CaseFoldMap<K, V, S>
        where
            K: Clone,
            V: Clone,
            S: Clone,
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<K, V, S> PartialEq for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            V: PartialEq,
            S: core::hash::BuildHasher,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<K, V, S> Eq for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            V: Eq,
            S: core::hash::BuildHasher,
        {
        }

        impl<K, V, S> core::fmt::Debug for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            V: core::fmt::Debug,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<K, V, S> Default for CaseFoldMap<K, V, S>
        where
            S: Default,
        {
            fn default() -> Self {
                Self($crate::map::HashMap::with_hasher(S::default()))
            }
        }

        impl<K, V, S> CaseFoldMap<K, V, S>
        where
            S: Default + core::hash::BuildHasher,
        {
            /// Creates an empty `CaseFoldMap`.
            ///
            /// The hash map is initially created with a capacity of 0, so it will not allocate until it
            /// is first inserted into.
            pub fn new() -> Self {
                Self::default()
            }

            /// Creates an empty `CaseFoldMap` with at least the specified capacity.
            ///
            /// The hash map will be able to hold at least `capacity` elements without
            /// reallocating. This method is allowed to allocate for more elements than
            /// `capacity`. If `capacity` is zero, the hash map will not allocate.
            pub fn with_capacity(capacity: usize) -> Self {
                Self($crate::map::HashMap::with_capacity_and_hasher(
                    capacity,
                    S::default(),
                ))
            }
        }

        impl<K, V, S> CaseFoldMap<K, V, S> {
            /// Creates an empty `CaseFoldMap` which will use the given hash builder to hash
            /// keys.
            ///
            /// The created map has the default initial capacity.
            ///
            /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
            /// the `CaseFoldMap` to be useful, see its documentation for details.
            pub const fn with_hasher(hash_builder: S) -> Self {
                Self($crate::map::HashMap::with_hasher(hash_builder))
            }

            /// Creates an empty `CaseFoldMap` with at least the specified capacity, using
            /// `hasher` to hash the keys.
            ///
            /// The hash map will be able to hold at least `capacity` elements without
            /// reallocating. This method is allowed to allocate for more elements than
            /// `capacity`. If `capacity` is zero, the hash map will not allocate.
            ///
            /// The `hasher` passed should implement the [`BuildHasher`] trait for
            /// the `HashMap` to be useful, see its documentation for details.
            ///
            pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
                Self($crate::map::HashMap::with_capacity_and_hasher(
                    capacity,
                    hash_builder,
                ))
            }
        }

        impl<K, V, S> CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            S: core::hash::BuildHasher,
        {
            /// Creates a consuming iterator visiting all the keys in arbitrary order. The map cannot be used after calling this. The iterator element type is `CaseFold<K>`.
            #[inline]
            pub fn into_keys(self) -> $crate::map::IntoKeys<CaseFold<K>, V> {
                self.0.into_keys()
            }

            /// Creates a consuming iterator visiting all the values in arbitrary order. The map cannot be used after calling this. The iterator element type is `V`.
            #[inline]
            pub fn into_values(self) -> $crate::map::IntoValues<CaseFold<K>, V> {
                self.0.into_values()
            }

            /// Gets the given key’s corresponding entry in the map for in-place manipulation.
            #[inline]
            #[cfg(not(feature = "hashbrown"))]
            pub fn entry(&mut self, k: K) -> std::collections::hash_map::Entry<'_, CaseFold<K>, V> {
                self.0.entry(CaseFold::new(k))
            }

            /// Gets the given key’s corresponding entry in the map for in-place manipulation.
            #[inline]
            #[cfg(feature = "hashbrown")]
            pub fn entry(&mut self, k: K) -> hashbrown::hash_map::Entry<'_, CaseFold<K>, V, S> {
                self.0.entry(CaseFold::new(k))
            }

            /// Returns a reference to the value corresponding to the key.
            #[inline]
            pub fn get<Q>(&self, k: &Q) -> Option<&V>
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.get(CaseFold::borrow(k.as_ref()))
            }

            /// Returns the key-value pair corresponding to the supplied key.
            #[inline]
            pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&CaseFold<K>, &V)>
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.get_key_value(CaseFold::borrow(k.as_ref()))
            }

            /// Attempts to get mutable references to `N` values in the map at once.
            ///
            /// Returns an array of length `N` with the results of each query. For soundness, at most one
            /// mutable reference will be returned to any value. `None` will be used if the key is missing.
            ///
            /// This method performs a check to ensure there are no duplicate keys, which currently has a time-complexity of O(n^2),
            /// so be careful when passing many keys.
            ///
            /// # Panics
            ///
            /// Panics if any keys are overlapping.
            #[inline]
            pub fn get_disjoint_mut<Q, const N: usize>(
                &mut self,
                ks: [&Q; N],
            ) -> [Option<&mut V>; N]
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0
                    .get_disjoint_mut(ks.map(|k| CaseFold::borrow(k.as_ref())))
            }

            /// Attempts to get mutable references to `N` values in the map at once, without validating that
            /// the values are unique.
            ///
            /// Returns an array of length `N` with the results of each query. `None` will be used if
            /// the key is missing.
            ///
            /// For a safe alternative see [`get_disjoint_mut`](`CaseFoldMap::get_disjoint_mut`).
            ///
            /// # Safety
            ///
            /// Calling this method with overlapping keys is undefined behavior even if the resulting
            /// references are not used.
            #[inline]
            pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(
                &mut self,
                ks: [&Q; N],
            ) -> [Option<&mut V>; N]
            where
                Q: AsRef<$t> + ?Sized,
            {
                // SAFETY: Keys are non-overlapping, per contract.
                unsafe {
                    self.0
                        .get_disjoint_unchecked_mut(ks.map(|k| CaseFold::borrow(k.as_ref())))
                }
            }

            /// Returns true if the map contains a value for the specified key.
            #[inline]
            pub fn contains_key<Q>(&self, k: &Q) -> bool
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.contains_key(CaseFold::borrow(k.as_ref()))
            }

            /// Returns a mutable reference to the value corresponding to the key.
            #[inline]
            pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.get_mut(CaseFold::borrow(k.as_ref()))
            }

            /// Inserts a key-value pair into the map.
            ///
            /// If the map did not have this key present, [`None`] is returned.
            ///
            /// If the map did have this key present, the value is updated, and the old
            /// value is returned. The key is not updated, though; this matters for
            /// types that can be `==` without being identical.
            #[inline]
            pub fn insert(&mut self, k: K, v: V) -> Option<V> {
                self.0.insert(CaseFold::new(k), v)
            }

            /// Removes a key from the map, returning the value at the key if the key was previously in
            /// the map.
            #[inline]
            pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.remove(CaseFold::borrow(k.as_ref()))
            }

            /// Removes a key from the map, returning the stored key and value if the key was previously in
            /// the map.
            #[inline]
            pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(CaseFold<K>, V)>
            where
                Q: AsRef<$t> + ?Sized,
            {
                self.0.remove_entry(CaseFold::borrow(k.as_ref()))
            }
        }

        impl<K, Q, V, S> core::ops::Index<&Q> for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            Q: AsRef<$t> + ?Sized,
            S: core::hash::BuildHasher,
        {
            type Output = V;

            #[inline]
            fn index(&self, key: &Q) -> &V {
                self.0.index(CaseFold::borrow(key.as_ref()))
            }
        }

        impl<K, V, S> core::ops::Deref for CaseFoldMap<K, V, S> {
            type Target = $crate::map::HashMap<CaseFold<K>, V, S>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<K, V, S> core::ops::DerefMut for CaseFoldMap<K, V, S> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<K, V, S> IntoIterator for CaseFoldMap<K, V, S> {
            type Item = (CaseFold<K>, V);
            type IntoIter = $crate::map::IntoIter<CaseFold<K>, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, K, V, S> IntoIterator for &'a CaseFoldMap<K, V, S> {
            type Item = (&'a CaseFold<K>, &'a V);
            type IntoIter = $crate::map::Iter<'a, CaseFold<K>, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a, K, V, S> IntoIterator for &'a mut CaseFoldMap<K, V, S> {
            type Item = (&'a CaseFold<K>, &'a mut V);
            type IntoIter = $crate::map::IterMut<'a, CaseFold<K>, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl<K, V, S> Extend<(K, V)> for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            S: core::hash::BuildHasher,
        {
            #[inline]
            fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
                self.0
                    .extend(iter.into_iter().map(|(k, v)| (CaseFold::new(k), v)));
            }
        }

        impl<K, V, S> FromIterator<(K, V)> for CaseFoldMap<K, V, S>
        where
            K: AsRef<$t>,
            S: core::hash::BuildHasher + Default,
        {
            fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
                let mut map = Self::with_hasher(Default::default());
                map.extend(iter);
                map
            }
        }

        impl<K, V, const N: usize> From<[(K, V); N]>
            for CaseFoldMap<K, V, $crate::map::DefaultHashBuilder>
        where
            K: AsRef<$t>,
        {
            fn from(arr: [(K, V); N]) -> Self {
                Self::from_iter(arr)
            }
        }
    };
}
