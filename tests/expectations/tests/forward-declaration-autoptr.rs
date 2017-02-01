/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub m_inner: *mut T,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub m_member: RefPtr<Foo>,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 8usize);
    assert_eq! (::std::mem::align_of::<Bar>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const Bar ) ) . m_member as * const _ as usize
                } , 0usize);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
