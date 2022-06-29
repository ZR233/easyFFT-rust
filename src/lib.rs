mod bindings;
mod error;
extern crate num;
extern crate ndarray;

use std::any::Any;
use std::ffi::c_void;
use std::ptr::{null, null_mut};
use ndarray::{Array, Array1, ArrayBase, ArrayD, Dimension, RawData};
use num::{Complex, Num};
use num::complex::Complex32;
pub use error::Result;
use error::handle_origin_err;
pub use error::Error;



trait OriginPlan{
    unsafe fn execute(&mut self)->Result<()>;
}

pub struct Plan<T: Num+ Clone + Copy> {
    data_in: Vec<T>,
    pub data_out: Vec<T>,
    shape: Vec<i32>,
    number_batches: usize,
    origin: Box<dyn OriginPlan> ,
}

pub enum Sign{
    Forward,
    Backward
}
pub enum Device{
    CPU,
    GPU
}


struct OriginPlanNotInit{}

impl OriginPlan for OriginPlanNotInit{
    unsafe fn execute(&mut self)->Result<()> {
        Err(Error::NotInit)
    }
}


struct OriginPlanFloat{
    ptr :  *mut bindings::FFTPlanFloat
}

impl OriginPlan for OriginPlanFloat{
    unsafe fn execute(&mut self)->Result<()> {
        let result = bindings::fft_new_result();
        bindings::fft_planf_execute(
            self.ptr,
            result);
        let r = handle_origin_err(result);
        bindings::fft_release_result(result);
        r
    }
}


impl Drop for OriginPlanFloat{
    fn drop(&mut self) {
        unsafe {
            bindings::fft_close_planf(self.ptr);
        }
    }
}

impl Into<bindings::FFT_SIGN> for Sign {
    fn into(self) -> bindings::FFT_SIGN {
        match self {
            Sign::Forward => bindings::FFT_SIGN_FORWARD,
            Sign::Backward => bindings::FFT_SIGN_BACKWARD
        }
    }
}
impl Into<bindings::FFT_DEVICE> for Device {
    fn into(self) -> bindings::FFT_SIGN {
        match self {
            Device::CPU => bindings::FFT_DEVICE_CPU,
            Device::GPU => bindings::FFT_DEVICE_GPU
        }
    }
}



impl Plan<Complex32>{
    pub fn new_complex_float(
        shape: Vec<i32>,
        number_batches: usize,
        sign: Sign,
        device: Device,
    )->Result<Plan<Complex32>>{
        let mut plan = Plan::new(
            shape, number_batches
        )?;
        unsafe {
            let config = bindings::FFTPlanConfig {
                dim: plan.shape.len() as i32,
                shape: plan.shape.as_ptr(),
                number_batches: number_batches as i32,
                sign: sign.into(),
                device: device.into()
            };

            let result = bindings::fft_new_result();
            let mut plan_origin = bindings::fft_planf_init(
                config,
                plan.data_in.as_mut_ptr() as *mut [f32; 2],
                plan.data_in.len() as u64,
                plan.data_out.as_mut_ptr() as *mut [f32; 2],
                plan.data_out.len() as u64,
                result
            );
            let r = handle_origin_err(result);
            bindings::fft_release_result(result);
            r?;
            plan.origin = Box::new(OriginPlanFloat{
                ptr: plan_origin
            });
        }

        Ok(plan)
    }

    pub fn execute(&mut self) ->Result<()>{
        unsafe {
            self.origin.execute()
        }
    }
}


impl<T:Num+ Clone + Copy> Plan<T> {
    fn new (
        shape: Vec<i32>,
        number_batches: usize,
    ) -> Result<Plan<T>> {
        let mut data_in = Vec::new();
        let mut data_out = Vec::new();
        let mut size = number_batches;
        for len in &shape {
            size *= (*len) as usize;
        }

        data_in.resize(size, T::zero());
        data_out.resize(size, T::zero());

        
        Ok(Plan{
            data_in,
            data_out,
            shape,
            number_batches,
            origin: Box::new(OriginPlanNotInit{}),
        }) 
    }
    pub fn copy_in(&mut self, data: &[T]){
        self.data_in.copy_from_slice(data);
    }
}






#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use ndarray::{array, Array, ShapeBuilder};
    use num::complex::Complex32;
    use crate::bindings::{FFT_SIGN_FORWARD, FFT_DEVICE_CPU};
    use crate::{Device, Plan, Sign};
    use ndarray::prelude::*;
    #[test]
    fn it_works() {
        let mut out1;
        {
            let mut plan = Plan::new_complex_float(
                vec![4],
                2,
                    Sign::Forward,
                    Device::GPU,
            ).unwrap();


            {
                for i in 0..4 {
                    plan.data_in[i] = Complex32::new(i as f32, -(i as f32));
                }
                for i in 4..8 {
                    let t = i as f32;
                    plan.data_in[i] = Complex32::new(-t, t);
                }
            }

            println!("3");
            plan.execute().expect("execute fail");
            println!("4");
            let out = plan.data_out.clone();

            println!("{:?}", out.len());

            out1 = plan.data_out.clone();
        }

        assert_eq!(out1,  vec![
            Complex32::new(6.0, -6.0), Complex32::new(0.0, 4.0), Complex32::new(-2.0, 2.0), Complex32::new(-4.0, 0.0),
            Complex32::new(-22.0, 22.0), Complex32::new(0.0, -4.0), Complex32::new(2.0, -2.0), Complex32::new(4.0, 0.0)
        ]);


    }
}
