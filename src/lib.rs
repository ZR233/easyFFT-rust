mod bindings;
extern crate num;
extern crate ndarray;

use std::any::Any;
use std::os::windows::process::CommandExt;
use std::ptr::null;
use ndarray::{Array, Array1, ArrayBase, ArrayD, Dimension, RawData};
use num::{Complex, Num};
use num::complex::Complex32;


pub struct  Plan<'b, T, D: Dimension> {
    data_in: &'b mut Array<T, D>,
    data_out: &'b mut Array<T, D>,
    shape: Vec<i32>,
    number_batches: i32,
    origin: Box<dyn Any>,
}


impl <'b, D: Dimension> Plan<'b, Complex32, D>{
    pub fn new_complex_float(
        sign: bindings::FFT_SIGN,
        device: bindings::FFT_DEVICE,
        data_in: &'b mut Array<Complex32, D>,
        data_out: &'b mut Array<Complex32, D>,
    )->Plan<'b, Complex32, D>{

        unsafe {
            let mut plan = Plan::new(
                data_in, data_out
            );

            let mut error = 0;

            let plan_origin = bindings::fft_new_plan_float(
                plan.shape.len() as i32,
                plan.shape.as_ptr(),
                plan.number_batches,
                sign,
                device,
                plan.data_in.as_mut_ptr() as *mut [f32; 2],
                plan.data_in.len() as u64,
                plan.data_out.as_mut_ptr() as *mut [f32; 2],
                plan.data_out.len() as u64,
                &mut error as *mut i32,
            );

            plan.origin = Box::new(plan_origin);
            plan
        }
    }
}



impl<'b, T, D: Dimension> Plan<'b, T, D> {
    fn new (
        data_in: &'b mut Array<T, D>,
        data_out: &'b mut Array<T, D>,
    ) -> Plan<'b, T, D>{


        let shape_nd = data_in.shape();
        let mut shape = Vec::new();

        for i in 1 .. shape_nd.len() {
            shape.push(shape_nd[i] as i32);
        }

        let number_batches = shape_nd[0] as i32;

        Plan{
            data_in,
            data_out,
            shape,
            number_batches,
            origin: Box::new(0i8),
        }
    }



}






#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use ndarray::{array, Array, ShapeBuilder};
    use num::complex::Complex32;
    use crate::bindings::{FFT_DEVICE_FFT_DEVICE_CPU, FFT_SIGN_FFT_SIGN_FORWARD};
    use crate::Plan;
    use ndarray::prelude::*;
    #[test]
    fn it_works() {


        let mut  a =
          Array::<Complex32, _>::ones((2, 4));


        let mut b =
            Array::<Complex32, _>::ones((2, 4));

        let plan = &mut Plan::new_complex_float(
            FFT_SIGN_FFT_SIGN_FORWARD,
            FFT_DEVICE_FFT_DEVICE_CPU,
            &mut a,
            &mut b,
        );
        {
            for i in 0..4 {
                plan.data_in[[0, i]] = Complex32::new(i as f32, -(i as f32));
            }
        }

        let s = &plan.shape;

        assert_eq!(4, 4);
    }
}
