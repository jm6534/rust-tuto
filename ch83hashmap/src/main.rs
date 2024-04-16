use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Green"), 20);

  println!("{:?}", scores);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let team = "Red".to_string();
  let score = 100;

  scores.insert(String::from(team.clone()), score);
  scores.insert(String::from("Blue"), 24);

  println!("{}", team); // owned
  println!("{:?}", scores);

  scores.entry(String::from("Blue")).or_insert(50);
  scores.entry(String::from("Yellow")).or_insert(60);

  println!("{:?}", scores);
}
