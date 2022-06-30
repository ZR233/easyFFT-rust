extern crate rand;

use std::thread::Thread;
use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libc::time;
use num::complex::Complex32;
use easyfft_binding::{Plan, Sign, Device, Result};
use rand::Rng;



fn create_sample(shape: Vec<i32>,
                 number_batches: usize)-> Vec<Complex32>{
    let mut rng =rand::thread_rng();
    let mut size = number_batches;
    for one in shape {
        size *=one as usize;
    }
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        data.push(Complex32::new(rng.gen_range(1f32..256f32), rng.gen_range(1f32..256f32)));
    }

    data
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let shape = vec![512];
    let batches = 10000;
    let sample = create_sample(shape.clone(), batches);

    let mut plan_gpu = Plan::new_complex_float(
        shape.clone(),
        batches,
        Sign::Forward,
        Device::GPU,
    ).unwrap();
    let mut plan_cpu = Plan::new_complex_float(
        shape.clone(),
        batches,
        Sign::Forward,
        Device::CPU,
    ).unwrap();

    for i in 0..sample.len() {
        plan_cpu.data_in[i] = sample[i].clone();
        plan_gpu.data_in[i] = sample[i].clone();
    }
    let cpu_name= plan_cpu.device_name().unwrap();
    let gpu_name = plan_gpu.device_name().unwrap();

    c.bench_function(cpu_name.as_str(), |b| b.iter(|| plan_cpu.execute().unwrap() ));
    c.bench_function(gpu_name.as_str(), |b| b.iter(|| plan_gpu.execute().unwrap() ));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);