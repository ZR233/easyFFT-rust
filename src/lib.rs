mod bindings;
mod error;
extern crate num;
extern crate ndarray;

use std::any::Any;
use std::ffi::c_void;
use std::os::windows::process::CommandExt;
use std::ptr::{null, null_mut};
use ndarray::{Array, Array1, ArrayBase, ArrayD, Dimension, RawData};
use num::{Complex, Num};
use num::complex::Complex32;
use crate::bindings::*;
pub use error::Error;
use error::handle_origin_err;

trait OriginPlan{
    unsafe fn execute(&mut self)->Result<(), Error>;
}

pub struct Plan<T: Num+ Clone> {
    data_in: Vec<T>,
    data_out: Vec<T>,
    shape: Vec<i32>,
    number_batches: usize,
    origin: Box<dyn OriginPlan> ,
}

struct OriginPlanNotInit{
}
impl OriginPlan for OriginPlanNotInit{
    unsafe fn execute(&mut self)->Result<(), Error> {
        Err(Error::NotInit)
    }
}


struct OriginPlanFloat{
    ptr: FFTPlanFloat
}

impl OriginPlan for OriginPlanFloat{
    unsafe fn execute(&mut self)->Result<(), Error> {
        let err = fft_planf_execute((&mut self.ptr) as *mut FFTPlanFloat);
        handle_origin_err(err)
    }
}


impl Drop for OriginPlanFloat{
    fn drop(&mut self) {
        unsafe {
            fft_close_planf((&mut self.ptr) as *mut FFTPlanFloat);
        }
    }
}

impl Plan<Complex32>{
    pub fn new_complex_float(
        shape: Vec<i32>,
        number_batches: usize,
        sign: FFT_SIGN,
        device: FFT_DEVICE,
    )->Result<Plan<Complex32>,Error>{
        let mut plan = Plan::new(
            shape, number_batches
        )?;
        unsafe {
            let mut plan_origin = FFTPlanFloat{
                config: FFTPlanConfig {
                    dim: plan.shape.len() as i32,
                    shape: plan.shape.as_ptr(),
                    number_batches: number_batches as i32,
                    sign,
                    device
                },
                ptr: null_mut()
            };

            let err = fft_planf_init(
                (&mut plan_origin ) as *mut FFTPlanFloat,
                plan.data_in.as_mut_ptr() as *mut [f32; 2],
                plan.data_in.len() as u64,
                plan.data_out.as_mut_ptr() as *mut [f32; 2],
                plan.data_out.len() as u64,
            );
            handle_origin_err(err)?;
            plan.origin = Box::new(OriginPlanFloat{
                ptr: plan_origin
            });
        }

        Ok(plan)
    }

    pub fn execute(&mut self) ->Result<(), Error>{

        unsafe {
            self.origin.execute()
        }
    }
}


impl<T:Num+ Clone> Plan<T> {
    fn new (
        shape: Vec<i32>,
        number_batches: usize,
    ) -> Result<Plan<T>, Error> {
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

}






#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use ndarray::{array, Array, ShapeBuilder};
    use num::complex::Complex32;
    use crate::bindings::{FFT_SIGN_FORWARD, FFT_DEVICE_CPU};
    use crate::Plan;
    use ndarray::prelude::*;
    #[test]
    fn it_works() {
        let mut out1;
        {
            // let mut  a =
            //     Array::<Complex32, _>::ones((2, 4));
            //
            //
            // let mut b =
            //     Array::<Complex32, _>::ones((2, 4));
            // println!("1");
            // let plan = Plan::new_complex_float(
            //     FFT_SIGN_FFT_SIGN_FORWARD,
            //     FFT_DEVICE_FFT_DEVICE_CPU,
            //     &mut a,
            //     &mut b,
            // );
            // println!("2");

            let mut plan = Plan::new_complex_float(
                vec![4],
                2,
                    FFT_SIGN_FORWARD,
                    FFT_DEVICE_CPU,
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
            plan.execute();
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
