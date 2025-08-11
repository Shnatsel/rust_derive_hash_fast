#![cfg_attr(not(test), no_std)]

/// Derives a fast `Hash` implementation for `bytemuck` types.
///
/// This macro implements [std::hash::Hash] by calling `.bytes_of()` on the
/// struct, which leverages `bytemuck` to get a byte representation of the
/// type for hashing.
///
/// # Examples
///
/// ```
/// use derive_hash_fast::derive_hash_fast_bytemuck;
/// use bytemuck::NoUninit;
///
/// // Define the struct that we want to hash
/// #[repr(C)] // required by bytemuck
/// #[derive(Eq, PartialEq, Clone, Copy, NoUninit)]
/// struct MyStruct {
///     a: bool,
///     b: u8,
///     c: u16,
/// }
///
/// // Derive the fast Hash implementation
/// derive_hash_fast_bytemuck!(MyStruct);
///
/// // Use the struct in a HashSet
/// let mut hashset = std::collections::HashSet::new();
/// hashset.insert(MyStruct{a: true, b: 2, c: 3});
/// assert!(hashset.contains(&MyStruct{a: true, b: 2, c: 3}));
/// ```
#[macro_export]
macro_rules! derive_hash_fast_bytemuck {
    ($T:ty) => {
        impl core::hash::Hash for $T {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let bytes = ::bytemuck::bytes_of(self);
                $crate::write_to_optimal_hasher_function::<{core::mem::size_of::<$T>()}>(bytes, state);
            }

            fn hash_slice<H: core::hash::Hasher>(data: &[Self], state: &mut H)
            where
                Self: Sized,
            {
                state.write(::bytemuck::cast_slice(data));
            }
        }
    };
}

/// Derives a fast `Hash` implementation for `zerocopy` types.
///
/// This macro implements [std::hash::Hash] by calling `.as_bytes()` on the
/// struct, which leverages `zerocopy` to get a byte representation of the
/// type for hashing.
///
/// # Examples
///
/// ```
/// use derive_hash_fast::derive_hash_fast_zerocopy;
/// use zerocopy::{Immutable, IntoBytes};
///
/// // Define the struct that we want to hash
/// #[derive(Eq, PartialEq, Immutable, IntoBytes)]
/// struct MyStruct {
///     a: bool,
///     b: u8,
///     c: u16,
/// }
///
/// // Derive the fast Hash implementation
/// derive_hash_fast_zerocopy!(MyStruct);
///
/// // Use the struct in a HashSet
/// let mut hashset = std::collections::HashSet::new();
/// hashset.insert(MyStruct{a: true, b: 2, c: 3});
/// assert!(hashset.contains(&MyStruct{a: true, b: 2, c: 3}));
/// ```
#[macro_export]
macro_rules! derive_hash_fast_zerocopy {
    ($T:ty) => {
        impl core::hash::Hash for $T {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let bytes = ::zerocopy::IntoBytes::as_bytes(self);
                $crate::write_to_optimal_hasher_function::<{core::mem::size_of::<$T>()}>(bytes, state);
            }

            fn hash_slice<H: core::hash::Hasher>(data: &[Self], state: &mut H)
            where
                Self: Sized,
            {
                state.write(::zerocopy::transmute_ref!(data));
            }
        }
    };
}

use core::hash::Hasher;

#[doc(hidden)]
#[inline]
pub fn write_to_optimal_hasher_function<const B: usize>(bytes: &[u8], state: &mut impl Hasher) {
    assert!(bytes.len() == B);
    // Dispatch to a specialized hashing function for the struct's size, if one is available.
    // This match incurs no runtime overhead in release mode because it matches on a constant.
    match B {
        1 => state.write_u8(u8::from_ne_bytes(bytes.try_into().unwrap())),
        2 => state.write_u16(u16::from_ne_bytes(bytes.try_into().unwrap())),
        3 => state.write_u32(pad_to_u32::<3>(bytes.try_into().unwrap())),
        4 => state.write_u32(u32::from_ne_bytes(bytes.try_into().unwrap())),
        5..=7 => state.write_u64(pad_to_u64::<B>(bytes.try_into().unwrap())),
        8 => state.write_u64(u64::from_ne_bytes(bytes.try_into().unwrap())),
        9..=15 => state.write_u128(pad_to_u128::<B>(bytes.try_into().unwrap())),
        16 => state.write_u128(u128::from_ne_bytes(bytes.try_into().unwrap())),
        17..=64 => hash_padded_large::<B>(bytes.try_into().unwrap(), state),
        // TODO: const generic optimiation to lower into several u128 writes with the final one padded
        _ => state.write(bytes),
    }
}

