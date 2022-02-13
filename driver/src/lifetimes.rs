//lifetime annotations tell Rust how generic lifetime params of multiple references
//relate to each other
//Rust borrow checker compares lifetimes
pub fn lifetimes() {

    let str1 = String::from("abcd");
    let str2 = "xyz";

    let r = longest(str1.as_str(), str2);
    println!("The longest string is {}", r);
    //Can't do this:
    let result;
    {
        //Why Works here? - because str has 'static lifetime? ie lives for the lifetime of program
        //let s = "hello";
        let s = String::from("hello");
        result = longest(str1.as_str(), s.as_str())
    }
    //result might be a pointer to s which is out of scope
    //println!("Result is out of scope: {}", result);
    let novel = String::from("Anatoly Bugakov");
    let first: &str = novel.split(' ').next().expect("Could not find a space.");
    let i = ImportantExcerpt{
        //shortcut for part: part,
        part: first
    };
    //here first is a reference to first part on novel.
    //i holds ref to first part of novel.
    //novel doesn't go out of scope until i

    //'static lives for the runtime of the program
    let st: &'static str = "I live for ever";

}

//we don't want longest to take ownership, so we pass string slices (which are references)
//but Rust doesn't know if &str returns to x or y
//lifetimes of both params and lifetime of returned ref are related s.t.
//RETURNED REF WILL BE VALID AS LONG AS BOTH PARAMS ARE (as long as 'a lasts)
//Rust needs to know this because of the borrow checker
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //here 'a is smallest of lifetimes of x or y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//no need to specity lifetime of y, since compiler doesn't need to know that
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//Structs can hold pointers (instead of owned types)
//But in that case need to add Lifetimes to every ref
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn lvl(&self) -> i32{
        7
    }
    fn an(&self, ann: &str) -> &str {
        //Rule 3 - str gets lifetime of self
        println!("Announcement: {}", ann);
        self.part
    }
}

//this should be
//fn first_word<'a>(s: &'a str) -> &'a str {
//but Rust compiler is smart enough to figure that out
//patterns programmed into Rust's compiler - lifetime elision rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//Rule 1: Each param that is a ref gets its own lifetime param. eg:
//fn foo<'a>(x: &'a i32) {}
//fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {}

//Rule 2: if one input lifetime param, that is assigned to all output lifetime params. eg:
//fn foo <'a>(x: &'a i32) -> &'a i32 {}

//Rule 3: if mult lifetime params, but one of the is &self or &mut self, the lifetime of self
//is assigned to all output lifetime params

//If after applying 3 rules compiler can't figure out lifetime of the output, it throws error
