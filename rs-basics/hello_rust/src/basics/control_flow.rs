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

pub fn label_sample_one() {
  let mut outer_count = 0;

  'outer: loop {
    println!("Outer count: {}", outer_count);
    let mut inner_count = 0;

    loop {
      println!("Inner count: {}", inner_count);
      inner_count += 1;

      if inner_count == 2 {
        break; // this breaks the inner loop
      }

      if outer_count == 3 {
        break 'outer; // This break the outer loop
      }
    }

    print!("----------------");
    outer_count += 1;
  }

  println!("Outer loop ended at count: {}", outer_count);
}

pub fn continue_sample_one() {
  for i in 1..=5 {
    if i == 3 {
      continue; // skips the iteration when i == 3;
    }

    println!("The number is: {}", i);
  }
}

pub fn for_loop_sample_one() {
  let numbers = [1, 2, 3, 4, 5];

  for num in numbers {
    println!("The number is {}", num)
  }
}

pub fn iterating_over_a_range() {
  for num in 1..=6 {
    println!("The number is {}", num)
  }
}

pub fn while_loop() {
  let mut number = 3;

  while number == 0 {
    println!("{}", number);
    number -= 1;
  }

  println!("Liftoff!")
}

pub fn combining_conditions_in_while_loop() {
  let mut number = 4;
  let limit = 10;

  while number < limit && number % 2 == 0 {
    println!("the number is {}", number);
    number += 2;
  }

  println!("Loop terminated with number: {}", number);
}

pub fn break_continue_for_loop() {
  for num in 1..10 {
    if num == 5 {
      println!("Breaking at number : {}", num);
      break;
    }

    if num % 2 == 0 {
      continue;
    }

    println!("Number: {}", num)
  }
}

#[allow(dead_code)]
pub fn break_continue_while_loop() {
  let num = 0;

  while num < 10 {
    if num == 5 {
      println!("Breaking at number : {}", num);
      break;
    }

    if num % 2 == 0 {
      continue;
    }

    println!("Number: {}", num)
  }
}

pub fn match_sample_one() {
  let number = 3;
  match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),
  }
}

pub fn matching_multiple() {
  let number = 2;

  match number {
    1 | 2 | 3 => println!("One, Two, or three"),
    _ => println!("Something else!"),
  }
}
