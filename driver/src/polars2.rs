use polars::prelude::*;
use ndarray::prelude::*;
use ndarray::concatenate;
use std::collections::VecDeque;

pub fn p() {
    let s0 = Series::new("a", &[1i64, 2, 3]);
    let s1 = Series::new("b", &[1i64, 1, 1]);
    let s2 = Series::new("c", &[2i64, 2, 2]);
    // construct a new ListChunked for a slice of Series.
    let list = Series::new("foo", &[s0.clone(), s1, s2]);

    // construct a few more Series.
    let s3 = Series::new("B", [1, 2, 3]);
    let s4 = Series::new("C", [1, 1, 1]);
    let df = DataFrame::new(vec![list, s3, s4]).unwrap();

    let s = serde_json::to_string(&s0).unwrap();
    println!("Series to String: {s}");
    let s0 = serde_json::from_str::<Series>(&s).unwrap();
    println!("{:?}", s0);

    let a = girr_corr_matrix();

    let st = serde_json::to_string(&a).unwrap();
    //println!("Array2 to String: {st}");

    let dt = s0.dtype();
    //println!("DType: {dt}");

    let s = Series::new("vendor_id", &["Ant", "no", "how", "Ant", "mans"]);

    let _rf = s.utf8().unwrap();
    let rf1 = _rf.get(0).unwrap();
    use rayon::iter::ParallelIterator;
    let rf_vec: Vec<f64> = _rf
        .par_iter()
        .map(|x| match x {
            Some(rf2) if rf2==rf1 => 1. ,
            _ => 0.999
        })
        .collect();
    let rf_arr = Array2::<f64>::from_shape_vec((1, rf_vec.len()), rf_vec);
    dbg!(rf_arr);

    let df = DataFrame::new(vec![s]).unwrap();
    let res = df.lazy()
    .with_columns( &[
        
        col("vendor_id").apply(|srs|{
        Ok(srs.utf8()?
            .str_lengths()
            .into_series())
    }, GetOutput::from_type(DataType::Int32))
    .alias("vendor_id_length"),
    col("vendor_id").cast(DataType::Boolean),
    ])
    .collect();

    dbg!(res);
    
}

fn girr_corr_matrix() -> Array2<f64> {
    let mut base_weights = Array2::<f64>::zeros((10, 10));
    let tenors = [ 0.25, 0.5, 1., 2., 3., 5., 10., 15., 20., 30. ];

    for ((row, col), val) in base_weights.indexed_iter_mut() {
        let tr = tenors[row];
        let tc = tenors[col];
        *val = f64::max( f64::exp( -0.03 * f64::abs(tr-tc) / tr.min(tc) ) , 0.4 );
    }
    base_weights
}

pub(crate)fn shift_columns_by(by: usize, a: &Array2<f64>) -> Array2<f64> {
    
    // if shift_by is > than number of columns
    let x: isize = ( by % a.len_of(Axis(1)) ) as isize;

    // if shift by 0 simply return the original
    if x == 0 {
        return a.clone()
    }
    // create an uninitialized array
    let mut b = Array2::uninit(a.dim());

    // x first columns in b are two last in a
    // rest of columns in b are the initial columns in a
    a.slice(s![.., -x..]).assign_to(b.slice_mut(s![.., ..x]));
    a.slice(s![.., ..-x]).assign_to(b.slice_mut(s![.., x..]));

    // Now we can promise that `b` is safe to use with all operations
    unsafe {
        b.assume_init()
    }
}