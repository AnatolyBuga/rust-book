/// can only make object-safe traits into trait objects
/// trait is object safe if all methods: 1) don't return Self
/// There are no generic type params
/// Because type (that impl trait) is forgotten through the use of trait obj
pub trait Draw{
    fn draw(&self);
    //fn clone(&self) -> Self; //needs to know Type to return it, but Box<dyn Draw> doesn't know type
}

///Component is a Vec of Trait Object.
pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
    //<dyn Draw> any type that implements Draw
    //Notice: works differently to generic type parameter T with trait bound.
    //Vec<<T>> can be only one T per instance, whereas trait objects allow multiple at runtime
    // If only 1 type expected use Vec<T>
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

///ALTERNATIVELY could do this, but only one T per inst
/// However, this is FASTER
struct Screen2<T: Draw> {
    components: Vec<T>,
}

impl<T> Screen2<T>
where T: Draw,
{
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

///Types which implement Draw
pub struct Button {
    pub w: u32,
    pub h: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw
        println!("Drawing Button {}", self.label)
    }
}

pub struct TextField {
    pub w: u32,
    pub h: u32,
    pub label: String,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        // code to draw
        println!("Drawing TextField {}", self.label)
    }
}