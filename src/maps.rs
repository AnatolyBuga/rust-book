use std::collections::HashMap;

pub fn maps(){
    //all keys same type
    //all values same type
    let mut sc = HashMap::new();
    let a = String::from("A");
    let b = String::from("B");
    sc.insert(a, 10);
    sc.insert(b, 1);
    println!("hashMap: {:?}", sc);
    //hashmap takes ownership, so a and b no longer valid

    let t = vec!["a", "b"];
    let score = vec![10, 50];
    //use <> here as rust doesn't know what to collect into
    let mut map: HashMap<_, _> = t.into_iter().zip(score.into_iter()).collect();
    println!("another map: {:?}", map);
    map.get("a");
    sc.get(&String::from("A"));
    //insert C if it doesn't exist, returns a pointer to the place in the map
    sc.entry(String::from("C")).or_insert(17);
    for (k, v) in &sc {
        println!("{}: {}", k, v);
    }
    let txt = "a b c";
    for w in txt.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    println!("another map: {:?}", map);
}