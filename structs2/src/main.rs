struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
  let black = Color(9,8,7);
  let origin = Point(3,4,5);

  println!("{}, {}, {}, {}", black.0, black.1, origin.0, origin.1)

  let subject = AlwaysEqual;
  let subject2 = AlwaysEqual;

  println!("{}", subject == subject2)
}
