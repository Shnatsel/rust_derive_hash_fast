# A faster `#[derive(Hash)]` for Rust

**TL;DR:** `#[derive(Hash)]` hashes your struct fields and slice elements one by one, which is slow. This crate hashes the entire struct at once, which is much faster.
 
**Limitations:** The struct must be safe to view as a slice of bytes. This is enforced by requiring derived traits from either [`bytemuck`](https://crates.io/crates/bytemuck) or [`zerocopy`](https://crates.io/crates/zerocopy), at your option.

## Tell me more

This crate is inspired by the [excellent blog post](https://purplesyringa.moe/blog/thoughts-on-rust-hashing/) by [@purplesyringa](https://github.com/purplesyringa) (who is not affiliated with this crate). Check it out for an in-depth exploration of the issues with `#[derive(Hash)]` and the `Hash` trait in general.

We achieve better performance by:

1. Hashing the entire struct at once (as opposed to each field individually)
1. Dispatching to a sequence of primitive writes such as `hasher.write_u64` which is determined at compile time, padded where necessary (as opposed to using the slow variable-length codepath in the hashers)
1. Replicating the optimization `std` performs for `u8` and other primitive types in slices, so that e.g. `&[MyType(u8)]` can he hashed as fast as `&[u8]`. This applies to structs with multiple fields as well.

## Usage

For using the crate with `zerocopy` (recommended), see the docs on `derive_hash_fast_zerocopy!`

For using the crate with `bytemuck` (which puts more restrictions on your type), see the docs on `derive_hash_fast_bytemuck!`

## Benchmarks

Clone the repository and run `cargo bench`.

I've published the raw results from a run [here](https://shnatsel.github.io/derive_hash_benchmark_report/report/), but nothing beats benchmarks on your hardware and on your verstion of Rust compiler.

In my benchmarks this approach is faster than `#[derive(Hash)]` across the board, but there is one exception. If you are hashing a very short slice (64 bits or less) and you're using a function with a fast fixed-size path and slow variable-sized path (pretty much only `rustc_hash::FxHasher`), this approach may be slower. This crate is still dramatically faster for structs and longer slices even with `rustc_hash::FxHasher`. Whether this helps or hinders depends on the abundance of short slices in the data you're hashing.

## Why not improve the Rust compiler?

Right now the pass that expands the `#[derive(Hash)]` macro happens before the properties of the type required for this optimization are known. So this would require rather significant architectural changes.

Hopefully that will happen sooner or later, but for now there's this crate.
