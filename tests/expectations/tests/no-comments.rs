/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Foo {
    pub s: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 4usize);
    assert_eq! (::std::mem::align_of::<Foo>() , 4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const Foo ) ) . s as * const _ as usize } ,
                0usize);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
