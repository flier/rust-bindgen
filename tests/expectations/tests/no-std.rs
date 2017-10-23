/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![no_std]
mod libc {
    pub type c_int = i32;
    pub enum c_void {}
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub bar: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::core::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::core::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).a as *const _ as usize },
        0usize,
        concat!("Alignment of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).b as *const _ as usize },
        4usize,
        concat!("Alignment of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bar as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(foo),
            "::",
            stringify!(bar)
        )
    );
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
