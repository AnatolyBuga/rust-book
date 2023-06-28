use std::str::FromStr;

pub fn execute() {
    let a = get_count_item("1a Anatoly 4");
    println!("{:?}", a);
}
/// "1 Anatoly"
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    // Iterates over the "pair"
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}