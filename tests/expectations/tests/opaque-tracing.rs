/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Container {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[test]
fn bindgen_test_layout_Container() {
    assert_eq!(::std::mem::size_of::<Container>() , 8usize , concat ! (
               "Size of: " , stringify ! ( Container ) ));
    assert_eq! (::std::mem::align_of::<Container>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( Container ) ));
}
impl Clone for Container {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "_Z3fooP9Container"]
    pub fn foo(c: *mut Container);
}
