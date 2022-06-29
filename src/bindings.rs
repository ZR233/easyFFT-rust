/* automatically generated by rust-bindgen 0.59.2 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub const FFT_ERROR_CODE_OK: FFT_ERROR_CODE = 0;
pub const FFT_ERROR_CODE_OPEN_CL: FFT_ERROR_CODE = 1;
pub const FFT_ERROR_CODE_VULKAN: FFT_ERROR_CODE = 2;
pub const FFT_ERROR_CODE_VKFFT: FFT_ERROR_CODE = 3;
pub const FFT_ERROR_CODE_IN_SIZE: FFT_ERROR_CODE = 4;
pub const FFT_ERROR_CODE_OUT_SIZE: FFT_ERROR_CODE = 5;
pub const FFT_ERROR_CODE_DIM_TOO_BIG: FFT_ERROR_CODE = 6;
pub const FFT_ERROR_CODE_DEVICE_NOT_FOUND: FFT_ERROR_CODE = 7;
pub const FFT_ERROR_CODE_MALLOC_FAILED: FFT_ERROR_CODE = 8;
pub const FFT_ERROR_CODE_OUT_OF_DEVICE_MEMORY: FFT_ERROR_CODE = 9;
pub const FFT_ERROR_CODE_NOT_SUPPORT_CL: FFT_ERROR_CODE = 10;
pub type FFT_ERROR_CODE = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Result {
    pub code: FFT_ERROR_CODE,
    pub msg: *mut ::std::os::raw::c_char,
    pub msg_size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Result() {
    assert_eq!(
        ::std::mem::size_of::<Result>(),
        24usize,
        concat!("Size of: ", stringify!(Result))
    );
    assert_eq!(
        ::std::mem::align_of::<Result>(),
        8usize,
        concat!("Alignment of ", stringify!(Result))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Result>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Result),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Result>())).msg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Result),
            "::",
            stringify!(msg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Result>())).msg_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Result),
            "::",
            stringify!(msg_size)
        )
    );
}
pub const FFT_SIGN_FORWARD: FFT_SIGN = 0;
pub const FFT_SIGN_BACKWARD: FFT_SIGN = 1;
pub type FFT_SIGN = ::std::os::raw::c_int;
pub const FFT_DEVICE_CPU: FFT_DEVICE = 0;
pub const FFT_DEVICE_GPU: FFT_DEVICE = 1;
pub type FFT_DEVICE = ::std::os::raw::c_int;
pub type ComplexF = [f32; 2usize];
pub type ComplexD = [f64; 2usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFTPlanConfig {
    pub dim: i32,
    pub shape: *const i32,
    pub number_batches: i32,
    pub sign: FFT_SIGN,
    pub device: FFT_DEVICE,
}
#[test]
fn bindgen_test_layout_FFTPlanConfig() {
    assert_eq!(
        ::std::mem::size_of::<FFTPlanConfig>(),
        32usize,
        concat!("Size of: ", stringify!(FFTPlanConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<FFTPlanConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(FFTPlanConfig))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanConfig>())).dim as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanConfig),
            "::",
            stringify!(dim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanConfig>())).shape as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanConfig),
            "::",
            stringify!(shape)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanConfig>())).number_batches as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanConfig),
            "::",
            stringify!(number_batches)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanConfig>())).sign as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanConfig),
            "::",
            stringify!(sign)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanConfig>())).device as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanConfig),
            "::",
            stringify!(device)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFTPlanFloat {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_FFTPlanFloat() {
    assert_eq!(
        ::std::mem::size_of::<FFTPlanFloat>(),
        4usize,
        concat!("Size of: ", stringify!(FFTPlanFloat))
    );
    assert_eq!(
        ::std::mem::align_of::<FFTPlanFloat>(),
        1usize,
        concat!("Alignment of ", stringify!(FFTPlanFloat))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFTPlanDouble {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_FFTPlanDouble() {
    assert_eq!(
        ::std::mem::size_of::<FFTPlanDouble>(),
        4usize,
        concat!("Size of: ", stringify!(FFTPlanDouble))
    );
    assert_eq!(
        ::std::mem::align_of::<FFTPlanDouble>(),
        1usize,
        concat!("Alignment of ", stringify!(FFTPlanDouble))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFTPlanDoubleR2C {
    pub config: FFTPlanConfig,
    pub ptr: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_FFTPlanDoubleR2C() {
    assert_eq!(
        ::std::mem::size_of::<FFTPlanDoubleR2C>(),
        40usize,
        concat!("Size of: ", stringify!(FFTPlanDoubleR2C))
    );
    assert_eq!(
        ::std::mem::align_of::<FFTPlanDoubleR2C>(),
        8usize,
        concat!("Alignment of ", stringify!(FFTPlanDoubleR2C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanDoubleR2C>())).config as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanDoubleR2C),
            "::",
            stringify!(config)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FFTPlanDoubleR2C>())).ptr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FFTPlanDoubleR2C),
            "::",
            stringify!(ptr)
        )
    );
}
extern "C" {
    pub fn fft_new_result() -> *mut Result;
}
extern "C" {
    pub fn fft_release_result(result: *mut Result);
}
extern "C" {
    pub fn fft_planf_init(
        config: FFTPlanConfig,
        in_complex: *mut ComplexF,
        in_size: u64,
        out_complex: *mut ComplexF,
        out_size: u64,
        result: *mut Result,
    ) -> *mut FFTPlanFloat;
}
extern "C" {
    pub fn fft_planf_device_name(
        plan: *mut FFTPlanFloat,
        name: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        result: *mut Result,
    );
}
extern "C" {
    pub fn fft_close_planf(plan: *mut FFTPlanFloat);
}
extern "C" {
    pub fn fft_close_plan(plan: *mut FFTPlanDouble);
}
extern "C" {
    pub fn fft_planf_execute(plan: *mut FFTPlanFloat, result: *mut Result);
}
