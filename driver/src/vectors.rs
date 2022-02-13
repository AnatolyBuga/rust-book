pub fn vectors() {
    let mut v = Vec::new();
    //macro figures out T of vector
    let mut _v1 = vec![1, 2, 3];
    v.push(5); v.push(6);
    println!("My Vector: {:?}", v);
    //pointer to vec element
    let _second: &i32 = &_v1[1];

    match _v1.get(0) {
        Some(first) => println!("First element is: {}", first),
        None => println!("There is no first element")
    }
    //mutable borrow here, making prev immutable invalid
    _v1.push(4);
    //Cannot use second now:
    //println!("Second element: {}", _second);
    //if used _v1 without reference, _v1 would be moved (since doesn't implement copy)
    //so use &v1
    for i in &mut _v1 {
        //dereference operator
        *i += 100
    }
    println!("Vector now: {:?}", _v1);
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12)
    ];
    //If don't know the set of types the program will get at runtime
    //use trait instead of enum (covered later)
    println!("Size: {}", std::mem::size_of::<Vec<SpreadSheetCell>>());
    let vec_of_str: Vec<&str> = vec!["Anatoly Bugakov"];
    println!("First: {}", vec_of_str[0]);
}

pub enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
