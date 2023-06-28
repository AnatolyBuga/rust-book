use rand::distributions::Bernoulli;
use rand::prelude::*;
use rand_distr::{Distribution, Normal, Standard, StandardNormal, Uniform};
use rayon::iter::{ParallelBridge, IntoParallelRefIterator};
use rayon::prelude::*;

pub fn rayon_test() {
    let x: Vec<f64> = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    let y = x.clone();
    let c: Vec<f64> = x.par_iter().zip(y).map(
        |(x, y)| {
            x+y
        }
    ).collect();
    println!("result: {:?}", c);
}