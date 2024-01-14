#[derive(Debug)]
struct Rectangle {
  width: i32,
  height: i32,
}

impl Rectangle {
  fn area(&self) -> i32 {
    self.width * self.height
  }
  fn can_hold(&self, rect: &Rectangle) -> bool {
    self.width > rect.width && self.height > rect.height
  }
  fn square(size: i32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  let square1 = Rectangle::square(100);

  println!("area of rec 1 - {}", &square1.area());
  print!("can rec1 hold r2 = {} \n", &square1.can_hold(&rect1));
  print!("can rec1 hold r2 = {} \n", &square1.can_hold(&rect2));
  print!("can rec1 hold r3 = {} \n", &square1.can_hold(&rect3))
}
