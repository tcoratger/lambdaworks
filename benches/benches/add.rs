use std::{ops::Add, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use utils::generate_random_elements;

use crate::utils::to_lambdaworks_vec;
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use std::ops::Mul;

pub mod utils;

const BENCHMARK_NAME: &str = "add";

pub fn criterion_benchmark(c: &mut Criterion) {
    // let arkworks_vec = generate_random_elements(2000000);

    // // arkworks-ff
    // {
    //     c.bench_function(
    //         &format!("{} 1M elements | ark-ff - ef8f758", BENCHMARK_NAME),
    //         |b| {
    //             b.iter(|| {
    //                 let mut iter = arkworks_vec.iter();

    //                 for _i in 0..1000000 {
    //                     let a = iter.next().unwrap();
    //                     let b = iter.next().unwrap();
    //                     black_box(black_box(&a).add(black_box(b)));
    //                 }
    //             });
    //         },
    //     );
    // }

    // // lambdaworks-math
    // {
    //     let lambdaworks_vec = to_lambdaworks_vec(&arkworks_vec);

    //     c.bench_function(
    //         &format!("{} 1M elements | lambdaworks", BENCHMARK_NAME,),
    //         |b| {
    //             b.iter(|| {
    //                 let mut iter = lambdaworks_vec.iter();

    //                 for _i in 0..1000000 {
    //                     let a = iter.next().unwrap();
    //                     let b = iter.next().unwrap();
    //                     black_box(black_box(&a).add(black_box(b)));
    //                 }
    //             });
    //         },
    //     );
    // }

    // lambdaworks-math
    {
        let x = FieldElement::<Stark252PrimeField>::from_hex_unchecked(
            &"0x6606d7dccf23a0f61182da8d1149497f01b909036384bedb3e4c3284e2f2c1e1",
        );
        let y = FieldElement::<Stark252PrimeField>::from_hex_unchecked(
            &"0x4cd366c0feadabcd6c61a395f6d9f91484ac4e51c3f8aede6c0ab49e2a55446a",
        );

        let xx: u64 = 56;
        let yy: u64 = 100;

        c.bench_function(
            &format!("{} 1M elements | lambdaworks", BENCHMARK_NAME,),
            |b| {
                b.iter(|| {
                    // black_box(black_box(xx) + black_box(yy));
                    black_box(black_box(&x).mul(black_box(y)));
                    // for _i in 0..100000 {
                    //     black_box(black_box(&x).add(black_box(y)));
                    // }
                });
            },
        );
    }
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default()
        .significance_level(0.01)
        .measurement_time(Duration::from_secs(15))
        .sample_size(300);
    targets = criterion_benchmark
}
criterion_main!(benches);
