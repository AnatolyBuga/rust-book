//Trait Summary has to be brought into scope
use crate::traits2::{Summary, Tweet, NewsArticle, notify};
use crate::traits3::{PointA, Millis, Meters, Human, Pilot, Wizard, Dog, Animal, OutlinePrint, Wrapper};

pub fn traits () {
    let tweet = Tweet {
        username: String::from("anatoly.bugakov"),
        content: String::from("Trump 2020"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("2 new tweet: {}", tweet.default());
    notify(&tweet);
    let k = "hello".as_bytes();
    let p = PointA{x: 1, y: 0} + PointA{x: 4, y: 0};
    println!("{:?}", p);
    let mm = Millis(31);
    let m = Meters(2);
    let mm_m = mm + m;
    println!("{:?}", mm_m);
    let p = Human;
    p.fly(); //By def Rust uses method impl directly on Struct
    Pilot::fly(&p);
    Wizard::fly(&p);
    // BUT what if fn doesn't take &self as param?
    // baby_name doesn't take self or &self so it's an associated function, not method
    let d = Dog::baby_name();
    println!("{}", d);
    // FULLY QUALIFIED SYNTAX
    println!("dog? {}", <Dog as Animal>::baby_name());
    PointA{x: 1, y: 0}.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("len = {}", w.len()); //take advantage of deref coer

    let a: Box<dyn Pilot> = Box::new(p);
    a.fly();

}

fn function(x: &str) {
    println!("{}", x)
}