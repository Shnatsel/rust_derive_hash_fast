#![allow(unused)]
use arbitrary::{Arbitrary, Unstructured};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use derive_hash_fast::*;
use std::collections::HashSet;
use std::hint::black_box;
use std::hash::{BuildHasherDefault, DefaultHasher, Hash, Hasher};

criterion_group!(benches, bench_compound_struct_64, bench_compound_struct_80, bench_compound_struct_128, bench_compound_struct_160);
criterion_main!(benches);

pub fn bench_compound_struct_64(c: &mut Criterion) {
    bench_compound_struct_64_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_compound_struct_64_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_compound_struct_64_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_compound_struct_64_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_compound_struct_64_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}

pub fn bench_compound_struct_64_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone + Default, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound64::Derive {a: true, b: 10, c: 20, d: 'a'}, "HashSet of compound 64-bit struct with [derive(Hash)]", 
        compound64::FastB {a: true, b: 10, c: 20, d: 'a'}, "HashSet of compound 64-bit struct with derive_hash_fast_bytemuck", 
        compound64::FastZ {a: true, b: 10, c: 20, d: 'a'}, "HashSet of compound 64-bit struct with derive_hash_fast_zerocopy",
        compound64::ByteHashB {a: true, b: 10, c: 20, d: 'a'}, "HashSet of compound 64-bit struct with bytemuck::ByteHash",
        compound64::ByteHashZ {a: true, b: 10, c: 20, d: 'a'}, "HashSet of compound 64-bit struct with zerocopy::ByteHash",
        hasher, hasher_name
    );
}

pub fn bench_compound_struct_80(c: &mut Criterion) {
    bench_compound_struct_80_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_compound_struct_80_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_compound_struct_80_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_compound_struct_80_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_compound_struct_80_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}


