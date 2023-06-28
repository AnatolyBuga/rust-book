use polars::prelude::*;
use polars::df;
use std::ops::Mul;
use polars::export::rayon::iter::{ParallelIterator};

pub fn p() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let ca = ChunkedArray::<Float64Type>::rand_standard_normal("Anatoly", 5)
                    .into_series(); //need rand feature

    //let nd = ca.to_ndarray(); - can't convert series

    let _df = df![
        "a" => [1, 2, 3, 4, 5],
        "b" => [Some("a"), Some("a"), Some("b"), Some("a"), None],
        "nrs" => [Some(1), Some(2), Some(3), None, Some(5)],
        "names" => ["foo", "ham", "spam", "egg", "bread"],
        "random" => &ca,
        "groups" => ["A", "A", "B", "C", "D"]
    ]?;

    let x = _df.to_ndarray::<Float64Type>();
    println!("x: {:?}", x);
    println!("df: {:?}", _df);

    // EXPRESSIONS
    let first = _df.clone().lazy() //can't use df anymore
    .select(vec![ //SELECT is one of 3 possible contexts

        when(col("a").gt_eq(lit::<i32>(4))) 
        .then(col("b"))
        .otherwise(col("b")), //this isa chained  expression

        col("groups")
        .n_unique()
        .alias("unique_groups"), //supposed to return one number, but 
        //since first expr return a vector
        //this one extends

        sum("random").over(vec![col("groups")]), //WINDOW
        //over = implicit GROUPBY
        //also extends
        //over - don't expect to change df size
        //groupby change size

        col("groups").sum(),

    ])
    .collect(); //need lazy feature

    println!("first: {:?}", first);

    let second = _df.clone().lazy()
    .filter(col("groups").eq(lit("A")))
    .select(vec![
        sum("random").alias("sum") + //supports binary operators
        col("random").std().alias("standard deviation"),
        col("a").filter(col("a").is_not_null())
    ])
    .collect();
    println!("second: {:?}", second);

    //GROUPBY AGGREGATE is another context
    let third = _df.clone().lazy()
    .groupby([col("groups"), col("b")])
    .agg(vec![
        col("random")
        .filter(col("nrs")
        .is_not_null())
        .sum()
        .mul(lit::<i32>(10))
        .suffix("_sum"),

        col("names")
        .reverse().
        alias("reversed names"),

    ])
    .sort_by_exprs([col("random_sum")], vec![false])
    .collect();    
    println!("third: {:?}", third);
    //Note
    //let x = col("groupd").mul(lit(10)); //this requires std::ops::Mul
    //let x = col("groupd").is_in(col("a")); //requires is_in feature
    //let f = col("groups").into_range(); //requires range feature
    let o = GetOutput::from_type(DataType::Utf8);
    let q = _df.clone().lazy()
    .groupby([col("groups")]) //groupby, groupby_dynamic, and groupby_rolling
    .agg([
        col("b").apply(capita, o.clone()).alias("bespoke_apply"),
    ]).limit(5);
    let fourth = q.collect();
    println!("fourth: {:?}", fourth);
    //let h = Fold::fold();
    //fold_exprs(acc, f, exprs) - as long as f is parallelised (see: https://docs.rs/polars/0.21.0/polars/export/rayon/iter/trait.ParallelIterator.html#method.fold)
    // ! see apply source code as example of how to return Expr
    let fofo = |s1: Series, s2: Series| Ok(&s1 + &s2);
    let random = "random";
    let fifth = _df.clone().lazy()
    .groupby([col("groups"), col("b")])
    .agg([
        fold_exprs(lit::<f64>(100.0), fofo, vec![col(random).mean(), col(random).max()]).alias("bespoke_fold_expr")
    ]).collect();
    
    println!("fifth: {:?}", fifth);    
    let sixth = _df.clone().lazy()
    .select([
        all(), //select all
        //NOTE, forting nrs on a reverse doesn't change order of a
        col("nrs").sort_by([col("a")], [true]).alias("nrs_sorted_by_a_reverse"), 
        //Note can't have two cols named nrs
        col("nrs").sort_by([col("a")], [true]).over([col("groups")]).alias("what does this do?")
    ]).collect();

    println!("six: {:?}", sixth);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed);

    let s0 = Series::new("a", &[1i64, 2, 3]);
    let s1 = Series::new("b", &[1i64, 1, 1]);
    let s2 = Series::new("c", &[2i64, 2, 2]);
    // construct a new ListChunked for a slice of Series.
    let list = Series::new("foo", &[s0, s1, s2]);

    let s0 = Series::new("a", &["ant", "bug", "dim"]);
    let s1 = Series::new("b", &["vic", "pu", "al"]);
    let s2 = Series::new("c", &["nat", "buga", "vik"]);
    // construct a new ListChunked for a slice of Series.
    let list1 = Series::new("cocopu", &[s0, s1, s2]);

    // construct a few more Series.
    let s0 = Series::new("B", [None, Some(2.), Some(3.)]);
    let s1 = Series::new("C", [1, 1, 1]);
    let df = DataFrame::new(vec![list, list1, s0, s1])?;

    println!("before explosion: {:?}", df);

    let exploded = df.explode(vec!["foo", "cocopu"]).unwrap();

    println!("after explosion: {:?}", exploded);

    println!("after explosion: {:?}", df.get_column_names());

    for i in df.iter() {
        println!("iter: {:?}", i)
    }

    let s7 = Series::new("7", [7, 3, 7]);

    let x = lit(s7.clone());

    let df_with = df.clone().lazy().with_columns(vec![
        x,
    ])
    .collect()?;

    let df_with = df.clone().lazy().with_columns(vec![
        col("foo").arr().max().apply(|srs|{
            let res: ListChunked = srs.i64().unwrap()
            .into_iter()
            .map(|x|{
                Series::new("", &[x.and_then(|y|Some(y as f64))])
            })
            .collect();
            Ok(res.into_series())
        }, GetOutput::from_type(DataType::List(Box::new(DataType::Float64)))),
    ])
    .collect()?;

    dbg!(df_with.clone());

    //let fltrd_df = df_with.clone().lazy().filter(
    //    col("B").eq(lit::<f64>(1.).or(lit::<f64>(3.)))
    //).collect().unwrap();

    let fltrd_df = df_with.clone().lazy().filter(
            col("B").neq(lit::<f64>(3.)).and(col("B").neq(NULL.lit()))
        ).collect().unwrap();

    dbg!(fltrd_df);

    Ok(())
}

fn capita(x: Series) -> Result<Series> {
    let y = x
            .utf8()
            .unwrap()
            //.into_iter()
            .par_iter() //ParallelIterator
            .map(|opt_str| {
                opt_str.map(|st| {
                    let mut c = st.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
            })})
            .collect::<Utf8Chunked>();
            Ok(y.into_series())
} 