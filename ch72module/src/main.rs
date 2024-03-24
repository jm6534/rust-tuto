use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let asparagus = Asparagus {
        name: String::from("Asparagus"),
        spears: 5,
    };
    println!("{:?}", asparagus);
}
