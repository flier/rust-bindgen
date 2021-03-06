/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
}
extern "C" {
    #[link_name = "_ZN3BarC1Ev"]
    pub fn Bar_Bar(this: *mut Bar);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
impl Bar {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Bar_Bar(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
