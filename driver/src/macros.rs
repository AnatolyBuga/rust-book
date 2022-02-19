//! Macro is a code which writes another code - METOPROGRAMMING. Examples:
//! #[derive]; attribute like; function-like;
//!  macro expands before compiler interprets
//! macro can impl a trait of a given type (derive), function cannot
//! ^because of that, macros have to be defined BEFORE calling them
//! Macros compare a val(rust source code) to patterns(compare with structure of val)
//! macro_rules! to define a macro
//! 1) Declerative :
/// Will be depreciated
#[macro_export] //makes macro available whenever crate is available
macro_rules! vc {
    ( $( $x:expr),* ) => { //pattern $x is the name of the expression
        // , is separator
        // * means 0 or more of whatever came BEFORE *
        {
            let mut temp = Vec::new();
            $(  // for each part that matches $()
                // $x is the expr
                temp.push($x);
            )*
            temp
        }
    };
}

// 2) Procedural. More like funcitons
// (For tech reasons) their definitions must reside in their own crate with a special
// crate type
// i) custom drive ii) attribute-like iii)function like


//Derive implements a Trait with one associated function to a struct
use hello_mac::HelloMacro;
use hello_mac_derive::HelloMacro;

#[derive(HelloMacro)] // this calls hello_macro_derive
struct Pancakes;

pub fn macros() {
    println!("{}", "Anatoly");
    Pancakes::hello_macro();
}

// ii) Attribute like: 
// #[route(GET, "/")]
// fn index() {}
// And then
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

// iii) Function like 



