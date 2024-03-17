
#[derive(Debug)]
enum IpAddr {
  //IPv4 and IPv6 is variants of IPAddress
  V4(u8, u8, u8, u8),
  V6(String)
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn call(&self) {
    println!("{:?} called!", self);
  }
}

fn main() {
  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  println!("{:?}, {:?}", home, loopback);

  let ms = [
    Message::Write(String::from("hello")),
    Message::Quit,
    Message::Move { x: 1, y: 2 },
    Message::ChangeColor(255,255,255)
  ];
  for m in ms {
    m.call();
  }

  let some_number = Some(5);
  let some_char = Some('c');
  let absent_number: Option<i32> = None;

  println!("{:?} {:?} {:?}", some_number, some_char, absent_number);
}
