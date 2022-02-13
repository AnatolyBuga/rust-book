//! Functions coerce to fn - function pointer (not Fn, which is a closure Trait)
//! fn is a Type, not a Trait
//! fn implements couse Traits(Fn, FnMut, FnOnce)
//! so you can also pass fn to a function which expects a Closure
fn add(x: i32) -> i32 { x + 1 }
fn twice(f: fn(i32) -> i32, arg: i32) -> i32{f(arg) +f(arg)}
pub fn functions(){
    let answer: i32 = twice(add, 1);
    println!("{}", answer);
    let list = vec![1, 2, 3];
    let list_str: Vec<String> = list.iter().map(|i| i.to_string()).collect();
    let list_str2:Vec<String> = list.iter().map(ToString::to_string).collect();
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
enum Status {
    Value(u32),
    Stop
}