//! External Mutability = Inherited mutability - defined by the owner and inherited to everything
//! you can reach from the owner
//! Essentially, Rc/Box let's you use a value in few/pne place(s) (multiple pointers)
//! RefCell and Cell provide mutability where it is not allowed by default

use std::rc::Rc;
use std::cell::{RefCell, Cell, Ref};

pub fn _more_pointers() {
    let mut p = Point{x:10, y:11}; //External Mutability
    p.x = 12;
    p.y = 0;
    let q = Point{x:3, y:4};
    //q.x = 33 ERROR
    let px: &u32 = &p.x; //Ok
    let py: &mut u32 = &mut p.y; //ok because p is mut
    let qx: &u32 = &q.x; //Ok
    //let qy: &mut u32 = &mut q.y ERROR because q is not mut
    let x1 = Rc::new(1); //ref to 1
    let x2 = Rc::clone(&x1); //another ref to 1
    let x3 = Rc::clone(&x2); //yet another ref to 1
    //x1 contains INTERNAL reference
    //inst of Rc contains INTERNAL reference counter which is modified(when new clone) and destroyed (when out of scope)
    //Hence inst of Rc has to modify itself even through owned by (stored inside) non-mut variable x1
    //This is achieved via internal mutability. eg UnsafeCell

    //Internal mutability means value in the cell can be modified without mut ref to the variable holding cell.
    //Eg Cell can contain COPY data and it can be mutated even if stored in non-mut location
    //interact with data via .get .set etc
    let c = Cell::new(2);
    c.get(); //copy of val inside cell
    c.set(2) ; // sets new val inside cell
    //c.take(); //Moves inside value
    //Cell cannot tell you what's contained inside via a ref
    println!("{:?}", c);
    //RefCell can contain non-COPY data and can give you &mut pointers to it's inside value
    //RefCell - multiple mut ref to a val inside an immut cell

    //Rc provides shared ownership of inside cell, and therefore can't be mutated
    //Box provides exclusive ownership, and therefor can be mutated.

    let ultra = Rc::new(RefCell::new("Anatoly"));
    let ultra2 = Rc::clone(&ultra);
    let ultra3 = Rc::clone(&ultra2);

    *ultra.borrow_mut() = "Viktoria";

    println!("ultra: {:?}", ultra);
    println!("ultra2: {:?}", ultra2);
    println!("ultra3: {:?}", ultra3);

    let alpha = RefCell::new(Rc::new(17));
    let beta = Rc::clone(&alpha.borrow_mut());
    let gamma = Rc::clone(&alpha.borrow_mut());
    *alpha.borrow_mut() = Rc::new(34);

    println!("alpha: {:?}", alpha);
    println!("beta: {:?}", beta);
    println!("gamma: {:?}", gamma);

    let ultima: Rc<Cell<u32>> = Rc::new(Cell::new(22));
    let ultima2 = Rc::clone(&ultima);
    let ultima3 = Rc::clone(&ultima);
    println!("ultima before: {:?}", ultima);
    (*ultima).get();//copies, so no effect
    (*ultima).take(); //moves
    println!("ultima after: {:?}", ultima);
    println!("ultima2: {:?}", ultima2);
    println!("ultima3: {:?}", ultima3);


}

struct Point {x: u32, y: u32}

//To point List::Cons to another List::Cons need Box
//To have two List::Cons pointing to the same List::Cons need Rc
//To be able to mutate a List::Cons to which other List:Cons are pointing, need RefCell
