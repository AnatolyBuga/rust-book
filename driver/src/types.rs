//! Type alias,
type kilo = i32; //kilo is type synonym for i32
//no benefits like those of New Type pattern
//Used mainly to reduce repition
use std::fmt;
use std::io::Error;
type Heavy = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &mut [u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Formatter) -> Result<()>;
}

//RUst has a type named ! - empty/never type
/// never returns a value - called diverging
//fn boo() -> ! {};
//ex - continue, panic!, print! statement 
// str (not &str) is a DYNAMICALLY SIZED TYPE

pub fn types(){
    let v: Vec<u8> = vec![1, 2, 3];
    let a: &[u8] = &v[..]; //vec slices always used with &
    //let x: Heavy = Box::new(|| println!("hi"));
    fn takes_Heavy_type (x: Heavy) {};
    fn returns_Heavy_type() -> Heavy {
        Box::new(|| println!("hi"))
    };
    //Rust needs to know how much memory to allocate:
    // let s: str = "ani"; //doesn't work
    //Remember &str stores STARTING POSITION and LENGTH (normally & is one value - pointer)
    // but here two values (but we know pointer=usize, len=usize, so 2 x usize)
    // Dyn sized types are always behind a pointer(& or Box or Rc)
    // to know size - trait sized
}

//fn gen<T>(t: T) {}; is actually:
fn gen <T: Sized>(t: T) {}
//can relax that restriction for
//but then use pointer 
fn generic<T: ?Sized>(t: &T) {} //this syntax UNIQUE to Sized
//If you want to return closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


