/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

macro_rules ! cpp { ( ) => { } ; ( # include $ filename : tt $ ( $ rest : tt ) * ) => { cpp ! { $ ( $ rest ) * } } ; ( { $ ( $ code : tt ) * } $ ( $ rest : tt ) * ) => { cpp ! { $ ( $ rest ) * } } ; }
cpp! { # include "generate-inline.hpp" }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo3barEv"]
    pub fn Foo_bar() -> ::std::os::raw::c_int;
}
impl Foo {
    #[inline]
    pub unsafe fn bar() -> ::std::os::raw::c_int {
        Foo_bar()
    }
}
extern "C" {
    #[link_name = "\u{1}_Z3foov"]
    pub fn foo() -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZL3barii"]
    pub fn bar(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
cpp! { { int _ZL3barii ( const int x , const int y ) { return bar ( x , y ) ; } } }
