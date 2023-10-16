#![feature(test)]

use seahash::SeaHasher;
use std::{collections::HashMap, hash::BuildHasher};

#[derive(Default)]
#[repr(transparent)]
pub struct SeaHash(SeaHasher);

impl SeaHash {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl BuildHasher for SeaHash {
    type Hasher = SeaHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SeaHasher::new()
    }
}

pub type SeaHashMap<K, V> = HashMap<K, V, SeaHash>;
