pub fn if_sample_one() {
  let number = 7;
  if number < 10 {
    println!("the number is less than 10");
  }
}

pub fn if_logical_sample() {
  let number = 25;
  if number % 5 == 0 && number % 2 == 0 {
    println!("the number is divisible by both 5 and 2")
  } else if number % 5 == 0 {
    println!("the number is only divisible by 5")
  } else {
    println!("the number is not divisible by 5")
  }
}

pub fn using_if_in_let_stmt() {
  let condition = true;
  let number = if condition { 10 } else { 5 };
  println!("the value of number is {}", number)
}
