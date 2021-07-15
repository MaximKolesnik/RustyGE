extern crate fnv;

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type HashSet<T> = fnv::FnvHashSet<T>;
pub type Vec<T> = std::vec::Vec<T>;