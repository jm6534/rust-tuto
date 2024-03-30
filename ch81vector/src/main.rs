
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    for i in &v2 {
        v1.push(*i);
    }

    println!("{}", &v1[0]);
    // println!("{}", &v1[100]); // panic: index out of bounds
    println!("{:?}", v1.get(100)); // get method returns Option<&T>

    println!("{:?}", v1);
    println!("{:?}", v2);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

}
