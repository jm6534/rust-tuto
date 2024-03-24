use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("{:?}", rand::thread_rng().gen_range(1..=101));
    let mut map = HashMap::new();
    map.insert("1", 1);
    println!("{:?}", map);
}
