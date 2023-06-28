//! patterns describe SHAPE of data
//! eg: MATCH ARMS
//! match VALUE {
//! PATTERN => EXPRESSION
//! PATTERN => EXPRESSION
//! }
//! IF LET MOVES VALUE, unless used _

pub fn patterns() {
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(c) = fav_color {
        println!("background: {}", c);
    } else if is_tuesday {
        println!("Today is Tuesday!")
        //shadow age
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Getting old")
        } else {
            println!("Still young")
        }
    } else{
        println!("Nothing matched")
    } 

    let mut v = vec![1, 2, 3];
    while let Some(top) = v.pop() {
        println!{"{}", top}
    }
    let v1 = vec!['a', 'b', 'c'];
    for (i, val) in v1.iter().enumerate() {
        println!("{} : at index {}", val, i)
    }

    foo(&(1,2));
    let x = Some(5);
    let y = 3;
    match x {
        // | or operator
        Some(1) | Some(2) => (),
        //Notice inner scope y shadows outerscope - use MATCH GUARD to compare to y. See below
        Some(y) => println!("Here"),
        _ =>(),
    }
    println!("x: {:?}, y: {:?}", x, y);
    match y{
        1..=5 => println!("Here"),
        _ => ()
    }

    let p = Point{x:0, y:7};
    let Point{x: a, y: b} = p;
    println!("a: {}, b: {}", a, b);
    println!("p: {:?}", p);
    // Shorter:
    let Point{x, y} = p;
    let msg = Message::ChangeColor(Color::Rgb(x , y, 3));

    // Destructuring and assign
    let pp = Point{x: 1, ..p};
    dbg!(pp);

    match msg {
        // .. to ignore the rest
        Message::ChangeColor(Color::Rgb(r, g, ..)) => println!("r: {}, g: {}", r, g),
        Message::Quit => println!("Quit"),
        Message::Move {x, y} => println!("x: {}, y: {}", x, y),
        Message::Write(string) => println!("{}", string),
        _ => (),
    }

    let( z, y, x) = (7, 15, "Anatoly");
    let mut x = Some(5);
    let y = Some(10);
    match (x, y) {
        _ => {x = y}
    };
    println!("x: {:?}, y: {:?}", x, y);

    match x {
        Some(a) if a < 5 => println!("Less than 5"),
        Some(a) => println!("Greater than 5"),
        None => ()
    }

    let x = Some(5);
    let y = 3;
    match x {
        // | or operator, can combine with MATCH GUARD
        Some(1) | Some(2) if y==2 => (),
        Some(i) if i ==y => println!("Here"),
        _ =>(),
    }

    let m = Color::Rgb(2, 5, 31);
    match m {
        Color::Rgb(r@  2..=7, ..) => println!("Here, r {}", r),
        _ => ()
    }
    let m2 = Message::Move{x: 3, y: 15};

    match m2 {
        //USEFULL WHEN IMPLEMENTING CONDITIONS AND NEED VALUE
        Message::Move{
            x: iks @ 3..=7,
            //y: igrik @ 6..=15
            y
        } => println!("Here x: {}, y: {}", iks, y),
        _ => ()
    }    
}

fn foo(&(x,y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
    }
#[derive(Debug)]
struct Point {
    x: i32, 
    y: i32
}

enum Message {
    //variant with no data
    Quit,
    //struct-like variant
    Move{x: i32, y: i32},
    //also tuple like
    Write(String),
    //tuple-like variant
    ChangeColor(Color)
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}