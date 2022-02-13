pub fn variables() {
    let a = 1;
    let x: u32 = 100;
    println!("The value of x is: {}", x);
    let x = match x.checked_add(x) {
        Some(num) => num,
        None => panic!(),
    }; //shadowing, ie creating a new variable
    println!("The value of x is: {}", x);
    {
        //x = x*2; can't do, x is immut
        let x = x*x;
        println!("Inner x: {}", x)
    }
    let tup = (500, 6.4, 1); //tuple can have different types; can't change size
    let (_x, _y, _z) = tup;
    println!("The value of y is: {}, same as {}", _y, tup.1);
    let ar = [1, 2, 3]; //can't change size
    let ar2 = [3;3]; // [3,3,3]
    let b = ar[0];
    //a[4] is an error(rather than random value) because Rust is memory safe
    let z = {
        let x = 3;
        x + 1 //without ; at the end this is an EXPRESSION, with ; it's a STATEMENT
    }; // z is 4
    println!("z is: {}", z);
    if add(x as isize, 4) < 10 {
        println!("Add is less than 10");
    } else if add(x as isize, 4) > 10 {
        println!("Add is greater than 10");
    }
    else {
        println!("Add is equal to 10");
    }

    let mut count = 0;
    let res = 'loop1: loop{
        println!{"count: {}", count};
        let mut rem = 10;

        loop {
            println!("rem: {}", rem);
            if rem == 8 {
                break;
            }
            if count == 2 {
                break 'loop1 rem*2;
            }
            rem -= 1;
        }
        count += 1;
    };

    println!("res: {}", res);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


}

fn add(x: isize, y: isize) -> isize{
    /* hello
    this is a comment */
    x + y //no ;
    // if ; - error as STATEMENT does not evaluate to value
}