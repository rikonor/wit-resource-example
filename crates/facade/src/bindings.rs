// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod local {
        pub mod build {
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod init {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_init_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::init();
                }
                pub trait Guest {
                    fn init() -> ();
                }
                #[doc(hidden)]
                macro_rules! __export_local_build_init_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "local:build/init#init")] unsafe extern "C" fn export_init() {
                        unsafe { $($path_to_types)*:: _export_init_cabi::<$ty > () } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_local_build_init_cabi;
            }
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod build {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_build_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::build();
                    let ptr1 = (&raw mut _RET_AREA.0).cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1
                                .add(2 * ::core::mem::size_of::<*const u8>())
                                .cast::<usize>() = len2;
                            *ptr1
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_build<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<*mut u8>();
                            let l2 = *arg0
                                .add(2 * ::core::mem::size_of::<*const u8>())
                                .cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn build() -> Result<(), _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_local_build_build_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "local:build/build#build")] unsafe extern "C" fn export_build()
                        -> * mut u8 { unsafe { $($path_to_types)*::
                        _export_build_cabi::<$ty > () } } #[unsafe (export_name =
                        "cabi_post_local:build/build#build")] unsafe extern "C" fn
                        _post_return_build(arg0 : * mut u8,) { unsafe {
                        $($path_to_types)*:: __post_return_build::<$ty > (arg0) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_local_build_build_cabi;
                #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
                #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
                struct _RetArea(
                    [::core::mem::MaybeUninit<
                        u8,
                    >; 3 * ::core::mem::size_of::<*const u8>()],
                );
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 3
                        * ::core::mem::size_of::<*const u8>()],
                );
            }
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod register {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_register_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::register(_rt::string_lift(bytes0));
                    let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2
                                .add(2 * ::core::mem::size_of::<*const u8>())
                                .cast::<usize>() = len3;
                            *ptr2
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_register<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<*mut u8>();
                            let l2 = *arg0
                                .add(2 * ::core::mem::size_of::<*const u8>())
                                .cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn register(name: _rt::String) -> Result<(), _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_local_build_register_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "local:build/register#register")] unsafe extern "C" fn
                        export_register(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
                        unsafe { $($path_to_types)*:: _export_register_cabi::<$ty >
                        (arg0, arg1) } } #[unsafe (export_name =
                        "cabi_post_local:build/register#register")] unsafe extern "C" fn
                        _post_return_register(arg0 : * mut u8,) { unsafe {
                        $($path_to_types)*:: __post_return_register::<$ty > (arg0) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_local_build_register_cabi;
                #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
                #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
                struct _RetArea(
                    [::core::mem::MaybeUninit<
                        u8,
                    >; 3 * ::core::mem::size_of::<*const u8>()],
                );
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 3
                        * ::core::mem::size_of::<*const u8>()],
                );
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_facade_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::local::build::init::__export_local_build_init_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::local::build::init); $($path_to_types_root)*::
        exports::local::build::build::__export_local_build_build_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::local::build::build);
        $($path_to_types_root)*::
        exports::local::build::register::__export_local_build_register_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::local::build::register);
    };
}
#[doc(inline)]
pub(crate) use __export_facade_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:local:build:facade:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 294] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa9\x01\x01A\x02\x01\
A\x06\x01B\x02\x01@\0\x01\0\x04\0\x04init\x01\0\x04\0\x10local:build/init\x05\0\x01\
B\x03\x01j\0\x01s\x01@\0\0\0\x04\0\x05build\x01\x01\x04\0\x11local:build/build\x05\
\x01\x01B\x03\x01j\0\x01s\x01@\x01\x04names\0\0\x04\0\x08register\x01\x01\x04\0\x14\
local:build/register\x05\x02\x04\0\x12local:build/facade\x04\0\x0b\x0c\x01\0\x06\
facade\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.227\
.1\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
