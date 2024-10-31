use std::mem::{align_of, size_of};

#[repr(C)]
struct MyStruct {
    a: u32,
    b: u64,
    c: u8,
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe {
            let base = std::ptr::null::<$ty>();
            let field = &(*base).$field;
            (field as *const _ as usize) - (base as *const _ as usize)
        }
    };
}

fn main() {
    println!("Size of MyStruct: {}", size_of::<MyStruct>());
    println!("Alignment of MyStruct: {}", align_of::<MyStruct>());
    println!("Offset of a: {}", offset_of!(MyStruct, a));
    println!("Offset of b: {}", offset_of!(MyStruct, b));
    println!("Offset of c: {}", offset_of!(MyStruct, c));
}
