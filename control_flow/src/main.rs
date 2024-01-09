use std::io;

fn main() {
  println!("Hello, world!");
  fahrenheit_to_celsius();
  fibonnaci();
}

fn fahrenheit_to_celsius() {
  println!("input temps");

  let mut fahrenheit = String::new();

  io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Error During the Input Parse. Please input a valid number");

  let fahrenheit: f64 = match fahrenheit.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input. Please enter a number.");
      return;
    }
  };

  let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

  println!("{celsius}");
}

fn fibonnaci() -> i32 {
  let mut last_term = String::new();

  println!("input N term");

  io::stdin()
    .read_line(&mut last_term)
    .expect("Only use integer 32 positive value");

  let last_term: i32 = match last_term.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("err");
      return 0;
    }
  };

  match last_term {
    0 => 0,
    1 => 1,
    _ => {
      let mut prev = 0;
      let mut current = 1;

      for _ in 2..=last_term {
        let new_value = prev + current;
        prev = current;
        current = new_value;
      }

      println!("{current}");

      current
    }
  }
}
