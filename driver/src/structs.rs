//use super::structs2::{Rectangle, User};
use super::structs2::*;

pub fn structs() {
    let mut user1 = new_user( String::from("anatoly@example.com"),
        String::from("anatoly123"));

    //access by dot
    user1.email = String::from("anatoly@mail.ru");

    let mut user2 = User {
        email: String::from("buga@yandex.com"),
        ..user1 //shortcut to use other params from user1
    };
    //active and sign_in_count datatypes implement Copy trait
    //String doesn't, so using ..user1 MOVES username
    //and hence user1 is not usable anymore

    //behaves like tuple
    let p = Point(1, 2,3 );

    let r = Rectangle{ width:30, height:50};
    println!("Rectangle area: {}", r.area());
    println!("Width: {}", r.width);
    println!("Width > 0: {}", r.width());

    //if p1 is a pointer to obj, then you still can do
    //p1.area() instead of (&p1).area()
    let b = &r;
    println!("Pointer area: {}", (&b).area());
    println!("Pointer area: {}", b.area());

    let r2 = Rectangle{ width:29, height:0};
    let r3 = Rectangle{ width:20, height:55};

    //r2 r3 are immutable because we only read them
    //passing pointers in case we want to use r2 r3 again
    println!("r can fit r2: {}", r.can_fit(&r2));
    println!("r can fit r3: {}", r.can_fit(&r3));

    let mut sq = Rectangle::square(3);
    sq.hello();
    //ENUMS
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    //V4/6 are functions which return IpAddrAlt
    let home2 = IpAddrAlt::V6(String::from("::1"));
    let home3 = IpAddrAlt::V4(127, 0, 0, 1);

    let mut opt: Option<u8> = Some(1);
    //Option val can be set to None, but a pointer/variable cannot
    opt = None;
    opt = Some(1);

    let pnd = Coin::Pound;
    let my_val = pnd.coin_value();
    println!("My Val: £{}", my_val);
    let two = Coin::TwoPounds(Country::England); //return enum Coin
    let my_val2 = two.coin_value();
    println!("My Val: £{}", my_val2);

    //match for Option
    let opt2 = plus_one(opt);

    //IF LET is sythetic sugar for MATCH
    redundant(&opt2);
    //SAME AS redundunt function
    if let Some(i) = opt2 {
        println!("Some: {}", i)
    }
    let mut count = 0;
    //can include else
    if let Coin::TwoPounds(stt) = two {
        println!("Two Quid made in: {:?}", stt)
    } else {
        count += 1
    }
}

