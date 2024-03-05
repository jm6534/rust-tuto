fn main() {
    let x = 3;
    let y = 4;
    println!("value is {}", another(x, y));
}

fn another(x: isize, y: isize) -> isize {
    return x * y;
}
