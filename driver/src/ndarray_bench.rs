use ndarray::{prelude::*, concatenate};
use rand::{distributions::Uniform, Rng};

pub fn arr_test1() -> Array2<f64> {
    let n =10;
    let y_len = 10000;
    let corr_with_self_yield = girr_corr_matrix();
    let corr_with_other_yields = girr_corr_matrix()*0.9;
    let range = Uniform::from(0..1000);
    let mut idx_to_be_removed: Vec<usize> = rand::thread_rng().sample_iter(&range).take(100).collect();
    idx_to_be_removed.sort_unstable();
    idx_to_be_removed.dedup();
    
    let mut vec_yield_corr_row1 = vec![corr_with_self_yield.view()];
    let mut vec_corr_with_other_curves = vec![corr_with_other_yields.view(); y_len -1 ];

    let arr_yield_corr_row1 = ndarray::concatenate(Axis(1), vec_yield_corr_row1.as_slice()).unwrap();

    let mut res = arr_yield_corr_row1.clone();
    for j in 1..y_len {
        //for each curve shift the block by 10 and then add to res as rows
        res = concatenate![Axis(0), res, shift_columns_by(j*n, &arr_yield_corr_row1)]
    };

    for i in idx_to_be_removed.iter().rev() {
        res.remove_index(Axis(0), *i);
        res.remove_index(Axis(1), *i);
    }
    res
}

// Helper function to remove indexes
// Not used at as .select(non_nan_zero_idx) is preffered
fn reduce_nans(mut a: Array1<f64>, mut m: Array2<f64>) -> (Array1<f64>, Array2<f64>) {
    let mut nans: Vec<usize> = vec![];
    for (i, n) in a.indexed_iter() {
        if n.is_nan()|(*n==0.) {nans.push(i)};
    };

    for i in nans.iter().rev() {
        a.remove_index(Axis(0), *i);
        m.remove_index(Axis(0), *i);
        m.remove_index(Axis(1), *i);
    };
    (a, m)
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

fn shift_columns_by(by: usize, a: &Array2<f64>) -> Array2<f64> {
    
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