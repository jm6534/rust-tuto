fn main() {
    let mut s = "initial contents".to_string();
    println!("{}", s);

    s.push_str(" and more");
    println!("{}", s);

    println!("{}", format!("{s} and more"));

    let hindi = String::from("हिंदी");
    // println!("{}", &hindi[0..1]); // panic: byte index 1 is not a char boundary; it is inside 'ह' (byte size 3)
    println!("{}", &hindi[0..3]); // it will print 'ह'

    for c in hindi.chars() {
        println!("{}", c);
    }

    for b in hindi.bytes() {
        println!("{}", b);
    }
}
