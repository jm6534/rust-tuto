fn main() {
  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);

  println!("{} {} {}", s1, s1, s3);
}

fn gives_ownership() -> String {
  String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
  s
}
