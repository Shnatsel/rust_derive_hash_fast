#![allow(unused)]
use criterion::{criterion_group, criterion_main, Criterion};
use derive_hash_fast::*;
use std::hint::black_box;
use std::hash::{DefaultHasher, Hash, Hasher};

criterion_group!(benches, bench_compound_struct_64, bench_compound_struct_80, bench_compound_struct_128, bench_compound_struct_160, bench_slice_of_compound_structs, bench_slice_of_u8_newtype, bench_short_slice_of_u8_newtype);
criterion_main!(benches);

fn hash_it(value: impl Hash, mut hasher: impl Hasher) -> u64 {
    // these black_box calls are load-bearing, the results a very different without them
    black_box(value).hash(black_box(&mut hasher));
    black_box(hasher.finish())
}

pub fn bench_compound_struct_64(c: &mut Criterion) {
    bench_compound_struct_64_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_compound_struct_64_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_compound_struct_64_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_compound_struct_64_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_compound_struct_64_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}

pub fn bench_compound_struct_64_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound64::Derive {a: true, b: 10, c: 20, d: 'a'}, "Compound 64-bit struct with [derive(Hash)]", 
        compound64::FastB {a: true, b: 10, c: 20, d: 'a'}, "Compound 64-bit struct with derive_hash_fast_bytemuck", 
        compound64::FastZ {a: true, b: 10, c: 20, d: 'a'}, "Compound 64-bit struct with derive_hash_fast_zerocopy",
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


pub fn bench_compound_struct_80_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound80::Derive {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "Compound 80-bit struct with [derive(Hash)]", 
        compound80::FastB {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "Compound 80-bit struct with derive_hash_fast_bytemuck", 
        compound80::FastZ {a: true, b: 2, c: 1337, d: 5, e: 69, f: 0xFFF}, "Compound 80-bit struct with derive_hash_fast_zerocopy",
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


pub fn bench_compound_struct_128_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound128::Derive {a: 1, b: 2, c: 1337, d: 100500}, "Compound 128-bit struct with [derive(Hash)]", 
        compound128::FastB {a: 1, b: 2, c: 1337, d: 100500}, "Compound 128-bit struct with derive_hash_fast_bytemuck", 
        compound128::FastZ {a: 1, b: 2, c: 1337, d: 100500}, "Compound 128-bit struct with derive_hash_fast_zerocopy",
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


pub fn bench_compound_struct_160_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        compound160::Derive {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "Compound 160-bit struct with [derive(Hash)]", 
        compound160::FastB {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "Compound 160-bit struct with derive_hash_fast_bytemuck", 
        compound160::FastZ {a: 1, b: 2, c: 1337, d: 100500, e: 30}, "Compound 160-bit struct with derive_hash_fast_zerocopy",
        hasher, hasher_name
    );
}

pub fn bench_slice_of_compound_structs(c: &mut Criterion) {
    bench_slice_of_compound_structs_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_slice_of_compound_structs_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_slice_of_compound_structs_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_slice_of_compound_structs_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_slice_of_compound_structs_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}

pub fn bench_slice_of_compound_structs_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        vec![compound64::Derive {a: true, b: 10, c: 20, d: 'a'}; 1024].as_slice(), "Slice of compound 64-bit structs with [derive(Hash)]", 
        vec![compound64::FastB {a: true, b: 10, c: 20, d: 'a'}; 1024].as_slice(), "Slice of compound 64-bit structs with derive_hash_fast_bytemuck", 
        vec![compound64::FastZ {a: true, b: 10, c: 20, d: 'a'}; 1024].as_slice(), "Slice of compound 64-bit structs with derive_hash_fast_zerocopy",
        hasher, hasher_name
    );
}

pub fn bench_slice_of_u8_newtype(c: &mut Criterion) {
    bench_slice_of_u8_newtype_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_slice_of_u8_newtype_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_slice_of_u8_newtype_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_slice_of_u8_newtype_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_slice_of_u8_newtype_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}

pub fn bench_slice_of_u8_newtype_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        vec![U8NewtypeDerive(5); 1024].as_slice(), "Slice of Newtype(u8) with [derive(Hash)], length 1024", 
        vec![U8NewtypeFastB(5); 1024].as_slice(), "Slice of Newtype(u8) with derive_hash_fast_bytemuck, length 1024", 
        vec![U8NewtypeFastZ(5); 1024].as_slice(), "Slice of Newtype(u8) with derive_hash_fast_zerocopy, length 1024",
        hasher, hasher_name
    );
}

pub fn bench_short_slice_of_u8_newtype(c: &mut Criterion) {
    bench_short_slice_of_u8_newtype_with_hasher(c, DefaultHasher::default(), "std::hash::DefaultHasher");
    bench_short_slice_of_u8_newtype_with_hasher(c, rustc_hash::FxHasher::default(), "rustc_hash::FxHasher");
    bench_short_slice_of_u8_newtype_with_hasher(c, rapidhash::RapidHasher::default(), "rapidhash::RapidHasher");
    bench_short_slice_of_u8_newtype_with_hasher(c, ahash::AHasher::default(), "ahash::AHasher");
    bench_short_slice_of_u8_newtype_with_hasher(c, xxhash_rust::xxh3::Xxh3Default::default(), "xxh3::Xxh3Default");
}

pub fn bench_short_slice_of_u8_newtype_with_hasher(c: &mut Criterion, hasher: impl Hasher + Clone, hasher_name: &str) {
    bench_structs_with_hasher(c, 
        vec![U8NewtypeDerive(5); 4].as_slice(), "Slice of Newtype(u8) with [derive(Hash)], length 4", 
        vec![U8NewtypeFastB(5); 4].as_slice(), "Slice of Newtype(u8) with derive_hash_fast_bytemuck, length 4", 
        vec![U8NewtypeFastZ(5); 4].as_slice(), "Slice of Newtype(u8) with derive_hash_fast_zerocopy, length 4",
        hasher, hasher_name
    );
}

pub fn bench_structs_with_hasher(c: &mut Criterion, struct_1: impl Hash + Clone, struct_1_name: &str, struct_2: impl Hash + Clone, struct_2_name: &str, struct_3: impl Hash + Clone, struct_3_name: &str, mut hasher: impl Hasher + Clone, hasher_name: &str) {
    let mut group = c.benchmark_group(hasher_name);
    group.bench_function(struct_1_name, |b| b.iter(|| hash_it(struct_1.clone(), hasher.clone())));
    group.bench_function(struct_2_name, |b| b.iter(|| hash_it(struct_2.clone(), hasher.clone())));
    group.bench_function(struct_3_name, |b| b.iter(|| hash_it(struct_3.clone(), hasher.clone())));
    group.finish();
}

#[macro_export]
macro_rules! generate_structs {
    (
        pub struct $name:ident {
            $( $field_name:ident : $field_type:ty ),*
        }
    ) => {
        #[derive(Hash, Clone)]
        pub struct Derive {
            $( pub $field_name: $field_type ),*
        }

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::NoUninit)]
        pub struct FastB {
            $( pub $field_name: $field_type ),*
        }

        derive_hash_fast::derive_hash_fast_bytemuck!(FastB);

        #[derive(Clone, zerocopy::Immutable, zerocopy::IntoBytes)]
        pub struct FastZ {
            $( pub $field_name: $field_type ),*
        }

        derive_hash_fast::derive_hash_fast_zerocopy!(FastZ);

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::NoUninit, bytemuck::ByteHash)]
        pub struct ByteHashB {
            $( pub $field_name: $field_type ),*
        }

        #[derive(Clone, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::ByteHash)]
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