//! Testing if .par_iter() is better than .into_iter()
//! Steps:
//! .apply func with .par_iter()/.into_par_iter() vs .into_iter()
//! see apply source code as example of how to return Expr

//use ndarray::Array;
//use polars::export::arrow::array::{Utf8Array, Float64Array};
//use polars::export::arrow::array::Array;
//use polars::export::arrow::datatypes::IntegerType;
//use polars::export::rayon::iter::IntoParallelRefIterator;
use polars::prelude::*;
use polars::df;
use polars::export::rayon::iter::ParallelIterator;
//use polars::export::rayon::iter::IndexedParallelIterator;
//use ndarray::{Zip, ArrayBase, ViewRepr};
//use ndarray::prelude::*;
use polars::export::rayon::prelude::*;

pub fn pp() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let n = ChunkedArray::<Float64Type>::rand_uniform("Anatoly", 5, 1.0, 7.5)
        .into_series();
    let u = ChunkedArray::<Float64Type>::rand_uniform("Bugakov",5, 0.0, 100.0)
        .into_series();
    let pd = df![
        "norm" => &n, 
        "uniform" => &u]?;
    
    let o = GetOutput::from_type(DataType::Float64);
    let elapsed = now.elapsed();
    println!("Elapsed1: {:.10?}", elapsed);
    let x = pd.clone().lazy()
            .with_columns([
                lit::<f64>(0.0).map_many(pa_fa, &[col("norm"), col("uniform")], o)
            ]).collect();
    let elapsed = now.elapsed();
    println!("Elapsed2: {:.10?}", elapsed);
    println!("df1: {:?}", x);
    Ok(())
}

pub fn pa_fa(s: &mut [Series])->Result<Series>{
    let u = s[2].f64()?;
    let n = s[1].f64()?;
    let n_iter = n.into_iter();

    let c: Vec<f64> = n_iter.zip(u.into_iter()).par_bridge().map(
    // ignore this line   let c: Vec<f64> = n_iter.zip(u).map( 
        |(n,u)|{
            n.unwrap().powf(1.777)*u.unwrap().sqrt()
        }
    ).collect();

    Ok(Series::new("Ant", c))
}