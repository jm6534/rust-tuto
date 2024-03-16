#[derive(Debug)]
struct Rect {
  width: u32,
  height: u32,
}

impl Rect {
  fn square(size: u32) -> Self {
    Self { width: size, height: size }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
  
  fn width(&self) -> bool {
    self.width > 0
  }

  fn double_width(self) -> Self {
    Self { width: self.width * 2, ..self }
  }

  fn can_hold(&self, other: &Self) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let scale = 2;
  let rect1 = Rect { width: dbg!(30 * scale), height: 50 };
  let rect2: Rect = Rect { width: 30, height: 25 };

  println!("The rectangle is {:?}. area is {}, with is {}", rect1, rect1.area(), rect1.width());
  println!("hold?: {}, hold?: {}", rect1.can_hold(&rect2), rect1.can_hold(&rect2.double_width()));

  println!("it's a square! {:?}", Rect::square(5))
}