pub fn bench_compound_struct_80_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone + Default, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound80::Derive {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "HashSet of compound 80-bit struct with [derive(Hash)]", 
        compound80::FastB {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "HashSet of compound 80-bit struct with derive_hash_fast_bytemuck", 
        compound80::FastZ {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "HashSet of compound 80-bit struct with derive_hash_fast_zerocopy",
        compound80::ByteHashB {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "HashSet of compound 80-bit struct with bytemuck::ByteHash",
        compound80::ByteHashZ {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "HashSet of compound 80-bit struct with zerocopy::ByteHash",
        hasher, hasher_name
    );
}

pub fn bench_compound_struct_128(c: &mut Criterion) {
    bench_compound_struct_128_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_compound_struct_128_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_compound_struct_128_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_compound_struct_128_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_compound_struct_128_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}


pub fn bench_compound_struct_128_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone + Default, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound128::Derive {a: 1, b: 2, c: 1337, d: 100500}, "HashSet of compound 128-bit struct with [derive(Hash)]", 
        compound128::FastB {a: 1, b: 2, c: 1337, d: 100500}, "HashSet of compound 128-bit struct with derive_hash_fast_bytemuck", 
        compound128::FastZ {a: 1, b: 2, c: 1337, d: 100500}, "HashSet of compound 128-bit struct with derive_hash_fast_zerocopy",
        compound128::ByteHashB {a: 1, b: 2, c: 1337, d: 100500}, "HashSet of compound 128-bit struct with bytemuck::ByteHash",
        compound128::ByteHashZ {a: 1, b: 2, c: 1337, d: 100500}, "HashSet of compound 128-bit struct with zerocopy::ByteHash",
        hasher, hasher_name
    );
}

pub fn bench_compound_struct_160(c: &mut Criterion) {
    bench_compound_struct_160_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_compound_struct_160_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_compound_struct_160_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_compound_struct_160_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_compound_struct_160_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}


pub fn bench_compound_struct_160_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone + Default, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound160::Derive {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "HashSet of compound 160-bit struct with [derive(Hash)]", 
        compound160::FastB {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "HashSet of compound 160-bit struct with derive_hash_fast_bytemuck", 
        compound160::FastZ {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "HashSet of compound 160-bit struct with derive_hash_fast_zerocopy",
        compound160::ByteHashB {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "HashSet of compound 160-bit struct with bytemuck::ByteHash",
        compound160::ByteHashZ {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "HashSet of compound 160-bit struct with zerocopy::ByteHash",
        hasher, hasher_name
    );
}

pub fn bench_structs_with_hasher(c: &mut Criterion, 
    struct_1: impl Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, struct_1_name: &str,
    struct_2: impl Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, struct_2_name: &str,
    struct_3: impl Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, struct_3_name: &str,
    struct_4: impl Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, struct_4_name: &str,
    struct_5: impl Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, struct_5_name: &str,
    mut hasher: impl Hasher + Clone + Default, hasher_name: &str
) {
    let mut group = c.benchmark_group(hasher_name);
    group.bench_function(struct_1_name, |b| bench_struct(b, &struct_1, &hasher));
    group.bench_function(struct_2_name, |b| bench_struct(b, &struct_2, &hasher));
    group.bench_function(struct_3_name, |b| bench_struct(b, &struct_3, &hasher));
    group.bench_function(struct_4_name, |b| bench_struct(b, &struct_4, &hasher));
    group.bench_function(struct_5_name, |b| bench_struct(b, &struct_5, &hasher));
    group.finish();
}

pub fn bench_struct<S: Hash + Eq + PartialEq + Clone + for<'a> Arbitrary<'a>, H: Hasher + Default>(b: &mut Bencher, struct_to_bench: &S, mut hasher: &H) {
    let mut data = vec![0; 4096];
    fastrand::fill(&mut data);
    let mut unstructured = Unstructured::new(&data);
    let mut structs: Vec<S> = Vec::new();
    for i_ in 0..256 {
        structs.push(S::arbitrary(&mut unstructured).unwrap());
    }

    b.iter(|| {
        let mut set: HashSet<S, BuildHasherDefault<H>> = HashSet::default();
        let mut structs_to_insert = (&structs[..structs.len() / 2]).to_vec();

        // insert half the structs into the set, triggering reallocations and rehashing
        for s in structs_to_insert.into_iter() {
            set.insert(s);
        }

        // look up the structs we inserted and some we haven't
        let mut counter: usize = 0;
        for s in structs.iter() {
            if set.contains(s) {
                counter += 1;
            }
        }

        assert_eq!(counter, structs.len() / 2);
    });
}

#[macro_export]
macro_rules! generate_structs {
    (
        pub struct $name:ident {
            $( $field_name:ident : $field_type:ty ),*
        }
    ) => {
        #[derive(Hash, Clone, Eq, PartialEq, arbitrary::Arbitrary)]
        pub struct Derive {
            $( pub $field_name: $field_type ),*
        }

        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, arbitrary::Arbitrary, bytemuck::NoUninit)]
        pub struct FastB {
            $( pub $field_name: $field_type ),*
        }

        derive_hash_fast::derive_hash_fast_bytemuck!(FastB);

        #[derive(Clone, Eq, PartialEq, arbitrary::Arbitrary, zerocopy::Immutable, zerocopy::IntoBytes)]
        pub struct FastZ {
            $( pub $field_name: $field_type ),*
        }

        derive_hash_fast::derive_hash_fast_zerocopy!(FastZ);

        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, arbitrary::Arbitrary, bytemuck::NoUninit, bytemuck::ByteHash)]
        pub struct ByteHashB {
            $( pub $field_name: $field_type ),*
        }

        #[derive(Clone, Eq, PartialEq, arbitrary::Arbitrary, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::ByteHash)]
        pub struct ByteHashZ {
            $( pub $field_name: $field_type ),*
        }
    };
}

mod compound64 {
    use super::generate_structs;

    generate_structs! {
        pub struct Compound64 {
            a: bool,
            b: u8,
            c: u16,
            d: char
        }
    }
}

mod compound80 {
    use super::generate_structs;

    generate_structs! {
        pub struct compound80 {
            a: bool,
            b: u8,
            c: u16,
            d: u16,
            e: u16,
            f: u16
        }
    }
}

mod compound128 {
    use super::generate_structs;

    generate_structs! {
        pub struct Compound128 {
            a: u32,
            b: u32,
            c: u32,
            d: u32
        }
    }
}

mod compound160 {
    use super::generate_structs;

    generate_structs! {
        pub struct Compound160 {
            a: u32,
            b: u32,
            c: u32,
            d: u32,
            e: u32
        }
    }
}

#[derive(Clone, Hash)]
struct U8NewtypeDerive(u8);

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct U8NewtypeFastB(u8);

derive_hash_fast_bytemuck!(U8NewtypeFastB);

#[derive(Clone, zerocopy::Immutable, zerocopy::IntoBytes)]
struct U8NewtypeFastZ(u8);

derive_hash_fast_zerocopy!(U8NewtypeFastZ);

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::NoUninit, bytemuck::ByteHash)]
struct U8NewtypeByteHashB(u8);

#[derive(Clone, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::ByteHash)]
struct U8NewtypeByteHashZ(u8);