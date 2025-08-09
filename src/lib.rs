use bytemuck::NoUninit;

#[repr(C)]
#[derive(Clone, Copy, NoUninit)]
struct Foo {
    a: u16,
    b: u16,
    c: char,
}

use core::hash::{Hash, Hasher};
impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( bytemuck::bytes_of(self));
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        state.write(bytemuck::cast_slice(data) );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
