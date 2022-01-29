//Closures are anonymous functions
//Closures can capture values from the scope in which they are defined

use std::thread;
use std::time::Duration;
use std::collections::HashMap;


pub fn closures() {
    let sim_user_spec_val = 10;
    let sim_rand_n = 7;
    //following serve same purpose (but not equivalent):
    fn add_1 (x: u32) -> u32 {x+1};
    let add_2 = |x: u32| -> u32 {x+1};
    let add_3 = |x| {x+1}; //Note: either specify return type or use so that Rust can determine return type
    add_3(1);
    let add_4 = |x| x as f64 + 1.0; //same as above
    add_4(6.6);
    //can do only one type for closure. Can't do this: let s = add_4(5); let u = add_4(6.6)
    generate_workout(sim_user_spec_val, sim_rand_n);

    //closures can capture environement and access variables from the scope in which they are defined
    let x = 4;
    let eq_to_x = |z| z==x; //can use x here since x is same scope as eq_to_x
    //storing env variables like that creates overhead. functions don't have such a problem
    let y = 4;
    println!("{}", eq_to_x(y));

    //similar to func, closures capture values from env in three ways:
    //1. taking ownership <-> FnOnce Trait takes ownership (aka moves). ?can be called once only?. ALL closures impl FnOnce
    //2.borrowing mut <-> FnMut Trait mutable borrows => can change env
    //3. borrowing immut <-> Fn Trait borrows values from env immutably
    //Rust infers which Traits to assign. eg eq_to_x is FnOnce and Fn
    // #! These Traits are determined by WHAT the closure DOES with captured values, not
    //HOW it captures them.

    //use move keyword to force take ownership, useful when passing closure to a new thread
    let k = vec![1, 2, 3];
    let v = vec![1, 2, 3];
    let equal_to_x2 = move |z| z == k;
    equal_to_x2(v);
    //Can't do this now, closure took the ownership:
    //println!("can't use x here: {:?}", k);
    //println!("can't use x here: {:?}", v);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    //what if I don't want to call ?
    //let expensive_res = simulated_expensive_calculation(intensity);
    //Closere only executes when result is needed:
    let mut expensive_res = Cacher::new( |num| {
        println!("calculating slowly....{}", num);
    thread::sleep(Duration::from_secs(1));
    num + 100
    });

    println!("{}", expensive_res.value(1));
    println!("{}", expensive_res.value(2));
    println!("{}", expensive_res.value(1));


    //expensive_closure contains definition of anonymous function

    //use LAZY EVALUATION (aka memorization) see struct below

    /*
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_res.value(intensity));
        println!("Next, do {} situps!", expensive_res.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_res.value(intensity));
        }
    }
     */


}

//Struct to hold the closure and resulting value
//only executes closure if we need resulting value
//caches resulting value

//Fn is a Trait. Closure can be one of: Fn, FnMut, FnOnce. u32 is paparm of closure
// (could be more than one param)
//calc, value are private, as our code shouldn't change those values
//it will only be changed through methods
struct Cacher<T, K, V>
    where T: Fn(K) -> V, //struct needs to know Type of closure(aka calc)
          K: std::hash::Hash + std::marker::Copy + std::cmp::Eq,
          V: std::marker::Copy,
{
    calc: T,
    value: HashMap<K, V>
}

impl<T, K, V> Cacher<T, K, V>
where T: Fn(K) -> V, //struct needs to know Type of closure(aka calc)
      K: std::hash::Hash + std::marker::Copy + std::cmp::Eq,
      V: std::marker::Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V>{
        Cacher {
            calc: calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) { //get requires Hash + Eq
            Some(v) => *v, //here moves v (because moving v out of the function?)
            None => {
                let res = (self.calc)(arg);
                self.value.insert(arg, res); //use of moved value (moving arg to self.value)
                //inser requires Hash and Eq
                res
            }
        }
    }
}