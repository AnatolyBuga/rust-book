//all collections are stored on HEAP
mod vectors;
mod maps;
mod strings;

mod errors;
mod structs;
mod structs2;
mod generics1;
mod traits;
mod traits2;
mod lifetimes;
mod lifetimes_generics_traitbounds;
mod variables;
mod closures1;
mod iterators;
mod smart_pointers;
mod smart_pointers2;
mod smart_pointers3;
mod smart_pointers4;
mod smart_pointers5;

//main that ran succesfully returns 0, else some integer
//can also return Result
//Box<dyn Error> is any kind of  Error
//if returns that, must end with Ok(())
fn main() {
    //structs
    //structs::structs();

    //Collections
    //vectors::vectors();
    //strings::strings();
    //maps::maps();
    //variables::variables();

    //Errors
    //errors::read_username_from_file_short();

    //Generics
    //generics1::generics();

    //Traits
    //traits::traits();

    //Lifetimes
    //lifetimes::lifetimes()

    //Closures
    //closures1::closures()

    //Iterators
    //iterators::iterators();

    //Smart pointers
    //smart_pointers::_box();
    //smart_pointers2::_rc();
    //smart_pointers4::_more_pointers();
    //smart_pointers5::inf_cycle();
    smart_pointers5::tree();
}
