use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

pub fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");
    //this match can be rewritten with ? operator (synthetic sugar for this match)
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        //no need to return since this is the last expression
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

pub fn read_username_from_file_short() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    //to use ? return type of function must be Result
    //Note: Error? goes through "from" func. That "from" func converts error type to
    //type defined in our func
    //impl From<OtherError> for ReturnedError
    //? can be used with Options. Returns None early, if not None then
    // results in val inside Some AND CONTINUES RUNNING
}

pub fn read_username_from_file_even_shorter() -> Result<String, io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    //can be done in 1 line using standrard library
    //use std::fs
    //fs::read_to_string("hello.txt");
}

pub fn read_or_create_or_panic() {
    let f: Result<File, std::io::Error> = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        //if error, create
        Err(error) => match error.kind() {

            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e)
            }
            other_error=> panic!("File exists, but there is a problem with \
            opening/reading the file: {:?}", other_error)
        }
    };
    //SAME AS(explained later)
    let f = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })} else {
                panic!("Problem opening/reading the file: {:?}", err)
            }
        }
    );
    //.unwrap() returns <T> if Ok, else panics
    let f = File::open("dingdong.txt").unwrap();
    //.expect() similar to unwrap, let's choose error message
    let f = File::open("dingdong.txt").expect("Failed to open dingdong.txt");
    //for Option returns <T> if Some(T), else panics
}