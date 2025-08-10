#[macro_export]
macro_rules! derive_hash_fast_bytemuck {
    ($T:ty) => {
        impl core::hash::Hash for $T {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let bytes = ::bytemuck::bytes_of(self);
                $crate::write_to_optimal_hasher_function(bytes, state);
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

#[macro_export]
macro_rules! derive_hash_fast_zerocopy {
    ($T:ty) => {
        impl core::hash::Hash for $T {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let bytes = ::zerocopy::IntoBytes::as_bytes(self);
                $crate::write_to_optimal_hasher_function(bytes, state);
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

#[doc(hidden)]
#[inline]
pub fn write_to_optimal_hasher_function<H: core::hash::Hasher>(bytes: &[u8], state: &mut H) {
    // Dispatch to a specialized hashing function for the struct's size, if one is available.
    // This match incurs no runtime overhead in release mode because it matches on a constant.
    match bytes.len() {
        1 => state.write_u8(u8::from_ne_bytes(bytes.try_into().unwrap())),
        2 => state.write_u16(u16::from_ne_bytes(bytes.try_into().unwrap())),
        3 => state.write_u32(pad_to_u32::<3>(bytes.try_into().unwrap())),
        4 => state.write_u32(u32::from_ne_bytes(bytes.try_into().unwrap())),
        5 => state.write_u64(pad_to_u64::<5>(bytes.try_into().unwrap())),
        6 => state.write_u64(pad_to_u64::<6>(bytes.try_into().unwrap())),
        7 => state.write_u64(pad_to_u64::<7>(bytes.try_into().unwrap())),
        8 => state.write_u64(u64::from_ne_bytes(bytes.try_into().unwrap())),
        9 => state.write_u128(pad_to_u128::<9>(bytes.try_into().unwrap())),
        10 => state.write_u128(pad_to_u128::<10>(bytes.try_into().unwrap())),
        11 => state.write_u128(pad_to_u128::<11>(bytes.try_into().unwrap())),
        12 => state.write_u128(pad_to_u128::<12>(bytes.try_into().unwrap())),
        13 => state.write_u128(pad_to_u128::<13>(bytes.try_into().unwrap())),
        14 => state.write_u128(pad_to_u128::<14>(bytes.try_into().unwrap())),
        15 => state.write_u128(pad_to_u128::<15>(bytes.try_into().unwrap())),
        16 => state.write_u128(u128::from_ne_bytes(bytes.try_into().unwrap())),
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
