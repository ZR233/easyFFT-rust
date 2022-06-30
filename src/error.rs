use std::fmt::{Debug, Display, Formatter};
use std::ops::AddAssign;
use crate::bindings;
use std::error::Error as StdError;
use std::ffi::CStr;
use std::result::Result as StdResult;

/// A result of a function that may return a `Error`.
pub type Result<T> = StdResult<T, Error>;


#[derive(Debug)]
pub enum Error{
    Unknown,
    InSize,
    OutSize,
    CL(String),
    DimTooBig,
    DeviceNotFound,
    VKFFT(String),
    NotInit,
    Vulkan(String),
    MallocFailed,
    OutOfDeviceMemory,
    NotSupportCL,
}


impl Error {

    pub fn strerror(&self) -> String {
        match self {
            Error::Unknown      => String::from("Unknown") ,
            Error::InSize           => String::from("input data size err"),
            Error::OutSize => String::from("output data size err"),
            Error::CL(msg)       => String::from("OpenCl Error ") + msg,
            Error::DimTooBig     => String::from("dim too big"),
            Error::DeviceNotFound => String::from("device not found"),
            Error::VKFFT(msg)         => String::from("VKFFT Error ") + msg,
            Error::NotInit      => String::from("NotInit"),
            Error::Vulkan(msg)         => String::from("VULKAN Error ") + msg,
            Error::MallocFailed => String::from("malloc failed"),
            Error::OutOfDeviceMemory => String::from("out of device memory"),
            Error::NotSupportCL => String::from("not support cl"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.strerror().as_str())
    }
}



pub unsafe fn handle_origin_err(result: *const  bindings::Result) ->Result<()>{
    let c_str: &CStr = unsafe { CStr::from_ptr((*result).msg) };
    let msg_str = c_str.to_str().unwrap();
    let msg = String::from(msg_str);

    match (*result).code {
        bindings::FFT_ERROR_CODE_OK=> return Ok(()),
        bindings::FFT_ERROR_CODE_OPEN_CL=> Err(Error::CL(msg)) ,
        bindings::FFT_ERROR_CODE_VULKAN=> Err(Error::Vulkan(msg)) ,
        bindings::FFT_ERROR_CODE_VKFFT=> Err(Error::VKFFT(msg)) ,
        bindings::FFT_ERROR_CODE_IN_SIZE=> Err(Error::InSize) ,
        bindings::FFT_ERROR_CODE_OUT_SIZE=> Err(Error::OutSize) ,
        bindings::FFT_ERROR_CODE_DIM_TOO_BIG=> Err(Error::DimTooBig) ,
        bindings::FFT_ERROR_CODE_DEVICE_NOT_FOUND=> Err(Error::DeviceNotFound) ,
        bindings::FFT_ERROR_CODE_MALLOC_FAILED=> Err(Error::MallocFailed) ,
        bindings::FFT_ERROR_CODE_OUT_OF_DEVICE_MEMORY=> Err(Error::OutOfDeviceMemory) ,
        bindings::FFT_ERROR_CODE_NOT_SUPPORT_CL=> Err(Error::NotSupportCL) ,
        _ => Err(Error::Unknown)
    }

}