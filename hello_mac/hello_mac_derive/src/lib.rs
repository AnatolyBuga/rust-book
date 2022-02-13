extern crate proc_macro; //comes with Rust
use proc_macro::TokenStream;
use quote::quote; //turns syn data struct back into code
use syn; //Rust code from string into a data structure

///this is called by #[derive(HelloMacro)] because of proc_macro_derive
#[proc_macro_derive(HelloMacro)] //matches Trait name
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a repr of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    //build trait impl 
    impl_hello_macro(&ast)
}

// ast will look smth like that:
/*
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
*/
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name { //#name is value of var name
            fn hello_macro() {
                //stringify converts expression into string without evaluating it, at compile time
                println!("Hello, Mac! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into() // convert quote into TokenStream
}
