/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


extern "C" {
    #[link_name = "_Z3fooPKcz"]
    pub fn foo(fmt: *const ::std::os::raw::c_char, ...);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 1usize);
    assert_eq! (::std::mem::align_of::<Bar>() , 1usize);
}
extern "C" {
    #[link_name = "_ZN3Bar3fooEPKcz"]
    pub fn Bar_foo(this: *mut Bar, fmt: *const ::std::os::raw::c_char, ...);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
