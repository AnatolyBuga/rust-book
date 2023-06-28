use serde::{Serialize, Deserialize};


pub fn srd() {
    let point = Point { x: 1, y: 2 };
    let s = S{f: 1.0};
    let a = E::A("Anatoly".into());
    
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    let ss = serde_json::to_string(&s).unwrap();
    let aa = serde_json::to_string(&a).unwrap(); // E::B cannot be ser due to skip
    
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
    println!("serialized = {}", ss);
    println!("serialized = {}", aa);
    
    // Convert the JSON string back to a Point.
    let deserialized = serde_json::from_str::<Point>(&serialized).unwrap();
    //Equivallent
    //let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    let sds = serde_json::from_str::<S>(&ss).unwrap();

    let z = 32;
    
    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    println!("deserialized = {:?}", sds);
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)] // Debug needed for debug print
#[serde(rename_all="lowercase")] // rename_all for both ser and deser
#[serde(rename(serialize = "new_ser_name", deserialize = "new_de_name"))] //or do like this if different for ser vs deser
#[serde(deny_unknown_fields)]  // <-- this is a container attribute - apply to struct or enum declaration
//#[serde(bound = "T: Eq" + std::default::Default)]
struct S {
    #[serde(default)]  // <-- this is a field attribute - apply to field (field Type must impl std::default::Default)
    //when deser, missing fields will be filled from struct's impl of Default
    f: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")]  // <-- this is also a container attribute - apply to struct or enum declaration
enum E {
    #[serde(rename = "a")]  // <-- this is a variant attribute - apply to variant of an enum
    #[serde(alias = "Ant")] // this will deser form given name A and alias Ant 
    A(String),
    #[serde(skip)] //never ser or deser
    B(String)
}