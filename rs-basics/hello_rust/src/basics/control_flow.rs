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

#[allow(dead_code)]
pub fn infinite_loop() {
  loop {
    println!("this will run forever!")
  }
}

pub fn break_loop() {
  let mut count = 0;

  loop {
    count += 1;
    println!("count: {}", count);

    if count == 5 {
      break;
    }
  }

  println!("Loop ended at count: {}", count)
}

pub fn returning_values_from_loop() {
  let mut count = 0;
  let result = loop {
    count += 1;

    if count == 10 {
      break count * 2;
    }
  };

  println!("The result is: {}", result)
}
