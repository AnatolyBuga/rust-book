use std::io;
use rand::Rng;  //needed for gen_range
use std::cmp::Ordering;
use std::env::{args, Args};


pub fn gg() {
    let secret_number = rand::thread_rng().gen_range(1..10);
    println!("The secret number is {}", secret_number);

    let mut _args: Args = args();
    //{:?} is to pring Debug trait
    println!("All Args: {:?}", _args);
    //note: nth(x) is a generator, which iterates over the input x times;
    //ie if calling nth(1), to get the NEXT argument, you must call nth(0);
    let first = _args.nth(0).unwrap();
    println!("First Arg: {}", first);


    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //shadowing guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //.trim() eleminates whitespaces, \n, \r\n

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            _ => {
                println!("You win!");
                break;
            }
        };
    }

    //Pointers, Borrowing
    let mut a = String::from("Anatoly");
    let guess_copy = a.clone(); //deep copy, doesn't change ownership, EXPENSIVE
        //without clone guess wouldn't be usable

    //&mut a is a mutable pointer to a, since function excects &mut
    //can be only one &mut to var at scope
    change(&mut a);
    let g = &a;
    //can't do this now
    //let h = &mut a;
    println!("{}", g);
    //ref scope starts where introduced and continues until where last used
    //Non-Lexical Lifetimes - g not used anymore, so considered as out of scope
    //BUT normally can't do mut ref after a ref or after another mut ref
    //the causes data races - when two points in the program manipulate same data object
    //so here, after g was used(because it's not used anymore), we can:
    let h = &mut a;
    //SLICES are pointers
    let _fn = first_name(h);
    println!("{}", _fn);
    let k = "Hello World"; //String Literals are SLICES
    //they have len but no capacity
    let _fn2 = first_name(&k); //k and &k is the same
    println!("{}", _fn2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    println!("{}", _fn); //even though we shadow a, pointer still points
    println!("{}", h);
}

fn change(some_string: &mut String) {
    //even if some_str is mut, the pointer has to be mut if we want to change value
    some_string.push_str(" Bugakov");
}

//str allows both str and String
fn first_name(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]; //full slice
}