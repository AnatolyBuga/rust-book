//! Single value can have multiple owners eg Graphs multiple edges point to same Node
//! Rc<T> for Reference Counting - when 0 references value dropped
//! Rc only for single-threaded scenarios
//! Rc doesn't allow direct mutation, only immut borrow
//! If Rc holds RefCell then you can have multiple owners AND can mutate
//!
//! Cell<T> is similar, but instead of reference to inner val, the val is copied(needs Copy) in and out.
//! Cell provides values, not refs. Similar to &mut T
//! Mutex<T> - interior mutability to be used across threads

use crate::smart_pointers2::List::{Cons, Nil};
use crate::smart_pointers2::List2::{Cons2, Nil2};
use std::rc::Rc;
use std::cell::RefCell;

pub fn _rc(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count b {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); //clone instead of taking ownership
    //now a and b share ownership of Rc<List> in a
    println!("count b {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); //c also shares ownership of Rc<List> in a
        //better than Cons(4, a.clone()); because doesn't make deep copy
        println!("count c {}", Rc::strong_count(&a));
    }
    //Drop dicreases count when c goes out of scope
    println!("count c out of scope {}", Rc::strong_count(&a));

    let d = Rc::new(5);
    let k = Rc::clone(&d);
    let mut i = *k;
    //*d += 5; can't do this since normally Rc is read only
    i = i + 1;
    //drop (d);
    println!("{}", d);
    println!("{}", k);
    println!("{}", i);

    let val = Rc::new(RefCell::new(17));
    let a = Rc::new(Cons2(Rc::clone(&val), Rc::new(Nil2)));
    //clone val above so that both a and val own the 17 inside - thanks to Rc
    let b = Cons2(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons2(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *val.borrow_mut() += 100;
    //deref Rc<T> to inner RefCell<T>
    //borrow_mut returns RefMut - so we change value
    //without it a, b, c would be referencing val and wouldn't be able to change it
    println!("val after = {:?}", val);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let e = RefCell::new(9);
    let mut mr = e.borrow_mut(); //value 9 borrowed
    *mr += 1;
    println!("{:?}", e);

}

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),//holds a value and Rc points to a List
    Nil
}

fn drop (r: Rc<i32>) {
    println!("{}", r)
}

///without RefCell we can't change values after created them
#[derive(Debug)]
enum List2{
    //Cons(i32, Rc<List>),//holds a value and Rc points to a List
    Cons2(Rc<RefCell<i32>>, Rc<List2>), //Rc let's you have multiple owners and RefCell lets you mutate immutable
    Nil2
}

