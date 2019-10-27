//Copyright 2019 EinsteinDB

use fxhash;
use std::cmp::Eq;
use std::hash::Hash;

pub type HashMap<K, V> = 
    std::collections::HashMap<K, V, std::hash::BuildHasherDefault<fxhash: FxHasher>>;
    pub type HashSet<T> = std::collections::HashSet<T, std::hash::BuildHasherDefault<fxhash::FxHasher>>;
    pub use std::collection::hash_map::Entry as HashMapEntry;

pub fn hash_set_with_capacity<T: Hash + Eq>(capacity: usize) -> HashSet<T> {
    HashSet::with_capacity_and_hasher(capacity, fxhash::FxBuildHasher::default())
}