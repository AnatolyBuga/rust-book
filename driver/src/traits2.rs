use std::fmt::{Debug, Display};

//each type implementing trait Summary must provide method summarize
pub trait Summary {
    fn summarize(&self) -> String;
    //this one is default for all T where Trait was implied
    //default trait methods can call other methods
    fn default(&self) -> String {
        format!("{} @default inc", self.summarize())
    }
}

//function which takes !some Type that implements Trait! as param
pub fn notify(item: &impl Summary) {
    //from here you can only call what's been defined on the Trait
    //item.username; - can't do that
    println!("Breaking news! {}", item.summarize())
}
//above is syntatic sugar for:
pub fn notify2<T: Summary>(item: &T){
    //^This is called trait bound syntax
    println!("Breaking news! {}", item.summarize())
}
//Below two are same:
fn notify3(i1: &impl Summary, i2: &impl Summary) {}
fn notify4<T: Summary>(i1: &T, i2: &T) {}

//Multiple Trait Bounds
fn notify5(i: &(impl Summary + Display)) {}
fn notify6<T: Summary + Display>(i: &T) {}
//WHERE clause
//instead of writing
fn some_function<T: Summary + Display, U: Display + Debug>(t: &T, u: &U) -> u8 {1}
//we can write
fn some_function2<T, U>(t: &T, u: &U) -> u8
    where T: Summary + Display,
          U: Display + Debug {
    1
}
//return val of a T that implements a Trait
//However, Summary here can only be one
//for example it can be Tweet, but can not be Tweet or NewsArticle
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse"),
        content: String::from("dog"),
        reply: false,
        retweet: false
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub body: String
}

//can only impl Trait on Type if at least one of Trait or Type is local to our crate
//eg std::Display on Tweet
//or Summary on Vec
//but not Display on Vec - this restriction called coherence - orphan rule
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        //format! concatenates strings
        format!("{}, by {} ({}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    //here Self is Pair<T>
    fn new(x: T, y: T) -> Self { Self{x, y} }
}
//only for Pair<T> where T implements Display and PartialOrd
impl<T: Display+PartialOrd> Pair<T> {
    fn compare_and_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x)
        } else {
            println!("The largest number is y = {}", self.y)
        }
    }
}
//the standrad library example:
//ToString trait implemented on any T which implements Display trait
//impl<T: Display> ToString for T {}