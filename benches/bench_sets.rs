use bench_hash_2023::{crit_group, RustC, SeaHash};
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use fnv::FnvBuildHasher;
use hashbrown::hash_map::DefaultHashBuilder as HashBrownHasher;
use paste::paste;
use std::{
    collections::{hash_map::RandomState, HashSet},
    hash::{BuildHasher, Hash},
};

fn name_fn(name: &str, inner: &str, vt: &str) -> String {
    format!("HashSet<{},{}>::{}", vt, name, inner)
}

macro_rules! bench_for {
    ($vt: ident, $v: expr) => {
        paste! {
            fn [<gen_bench_  $vt:snake>]<H>(name: &str, c: &mut Criterion)
            where
                H: BuildHasher + Default,
            {
                let mut group = crit_group(c, name);
                let new =  || {
                    HashSet::<$vt, H>::default()
                };
                let insert = |h: &mut HashSet<$vt, H>| {
                    h.insert($v);
                };
                let remove = |h: &mut HashSet<$vt, H>| {
                    h.remove(&$v)
                };
                let get = |h: &HashSet<$vt, H>| {
                    h.get(&$v);
                };
                let vt = stringify!{$vt};
                let name_fn = |inner: &str| name_fn(name, inner, vt);
                group.bench_function(
                    &name_fn("insert[!exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            remove(&mut h);
                            h
                        },
                        |mut this| {
                            insert(black_box(&mut this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
                group.bench_function(
                    &name_fn("insert[exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            insert(&mut h);
                            h
                        },
                        |mut this| {
                            insert(black_box(&mut this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
                group.bench_function(
                    &name_fn("get[!exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            remove(&mut h);
                            h
                        },
                        |this| {
                            get(black_box(&this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
                group.bench_function(
                    &name_fn("get[exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            insert(&mut h);
                            h
                        },
                        |this| {
                            get(black_box(&this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
                group.bench_function(
                    &name_fn("remove[!exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            remove(&mut h);
                            h
                        },
                        |mut this| {
                            remove(black_box(&mut this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
                group.bench_function(
                    &name_fn("remove[exists]"),
                    |b| {
                    b.iter_batched_ref(
                        || {
                            let mut h = new();
                            insert(&mut h);
                            h
                        },
                        |mut this| {
                            remove(black_box(&mut this))
                        },
                        codspeed_criterion_compat::BatchSize::SmallInput,
                    )
                });
            }

            fn [< $vt:snake _bench_std_hashset>](b: &mut Criterion) {
                [<gen_bench_  $vt:snake>]::<RandomState>("std::collections::hash_map::RandomState", b)
            }

            fn [< $vt:snake _bench_seahash_hashset>](b: &mut Criterion) {
                [<gen_bench_  $vt:snake>]::<SeaHash>("seahash::SeaHasher", b)
            }

            fn [< $vt:snake _bench_ahash_hashset>](b: &mut Criterion) {
                [<gen_bench_  $vt:snake>]::<ahash::RandomState>("ahash::RandomState", b)
            }

            fn [< $vt:snake _bench_hashbrown_hashset>](b: &mut Criterion) {
                [<gen_bench_  $vt:snake>]::<HashBrownHasher>("hashbrown::DefaultHashBuilder", b)
            }

            fn [<$vt:snake _bench_fnv_hashset>](b: &mut Criterion) {
                [<gen_bench_ $vt:snake>]::<FnvBuildHasher>("fnv::FnvBuildHasher", b)
            }

            fn [<$vt:snake _ bench_rustc_hash_hashset>](b: &mut Criterion) {
                [<gen_bench_ $vt:snake>]::<RustC>("rustc_hash::FxHasher", b)
            }


            criterion_group!(
                [<benches_ $vt:snake>],
                [<$vt:snake _bench_std_hashset>],
                [<$vt:snake _bench_seahash_hashset>],
                [<$vt:snake _bench_hashbrown_hashset>],
                [<$vt:snake _bench_ahash_hashset>],
                [<$vt:snake _bench_fnv_hashset>],
                [<$vt:snake _ bench_rustc_hash_hashset>]
            );
        }
    };
}

macro_rules! bench_table {
    ( $({ $vt: ident, $v: expr }), + $(,)?) => {
        $(
            bench_for!{
                $vt,
                $v
            }
        )*

        paste::paste!{
            criterion_main!(
                $(
                [<benches_ $vt:snake>],
                )*
            );
        }

    };
}

type RawStr = &'static str;

#[derive(PartialEq, Eq, Clone)]
enum MyKeyValue {
    Key(String),
    Value(String),
}

impl Hash for MyKeyValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Key(s) => state.write(s.as_bytes()),
            Self::Value(s) => state.write(s.as_bytes()),
        }
    }
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for datum in data {
            datum.hash(state)
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum StaticAb {
    Abc,
    Def,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum StaticAbSized {
    Abc,
    Def(Vec<u8>),
}

#[repr(C)]
#[derive(PartialEq, Eq, Hash, Clone)]
enum StaticAbReprC {
    Abc,
    Def,
}

const COMPARABLE_KEY: &str = "k";
const COMPARABLE_SIZED: &str = "12345";

type Bts2 = Vec<u8>;
type Bts4 = Vec<u8>;
type Bts8 = Vec<u8>;
type Bts32 = Vec<u8>;
type Bts64 = Vec<u8>;
type Bts128 = Vec<u8>;
type Bts256 = Vec<u8>;

bench_table! {
    { RawStr, COMPARABLE_SIZED},
    { String, COMPARABLE_SIZED.to_string() },
    { usize, 1 },
    { i32, 42 },
    {
        MyKeyValue,
        MyKeyValue::Key(COMPARABLE_KEY.to_string())
    },
    {
        StaticAb,
        StaticAb::Abc
    },
    {
        StaticAbReprC,
        StaticAbReprC::Def
    },
    {
        StaticAbSized,
        StaticAbSized::Def(Vec::with_capacity(2^8))
    },
    {
        Bts2,
        Vec::with_capacity(2)
    },
    {
        Bts4,
        Vec::with_capacity(4)
    },
    {
        Bts8,
        Vec::with_capacity(8)
    },
    {
        Bts32,
        Vec::with_capacity(32)
    },
    {
        Bts64,
        Vec::with_capacity(64)
    },
    {
        Bts128,
        Vec::with_capacity(128)
    },
    {
        Bts256,
        Vec::with_capacity(256)
    },
}
