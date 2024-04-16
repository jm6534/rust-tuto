use std::fs;
use std::io;

fn main() {
  // let f = File::open("hello.txt");
  // let f_ok = match f {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Cannot create file: {:?}", e),
  //     },
  //     other_error => panic!("Cannot open file: {:?}", other_error),
  //   },
  // };

  // let f = File::open("hello.txt")
  // .expect("hello2.txt should be included in the project");

  // println!("{:?}", f);

  let username = read_usernane_from_file();
  match username {
    Ok(username) => println!("Username: {}", username),
    Err(e) => panic!("Cannot read username: {:?}", e),
  }
}

fn read_usernane_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
