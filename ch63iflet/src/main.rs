fn some_else(o: Option<i32>, count: &mut u32) {
    if let Some(i) = o {
        println!("Some: {}", i);
    } else {
        *count += 1;
    }
}

fn main() {
    let mut count = 0;
    
    some_else(Some(5), &mut count);
    for _ in 0..3 {
        some_else(None, &mut count);
    }

    println!("Count: {}", count);
}
