use std::rc::{Rc, Weak};
use std::cell::{RefCell, Cell};
use crate::smart_pointers5::List3::{Cons3, Nil3};

#[derive(Debug)]
enum List3{
    Cons3(i32, RefCell<Rc<List3>>),//Here can modify which List value a Cons is pointing to
    Nil3
}

impl List3 {
    fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
        match self {
            List3::Cons3(_, item) => Some(item),
            List3::Nil3 => None,
        }
    }
}


pub fn inf_cycle() {
    let a = Rc::new(Cons3(5, RefCell::new(Rc::new(Nil3))));//Rc<Cons(5, RefCell<Rc<Nill>>)>
    //a holds List(5, Nil)
    println!("a rc count: {}", Rc::strong_count(&a) );
    println!("a next item: {:?}", a.tail() );

    let b = Rc::new(Cons3(10, RefCell::new(Rc::clone(&a))));
    //List(10, a)
    println!("a rc count after b: {}", Rc::strong_count(&a) );
    println!("b rc count: {}", Rc::strong_count(&b) );
    println!("b next item: {:?}", b.tail() );

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b)
        //a List(5, b), creating a cycle:
    }
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    //Rc is only cleaned up after count is 0

    // !CYCLE!
    //println!("a next item: {:?}", a.tail())
    let k = Rc::new(5);
    let l = Rc::clone(&k);
    let k = Rc::clone(&l);
}

/// Building a Tree
/// value+children
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, //Node owns, RefCell allows to modify, Rc to share ownership via clone
    //(but the only way to access is through .borrow()
    parent: RefCell<Weak<Node>> //Can't be Rc because that would create ref cycle,
    //ie both would be pointing at each other
}

///Weak references don't express ownership relationship, and val is dropped even if weak ref count not 0
pub fn tree() {
    let leaf = Rc::new(Node{
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("leaf parent before = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong count: {}, \
              leaf weak   count: {}",
              Rc::strong_count(&leaf), Rc::weak_count(&leaf));


    let branch = Rc::new(Node{
        value: 100,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
    //Node in leaf has two owners: leaf and branch
    //can get children of the branch
    //But how to get parents of the leaf?
    //Notice Node owns children so cannot move out: let a = (*branch).children
    let a = (*branch).children.borrow();
    println!("{:?}", a);

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    //let b = *leaf.parent.borrow_mut(); - can'r do, doesn't implement Copy
    //creates Weak ref. Weak holds non-owning ref
    //weak upgrade returns Option<Rc<T>>
    //no guarantees of value being present
    println!("leaf parent after = {:?}", leaf.parent.borrow().upgrade());
    //upgrade returns an option
    // !If parent goes out of scope, weak count drops by 1 and leaf.parent becomes None

    println!("leaf strong count: {}, \
              leaf weak   count: {}",
              Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    println!("branch strong count: {}, \
              branch weak   count: {}",
              Rc::strong_count(&branch), Rc::weak_count(&branch));

}

pub fn inf_cycle2() {
    let c = Rc::new(1);
    let a = Rc::new(RefCell::new(Rc::clone(&c)));
    let b = Rc::new(RefCell::new(Rc::clone(&c)));

    //*a.borrow_mut() = Rc::clone(&b);
}