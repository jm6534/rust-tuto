#[derive(Debug)]
enum UsState {
    Alaska,
    Colorado,
    California
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    println!("{:?}", coin);
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(_) => Some(999),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::California)));

    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));
}
