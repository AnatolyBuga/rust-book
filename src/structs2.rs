pub fn new_user(email: String, username: String) -> User {
    User {
        email: email,
        username, //simplified, when field names are same
        active: true,
        sign_in_count: 10
    }
}

//can extend Rectangle outside of it's class
impl Rectangle{
    pub fn hello(&self) {
        println!("Func defined in main")
    }
}

pub fn plus_one(x: Option<u8>) -> Option<u8> {
    //both cases have to return u8
    match x {
        Some(i) => Some(i+1),
        //if x matches Option::None
        None => None, //can be None. Does nothing/returns Unit Value
    }
}

//this can be replaced with if let
pub fn redundant(x: &Option<u8>) {
    match x {
        Some(i) => println!("Some: {}", i),
        _ => ()
    }
}

#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    //Using String, not str for struct to Own the data
    pub email: String,
    //possible to use &str but requires "lifetimes"
    pub sign_in_count: u64,
}

//tuple struct
pub struct Point(pub i32, pub i32, pub i32);

//Unit like structs
pub struct UnitLikeStruct;

pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle{
    //short for self: &Self
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
    //method can have same name as attrib
    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated functions often used for constructors
    pub fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

//ENUMS have Variants
//ENUMS achieves same as creating multiple structs
pub enum IpAddrKind {
    V4, //variant - similar to unit struct
    V6
}

pub struct IpAddr{
    pub kind: IpAddrKind,
    pub address: String,
}
//Instead of using two, can use one:
pub enum IpAddrAlt {
    V4(u8, u8, u8, u8),
    V6(String)
}
//can define methods on enum
impl IpAddrAlt {
    pub fn call(&self) -> String{
        String::from("Hello")
    }
}
//Option is an Enum defined by standard library
//Everywhere where type is not Option<T>, you can assume variable is not None
pub enum Coin {
    Penny,
    TwoPence,
    FivePence,
    TenPence,
    TwentyPence,
    FiftyPence,
    Pound,
    TwoPounds(Country)
}

impl Coin {
    pub fn coin_value(&self) -> f32{
        match self {
            Coin::Penny => 0.01,
            Coin::TwentyPence => 0.02,
            Coin::FivePence => 0.05,
            Coin::TenPence => 0.1,
            Coin::TwentyPence => 0.2,
            Coin::FiftyPence => 0.5,
            Coin::Pound => {
                println!("One Quid");
                1.0
            },
            Coin::TwoPounds(country) => {
                ///country binds, ie is the Country associated with self coin
                println!("Two Quid from: {:?}", country);
                2.0},
            _ => panic!() //when don't want to use value of other use _
        }
    }
}

#[derive(Debug)]
pub enum Country {
    England,
    Scotland,
    Wales,
    NorthernIreland
}