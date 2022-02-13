//! Can specify default generic type
//! ^can be used for operator overloading (eg + * etc)
//! Usefull when:
//! 1) You already have impl of the Trait and don't want to break it
//! by adding a required param
//! 2) to allow customization like Add example below
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PointA {
    pub x: i32, 
    pub y: i32,
}
/// trait Add specifies default for Right Hand Side
/// trait Add<Rhs=Self> {
/// type Output;
/// fn add(self, rhs: Rhs) -> Self::Output
/// }
/// so here we use default
impl Add for PointA {
    type Output = PointA;
    fn add(self, other: PointA) -> PointA {
        PointA { x: self.x + other.x, 
            y: self.y + other.y }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Millis(pub u32);
pub struct Meters(pub u32);

/// Here use Meters for Rhs param
impl Add<Meters> for Millis {
    type Output = Millis;
    fn add(self, other: Meters) -> Self{
        Self(self.0 + (other.0*10_000))
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human{
    // no need for pub here since implied by pub trait
    fn fly(&self) { 
        println!("Fighter Pilot")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard Up")
    }
}

impl Human {
    // has to be pub
    pub fn fly(&self) {
        println!("Human flies")
    }
}
/// has to be pub to use baby_name outside
pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;


impl Dog {
    //doesn't take self or &self, hence associated function, not a method
    //in associated func need to specify pub
    pub fn baby_name() -> String {
        String::from("DOG")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("ANIMAL")
    }
}

use std::fmt;

// SUPERTRAITS
pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); //relies on Display
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for PointA {} //PointA has to impl Display

impl fmt::Display for PointA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Possible to get over the ORPHAN rule using a wrapper 
/// aka Newtype Pattern
/// Can be used to spicify units (like Milis and Meters)
/// abstruct away API, like calc_average/assign_id(for eg People<Hashmap<id, str>> )
pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

use std::ops::Deref;

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0 //pointer to Vec
    }
}



