fn main() {
    let v = vec![1, 2, 3];

    println!("{}", v[99]); // panic: index out of bounds: the len is 3 but the index is 99
}
