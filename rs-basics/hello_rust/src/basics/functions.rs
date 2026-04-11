#![allow(dead_code)]
#![allow(non_snake_case)]

pub fn sample_function() {
  println!("hello world 3");
}

pub fn parameter_sample(name: &str) {
  println!("Hello: {}", name)
}

pub fn calculate_area(width: i32, height: i32) {
  let (result, message) = calculated_area(width, height);
  print!("{} - {}", result, message)
}

fn calculated_area(width: i32, height: i32) -> (i32, String) {
  let result = width * height;
  (result, format!("Area is {}", result))
}

pub fn expression_sample() {
  let y = {
    let x = 1 + 3;
    x + 1
  };
  println!("value of y is {}", y)
}
