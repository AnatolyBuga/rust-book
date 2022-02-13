pub fn generics() {
    let nl = vec![93, 79, 71, 68];
    let r = largest(&nl);
    let r2 = *(largest2(&nl));
    println!("Largest number: {}", r);
    println!("Largest number: {}", r2);
    let p1 = Point{ x: 5.0, y: 10.1};

    //let p2 = Point{ x:"Anatoly", y:'b'};
    let p2 = Point{ x: 2.2, y:'b'};
    let p3 = p1.mix(p2);
    println!("Point 3: {:?}", p3);
    //println!("Point 1: {:?}", p1);

}

//shortcut &v[i32] for &Vec<i32>
//l is slice of values of type T which implements Traits PartialOrd and Copy
fn largest<T: PartialOrd + Copy> (l: &[T]) -> T {
    //T might not implement Copy trait (unike i32 and char)
    //so we can't move the value out of l[0] into largest
    //So add Copy to Trait bounds
    let mut largest = l[0];
    //notice &i: i32
    //pattern match items of l, which are pointers, then distruct them with &
    for &i in l {
        //we can only use T which have std::cmp::PartialOrd trait
        //ie which can be >, so we must include that in Trait bounds
        if i>largest {
            largest = i;
        }
    }
    largest
}

fn largest2<T: PartialOrd> (l: &[T]) -> &T {
    //T might not implement Copy trait (unike i32 and char)
    //so we can't move the value out of l[0] into largest
    //So add Copy to Trait bounds
    let mut largest = &l[0];
    //notice &i: i32
    //pattern match items of l, which are pointers, then distruct them with &
    for i in l {
        //we can only use T which have std::cmp::PartialOrd trait
        //ie which can be >, so we must include that in Trait bounds
        if *i>*largest {
            largest = i;
        }
    }
    largest
}


#[derive(Debug)]
struct Point<T, U>{
    x: T,
    y: U
}

enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
//when impying methods, has to be done like that
//such a method is declared on ANY inst of Point
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
//only for Point<f32, f32>
impl Point<f32, f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X, Y> Point<X, Y> {
    //have to use self and not &self, as x doesn't implement a copy, and can't be moved
    //because passing self.x to Point would pass the ownership
    fn mix<Z, W>(self, other_param: Point<Z, W>) -> Point<X, W>{
        Point {
            x: self.x,
            y: other_param.y
        }
    }
}