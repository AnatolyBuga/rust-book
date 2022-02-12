//in Rust iterators are LAZY ie they have no effect until you call methods

use std::iter::{Filter, Map};
use std::path::Iter;

pub fn iterators() {
    let v = vec![2, 1];
    let mut v_iter = v.iter(); //.next() return immut ref to vals in vector
    //let v_iter2 = v.into_iter(); //iter takes ownership .next() returns owned values
    //let v_iter3 = v.iter_mut(); //.next() returns mut ref
    println!("{:?}", v_iter.next()); //calling next on iter changes internal state, hence has to be mut
    //for loop takes ownership of iter and makes it mut behind the scenes
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    let sum: i32 = v_iter.sum(); //sum takes ownership, so can't use v_iter again
    //map calls a closure on each item of iter
    let v_iter_mapped = v.iter().map(|x| x + 100);
    for i in v_iter_mapped {
        println!("{}", i)
    }
    let v_iter_mapped2 = v.iter().map(|x| x + 100);
    let v2: Vec<i32> = v_iter_mapped2.collect(); //collect takes ownership, returns all items in iter as Vec
    println!("{:?}", v2);

    //Using closure that capture env
    //filter is iter adoptor, takes a closure that return bool
    //returns iterator, but only where closure returned True

    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")},
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);

    let mut counter = Counter::new(0);
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());

    let f = Counter::new(0)
                   .zip(Counter::new(2).skip(1));
    for o in f {
        println!("{:?}", o)
    }

    let fltr = iter_playground();
    let mut sum: u32 = 0;
    for j in fltr {
        sum += j;
        println!("{:?}", j)
    }
    println!("{}", sum);
    returns();
}

//Iterator is a pub Trait(in std) which looks similar to this:
pub trait _Iterator {
    type Item; //associated type with this trait
    fn next(&mut self) -> Option<Self::Item>; //Item type will be the type returned from the Iterator
    //default methods
    //methods which call next are called consuming adaptors and others - iterator adaptors
    //because lazy you have to call consuming adaptor before iterator adaptors
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(PartialEq, Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new(initial: u32) -> Self {
        Self {count: initial}
    }
}

/// Can't implement a trait on Type multiple times
/// that's the difference between associated types and generics
impl Iterator for Counter
{
    type Item = u32;
    fn next (&mut self) -> Option<Self::Item> {
        if self.count < 6 {
            let count_sq = self.count*self.count;
            self.count += 1;
            Some(count_sq) //need Copy here to move
        } else {None}
    }
}
//
fn iter_playground()  -> impl Iterator<Item=u32> {
    let fltr = Counter::new(0)
                   .zip(Counter::new(2).skip(1))// Iterable of Item=(u32, u32) aka tuple
                   .map(|(a,b)| a*b) // Iterable Item u32
                   .filter(|x| x%3 != 0) //Iterable Item u32
    //Note zip doesn't produce when one of items is None, ie not produces (5, None)
        ;
    fltr
}

fn returns () {
    let mut a = [3, 4, 5, 7, 11, 10];
    let buf: &mut [i32] = &mut a;
    let lag = 1;

    let _returns: Vec<i32> = a.iter()
                   .zip(&a[lag..a.len()])
                   .map(|(&c, &s)| s - c)
                   .collect();

    println!("{:?}", _returns)

}