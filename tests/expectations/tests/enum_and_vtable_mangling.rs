/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub const match_: _bindgen_ty_1 = _bindgen_ty_1::match_;
pub const whatever_else: _bindgen_ty_1 = _bindgen_ty_1::whatever_else;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 { match_ = 0, whatever_else = 1, }
#[repr(C)]
pub struct C__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 16usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
    assert_eq! (unsafe { & ( * ( 0 as * const C ) ) . i as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( i ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