#[inline]
fn pad_to_u32<const N: usize>(bytes: &[u8; N]) -> u32 {
    let mut padded_bytes = [0u8; core::mem::size_of::<u32>()];
    padded_bytes[..N].copy_from_slice(bytes);
    u32::from_ne_bytes(padded_bytes.try_into().unwrap())
}

#[inline]
fn pad_to_u64<const N: usize>(bytes: &[u8; N]) -> u64 {
    let mut padded_bytes = [0u8; core::mem::size_of::<u64>()];
    padded_bytes[..N].copy_from_slice(bytes);
    u64::from_ne_bytes(padded_bytes.try_into().unwrap())
}

#[inline]
fn pad_to_u128<const N: usize>(bytes: &[u8; N]) -> u128 {
    let mut padded_bytes = [0u8; core::mem::size_of::<u128>()];
    padded_bytes[..N].copy_from_slice(bytes);
    u128::from_ne_bytes(padded_bytes.try_into().unwrap())
}

#[inline]
fn hash_padded_large<const N: usize>(bytes: &[u8; N], state: &mut impl Hasher) {
    const SIZEOF_U128: usize = core::mem::size_of::<u128>();
    let chunks_iter = bytes.chunks_exact(SIZEOF_U128);
    let remainder = chunks_iter.remainder();
    for chunk in chunks_iter {
        state.write_u128(u128::from_ne_bytes(chunk.try_into().unwrap()))
    }
    // pad to either u64 or u128 to limit the amount of extra work performed
    // compared to always padding to u128.
    // We don't want the full write_to_optimal_hasher_function() here
    // because it regresses performance on the faster hashes,
    // and only helps really naive implementations like std::DefaultHasher
    match remainder.len() {
        0 => (), // nothing to do
        1..=7 => {
            let mut padded_bytes = [0u8; core::mem::size_of::<u64>()];
            padded_bytes[..remainder.len()].copy_from_slice(remainder);
            state.write_u64(u64::from_ne_bytes(padded_bytes.try_into().unwrap()))
        }
        8 => state.write_u64(u64::from_ne_bytes(remainder.try_into().unwrap())),
        9..=15 => {
            let mut padded_bytes = [0u8; core::mem::size_of::<u128>()];
            padded_bytes[..remainder.len()].copy_from_slice(remainder);
            state.write_u128(u128::from_ne_bytes(padded_bytes.try_into().unwrap()))
        },
        SIZEOF_U128.. => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy, bytemuck::NoUninit)]
    struct FooB {
        a: u16,
        b: u16,
        c: char,
    }

    derive_hash_fast_bytemuck!(FooB);

    fn hash_struct_bytemuck(val: FooB) -> u64 {
        use std::hash::{DefaultHasher, Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_bytemuck() {
        let test_struct_1 = FooB {
            a: 5,
            b: 10,
            c: 'a',
        };
        let test_struct_2 = FooB {
            a: 5,
            b: 500,
            c: 'a',
        };

        let hash_1 = hash_struct_bytemuck(test_struct_1);
        let hash_1_again = hash_struct_bytemuck(test_struct_1);
        let hash_2 = hash_struct_bytemuck(test_struct_2);

        assert_eq!(hash_1, hash_1_again);
        assert_ne!(hash_1, hash_2);
    }

    #[derive(Clone, zerocopy::Immutable, zerocopy::IntoBytes)]
    struct FooZ {
        a: u16,
        b: u16,
        c: char,
    }

    derive_hash_fast_zerocopy!(FooZ);

    fn hash_struct_zerocopy(val: FooZ) -> u64 {
        use std::hash::{DefaultHasher, Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_zerocopy() {
        let test_struct_1 = FooZ {
            a: 5,
            b: 10,
            c: 'a',
        };
        let test_struct_2 = FooZ {
            a: 5,
            b: 500,
            c: 'a',
        };

        let hash_1 = hash_struct_zerocopy(test_struct_1.clone());
        let hash_1_again = hash_struct_zerocopy(test_struct_1);
        let hash_2 = hash_struct_zerocopy(test_struct_2);

        assert_eq!(hash_1, hash_1_again);
        assert_ne!(hash_1, hash_2);
    }
}
