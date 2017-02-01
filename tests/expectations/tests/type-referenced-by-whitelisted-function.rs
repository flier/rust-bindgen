/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct dl_phdr_info {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_dl_phdr_info() {
    assert_eq!(::std::mem::size_of::<dl_phdr_info>() , 4usize);
    assert_eq! (::std::mem::align_of::<dl_phdr_info>() , 4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const dl_phdr_info ) ) . x as * const _ as
                usize } , 0usize);
}
impl Clone for dl_phdr_info {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn dl_iterate_phdr(arg1: *mut dl_phdr_info) -> ::std::os::raw::c_int;
}
