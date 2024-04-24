use std::cmp::PartialOrd;
use std::marker::Copy;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

#[derive(Debug)]
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }

  fn y(&self) -> &U {
    &self.y
  }
}

impl Point<f64, f64> {
  fn distance(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

impl<T, U> Point<T,U> {
  fn mixup<R,S>(self, other: Point<R, S>) -> Point<T, S> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let num_list = vec![34, 50, 25, 100, 65];
  let result = largest(&num_list);
  println!("The largest number: {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char: {}", result);

  let int_p = Point { x: 5, y: 10 };
  let float_p = Point { x: 3.3, y: 4.4 };

  println!("x: {}, y: {}", int_p.x(), int_p.y());
  println!("x: {}, y: {}", float_p.x(), float_p.y());

  println!("Distance: {}", float_p.distance());
  // println!("Distance: {}", mixed_p.distance()); // error: no method named `distance` found for struct `Point<{integer}, {float}>` in the current scope

  let mixed_p = int_p.mixup(float_p);
  println!("x: {}, y: {}", mixed_p.x(), mixed_p.y());
}
