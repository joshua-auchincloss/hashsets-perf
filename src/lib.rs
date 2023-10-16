use codspeed_criterion_compat::{measurement::WallTime, BenchmarkGroup, Criterion, SamplingMode};
use rustc_hash::FxHasher;
use seahash::SeaHasher;
use std::{hash::BuildHasher, time::Duration};

#[derive(Default, Clone)]
#[repr(transparent)]
pub struct SeaHash;

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

#[derive(Default)]
#[repr(transparent)]
pub struct RustC;

impl RustC {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl BuildHasher for RustC {
    type Hasher = FxHasher;
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher::default()
    }
}

pub fn crit_group<'a>(c: &'a mut Criterion, name: &str) -> BenchmarkGroup<'a, WallTime> {
    let mut group = c.benchmark_group(name);
    group
        .measurement_time(Duration::from_millis(500))
        .sample_size(100)
        .confidence_level(0.98)
        .sampling_mode(SamplingMode::Linear)
        .warm_up_time(Duration::from_millis(100));
    group
}
