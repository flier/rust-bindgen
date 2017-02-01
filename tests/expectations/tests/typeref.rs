/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nsFoo {
    pub mBar: mozilla_StyleShapeSource<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_nsFoo() {
    assert_eq!(::std::mem::size_of::<nsFoo>() , 8usize);
    assert_eq! (::std::mem::align_of::<nsFoo>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const nsFoo ) ) . mBar as * const _ as usize }
                , 0usize);
}
impl Clone for nsFoo {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct mozilla_FragmentOrURL {
    pub mIsLocalRef: bool,
}
#[test]
fn bindgen_test_layout_mozilla_FragmentOrURL() {
    assert_eq!(::std::mem::size_of::<mozilla_FragmentOrURL>() , 1usize);
    assert_eq! (::std::mem::align_of::<mozilla_FragmentOrURL>() , 1usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const mozilla_FragmentOrURL ) ) . mIsLocalRef
                as * const _ as usize } , 0usize);
}
impl Clone for mozilla_FragmentOrURL {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct mozilla_Position {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_mozilla_Position() {
    assert_eq!(::std::mem::size_of::<mozilla_Position>() , 1usize);
    assert_eq! (::std::mem::align_of::<mozilla_Position>() , 1usize);
}
impl Clone for mozilla_Position {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mozilla_StyleShapeSource<ReferenceBox> {
    pub __bindgen_anon_1: mozilla_StyleShapeSource__bindgen_ty_1<ReferenceBox>,
    pub _phantom_0: ::std::marker::PhantomData<ReferenceBox>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mozilla_StyleShapeSource__bindgen_ty_1<ReferenceBox> {
    pub mPosition: __BindgenUnionField<*mut mozilla_Position>,
    pub mFragmentOrURL: __BindgenUnionField<*mut mozilla_FragmentOrURL>,
    pub bindgen_union_field: u64,
    pub _phantom_0: ::std::marker::PhantomData<ReferenceBox>,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub mFoo: *mut nsFoo,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 8usize);
    assert_eq! (::std::mem::align_of::<Bar>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const Bar ) ) . mFoo as * const _ as usize } ,
                0usize);
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
