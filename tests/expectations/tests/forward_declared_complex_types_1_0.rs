/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo_empty {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo_empty() {
    assert_eq!(
        ::std::mem::size_of::<Foo_empty>(),
        1usize,
        concat!("Size of: ", stringify!(Foo_empty))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo_empty>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo_empty))
    );
}
impl Clone for Foo_empty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub f: *mut Foo,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { &(*(0 as *const Bar)).f as *const _ as usize },
        0usize,
        concat!("Alignment of field: ", stringify!(Bar), "::", stringify!(f))
    );
}
impl Clone for Bar {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z10baz_structP3Foo"]
    pub fn baz_struct(f: *mut Foo);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Union {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_unionP5Union"]
    pub fn baz_union(u: *mut Union);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quux {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_classP4Quux"]
    pub fn baz_class(q: *mut Quux);
}
