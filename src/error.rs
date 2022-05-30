use std::fmt::{Debug, Display, Formatter};
use std::ops::AddAssign;
use crate::bindings::*;

pub enum  Error{
    Unknown,
    InSize,
    OutSize,
    CL,
    DimTooBig,
    NoClDevice,
    VKFFT,
    NotInit,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error{

}


pub fn handle_origin_err(err: FFT_ERROR_CODE) ->Result<(), Error>{
    match err {
        FFT_ERROR_CODE_OK=> Ok(()),
        FFT_ERROR_CODE_IN_SIZE=> Err(Error::InSize) ,
        FFT_ERROR_CODE_OUT_SIZE=> Err(Error::OutSize),
        FFT_ERROR_CODE_CL=> Err(Error::CL),
        FFT_ERROR_CODE_DIM_TOO_BIG=> Err(Error::DimTooBig),
        FFT_ERROR_CODE_NO_CL_DEVICE=> Err(Error::NoClDevice),
        FFT_ERROR_CODE_VKFFT=> Err(Error::VKFFT),
        _ => Err(Error::Unknown)
    }

}