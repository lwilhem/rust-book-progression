use std::io;

fn main() {
  println!("Hello, world!");
  fahrenheit_to_celsius();
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

  println!("{celsius}")
}
