fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

struct ImportantExcept<'a> {
  part: &'a str,
}

impl <'a> ImportantExcept<'a> {
  fn level(&self) -> i32 {
    3
  }
  fn announce_and_return_part(&self, announce: &str) -> &str {
    println!("Attention please: {}", announce);
    self.part
  }
}

fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest: {}", result);

  let ie = ImportantExcept { part: string2 };
  println!("ImportantExcept: {}", ie.announce_and_return_part("Hello"));
}
