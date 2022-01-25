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
    variables::variables();

    //Errors
    //errors::read_username_from_file_short();

    //Generics
    //generics1::generics();

    //Traits
    //traits::traits();

    //Lifetimes
    //lifetimes::lifetimes()
}
