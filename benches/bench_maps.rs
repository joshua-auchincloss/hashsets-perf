use bench_hash_2023::SeaHash;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
};

// use std::ve

macro_rules! bench_for {
    ($typ: ident, $k: expr, $v: expr) => {
        bench_for! {$typ, $typ, $k, $v}
    };
    ($kt: ident, $vt: ident, $k: expr, $v: expr) => {
        paste::paste! {
            fn [<gen_bench_ $kt:snake _ $vt:snake>]<H>(name: &str, c: &mut Criterion)
            where
                H: BuildHasher + Default,
            {
                let mut h = HashMap::<$kt, $vt, H>::default();
                let to_insert = $v;
                let insert = |h: &mut HashMap<$kt, $vt, H>| {
                    h.insert($k, to_insert.clone());
                };
                let remove = |h: &mut HashMap<$kt, $vt, H>| {
                    h.remove(&$k)
                };
                let get = |h: &HashMap<$kt, $vt, H>| {
                    h.get(&$k);
                };
                let kt = stringify!{$kt};
                let vt = stringify!{$vt};
                let name_fn = |inner: &str| format!("{}_{}_{}_{}", name, inner, kt, vt);
                insert(&mut h);
                c.bench_function(
                    &name_fn("insert_noexist"),
                    |b| {
                    b.iter_custom(|iters| {
                        let mut tot = std::time::Duration::from_millis(0);
                        for _ in 0..iters {
                            remove(&mut h);
                            let start = std::time::Instant::now();
                            insert(black_box(&mut h));
                            tot += start.elapsed();
                        }
                        tot
                    })
                });
                c.bench_function(
                    &name_fn("insert_exist"),
                    |b| {
                    b.iter_custom(|iters| {
                        let mut tot = std::time::Duration::from_millis(0);
                        insert(&mut h);
                        for _ in 0..iters {
                            let start = std::time::Instant::now();
                            insert(black_box(&mut h));
                            tot += start.elapsed();
                        }
                        tot
                    })
                });
                c.bench_function(
                    &name_fn("get_noexist"),
                    |b| {
                    b.iter_custom(|iters| {
                        let mut tot = std::time::Duration::from_millis(0);
                        remove(&mut h);
                        for _ in 0..iters {
                            let start = std::time::Instant::now();
                            get(black_box(&h));
                            tot += start.elapsed();
                        }
                        tot
                    })
                });
                c.bench_function(
                    &name_fn("get_exist"),
                    |b| {
                    b.iter_custom(|iters| {
                        let mut tot = std::time::Duration::from_millis(0);
                        insert(&mut h);
                        for _ in 0..iters {
                            let start = std::time::Instant::now();
                            get(black_box(&h));
                            tot += start.elapsed();
                        }
                        tot
                    })
                });
                c.bench_function(
                    &name_fn("remove_noexist"),
                    |b| {
                    b.iter_custom(|iters| {
                        remove(&mut h);
                        let mut tot = std::time::Duration::from_millis(0);
                        for _ in 0..iters {
                            let start = std::time::Instant::now();
                            remove(black_box(&mut h));
                            tot += start.elapsed()
                        }
                        tot
                    })
                });
                c.bench_function(
                    &name_fn("remove_exists"),
                    |b| {
                    b.iter_custom(|iters| {
                        let mut tot = std::time::Duration::from_millis(0);
                        for _ in 0..iters {
                            insert(&mut h);
                            let start = std::time::Instant::now();
                            remove(black_box(&mut h));
                            tot += start.elapsed()
                        }
                        tot
                    })
                });
            }

            fn [<bench_std_hashmap_ $kt:snake _ $vt:snake>](b: &mut Criterion) {
                [<gen_bench_ $kt:snake _ $vt:snake>]::<RandomState>("std", b)
            }

            fn [<bench_seahash_hashmap_ $kt:snake _ $vt:snake>](b: &mut Criterion) {
                [<gen_bench_ $kt:snake _ $vt:snake>]::<SeaHash>("seahash", b)
            }

            fn [<bench_ahash_hashmap_ $kt:snake _ $vt:snake>](b: &mut Criterion) {
                [<gen_bench_ $kt:snake _ $vt:snake>]::<ahash::RandomState>("ahash", b)
            }


            criterion_group!(
                [<benches_ $kt:snake _ $vt:snake>],
                [<bench_std_hashmap_ $kt:snake _ $vt:snake>],
                [<bench_seahash_hashmap_ $kt:snake _ $vt:snake>],
                [<bench_ahash_hashmap_ $kt:snake _ $vt:snake>]
            );
        }
    };
}

macro_rules! bench_table {
    // (@inner $({$kt: ident, $vt: ident, $k: literal, $v: literal }), + $(,)?) => {

    // };
    ( $({ $kt: ident, $vt: ident, $k: expr, $v: expr }), + $(,)?) => {
        $(
            bench_for!{
                $kt,
                $vt,
                $k,
                $v
            }
        )*

        paste::paste!{
            criterion_main!(
                $(
                [<benches_ $kt:snake _ $vt:snake>],
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

#[repr(C)]
#[derive(PartialEq, Eq, Hash, Clone)]
enum StaticAbReprC {
    Abc,
    Def,
}

const COMPARABLE_KEY: &str = "k";
const COMPARABLE_SIZED: &str = "12345";

bench_table! {
    { RawStr, RawStr, COMPARABLE_KEY, COMPARABLE_SIZED},
    { RawStr, String, COMPARABLE_KEY, COMPARABLE_SIZED.to_string() },
    { RawStr, usize, COMPARABLE_KEY, 1 },
    { usize, usize, 1, 42 },
    { usize, RawStr, 1, COMPARABLE_SIZED },
    { usize, String, 1, COMPARABLE_SIZED.to_string() },
    { i32, i32, 1, 42 },
    { i32, usize, 1, 42 },
    { i32, RawStr, 1, COMPARABLE_SIZED },
    { String, String, COMPARABLE_KEY.to_string(), COMPARABLE_SIZED.to_string() },
    { String, RawStr, COMPARABLE_KEY.to_string(), COMPARABLE_SIZED },
    {
        MyKeyValue,
        MyKeyValue,
        MyKeyValue::Key(COMPARABLE_KEY.to_string()),
        MyKeyValue::Value(COMPARABLE_SIZED.to_string())
    },
    {
        MyKeyValue,
        RawStr,
        MyKeyValue::Key(COMPARABLE_KEY.to_string()),
        COMPARABLE_SIZED
    },
    {
        StaticAb,
        StaticAb,
        StaticAb::Abc,
        StaticAb::Def
    },
    {
        StaticAb,
        RawStr,
        StaticAb::Abc,
        COMPARABLE_SIZED
    },
    {
        StaticAbReprC,
        StaticAbReprC,
        StaticAbReprC::Abc,
        StaticAbReprC::Def
    }
}
