pub fn strings() {
    //remember str is a slice, ie a reference/pointer
    let mut a = "Anatoly".to_string();
    //push takes a char
    a.push('!');
    println!("a: {}", a);
    // + calls an add function on a, and takes ownership, smth like this:
    //fn add(self, s: &str) -> String {
    //Notice: compiler coerces &String to &str
    //because self, not &self, a will be moved
    //so a no longer valid
    let b = a + " Bugakov";
    println!("a is now b: {}", b);
    //String is a wrapper over Vec<u8>, so some sting might be stored like this:
    //[224, 101, 159]
    //cannot do b[0], Rust doesn't know how many bytes each char is
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //s takes first 4 bytes. Cirilics are 2bytes per char, so s is Зд
    println!("4 bytes: {}", s);
    //zip similar to python
    for it in hello.chars().zip(hello.bytes()) {
        println!("{}-{}", it.0, it.1);
    }
}