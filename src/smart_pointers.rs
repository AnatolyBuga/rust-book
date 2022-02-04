//! String, Vec<T> are smart pointers
//! has to implement Deref and Drop traits
//! Deref allows to behave like a ref
//! Drop when inst goes out of scope - drop Heap data
//! Smart Pointers own data they refer to

use crate::smart_pointers::List::{Cons, Nil};
use std::ops::Deref;
use std::fmt::Debug;
use std::mem::drop;

///Use when:
/// 1. Size can't be known at compile time
/// 2. Large data which is expensive to keep Copying on Stack
/// 3. When want to own, and care only that it's a type that impl particular Trait
pub fn _box() {
    //b has value of a Box(on Stack) that points to value 5(on Heap)
    //b owns that box
    let b = Box::new(5);
    //Box allows to define some Types which are otherwise not possible
    //eg Recursive Type (part of value is another value of same type)
    // eg cons list AKA singly linked list- size can't be known at compile time
    let list = Cons(1, Box::new(Cons(2,
    Box::new(Cons(3, Box::new(Nil))))));
    let list2 = Cons(2, Box::new(list)); //ownership of list passed!!!
    //println!("{:?}", list); can't do this anymore
    println!("{:?}", list2);

    //Deref allows you to customize behaviour of * operator
    let x = 5;
    let y = MyBox::new(x); //Box copies val of x to the Heap
    println!("{}", *y);
    //*y is shortcut for *(y.deref())
    let p = y.deref();
    let m = MyBox::new(String::from("Anatoly"));
    hello(&m); // &MyBox<String> -> &String -> &str
    //eq to
    hello(&(*m)[..]);
    let f = String::from("Anatoly");
    let g = &f;
    //let h = *f;
    //Notice MyBox-Anatoly dropped first, because m is on the stack
    //Early drop:
    drop(y);
}

/// Box is 1) a pointer to the Heap 2) Heap allocation with value inside
/// Box implements Drop, so when goes out of scope - data removed
/// When a Box is moved, only the pointer is moved - this can be cheaper than move
#[derive(Debug)]
enum List {
    //can't do this, infinite size:
    //Cons(i32, List),
    //Box<T> is a pointer, but OWNs, so Rust knows how much space it needs (pointer size is the same)
    //Box<T> will point to the next List value which will be on the Heap
    Cons(i32, Box<List>),
    Nil,
}


///How much space to allocate?
/// Only one variant will be used, so largest value
enum Message {
    Quit, //No space
    Move {x: i32, y: i32}, //two i32 values
    Write(String),
    ChangeColor(i32, i32, i32)
}

///MyBox is similar to Box, but value is NOT on the Heap
/// MyBox is a tuple struct
struct MyBox<T> (T) where T: Debug;

impl<T: Debug> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

///In order to Deref we need the Trait
/// Use DerefMut trait to override * for mut refereces
impl<T: Debug> Deref for MyBox<T> {
    type Target = T; //associated Type
    ///If deref returned the value directly, the value would be moved out of self
    /// so we return reference to a value
    fn deref(&self) -> &Self::Target {
        &self.0 //here 0 is the index of the tuple
    }
}

/// Deref coercion  - works on types that implement Deref
/// Deref coer. converts such a type into a ref to another type eg &String to &str
/// Because String Deref trait returns &str
/// Happens automatically when when we pass a ref to a particular type as an arg to a func
/// that doesn't match the param type in func def
/// sequence of calls to deref converts the type we provide to the type func needs
fn hello(name: &str) {
    println!("Hello, {}", name);
}

//&T to &U when T: Deref<Target=U> eg &MyBox -> &String
//&mut T to &mut U when T: DerefMut<Target=U>
//&mut T to &U when T: Deref<Target=U>
//Notice immut ref doesn't coerce to mut, because if mut exists, no other mut/immut can exist
//reminder: no 2 mut simultaneously, no mut and immut simultaneously


///Drop specifies code to run when a value goes out of scope
impl<T> Drop for MyBox<T>
where T: Debug,
{
    fn drop(&mut self) {
        println!("Dropping MyBox with value: {:?}", self.0)
    }
}
