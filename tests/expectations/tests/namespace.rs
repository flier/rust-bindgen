/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    extern "C" {
        #[link_name = "_Z9top_levelv"]
        pub fn top_level();
    }
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type whatever_int_t = ::std::os::raw::c_int;
        extern "C" {
            #[link_name = "_ZN8whatever11in_whateverEv"]
            pub fn in_whatever();
        }
    }
    pub mod _bindgen_mod_id_13 {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "_ZN12_GLOBAL__N_13fooEv"]
            pub fn foo();
        }
        #[repr(C)]
        #[derive(Debug, Copy)]
        pub struct A {
            pub b: root::whatever::whatever_int_t,
        }
        #[test]
        fn bindgen_test_layout_A() {
            assert_eq!(::std::mem::size_of::<A>() , 4usize);
            assert_eq! (::std::mem::align_of::<A>() , 4usize);
            assert_eq! (unsafe {
                        & ( * ( 0 as * const A ) ) . b as * const _ as usize }
                        , 0usize);
        }
        extern "C" {
            #[link_name = "_ZN12_GLOBAL__N_11A20lets_hope_this_worksEv"]
            pub fn A_lets_hope_this_works(this:
                                              *mut root::_bindgen_mod_id_13::A)
             -> ::std::os::raw::c_int;
        }
        impl Clone for A {
            fn clone(&self) -> Self { *self }
        }
        impl A {
            #[inline]
            pub unsafe fn lets_hope_this_works(&mut self)
             -> ::std::os::raw::c_int {
                A_lets_hope_this_works(&mut *self)
            }
        }
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct C<T> {
        pub _base: root::_bindgen_mod_id_13::A,
        pub m_c: T,
        pub m_c_ptr: *mut T,
        pub m_c_arr: [T; 10usize],
    }
    pub mod w {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type whatever_int_t = ::std::os::raw::c_uint;
        #[repr(C)]
        #[derive(Debug)]
        pub struct D<T> {
            pub m_c: root::C<T>,
        }
        extern "C" {
            #[link_name = "_ZN1w3hehEv"]
            pub fn heh() -> root::w::whatever_int_t;
        }
        extern "C" {
            #[link_name = "_ZN1w3fooEv"]
            pub fn foo() -> root::C<::std::os::raw::c_int>;
        }
        extern "C" {
            #[link_name = "_ZN1w4barrEv"]
            pub fn barr() -> root::C<f32>;
        }
    }
}
