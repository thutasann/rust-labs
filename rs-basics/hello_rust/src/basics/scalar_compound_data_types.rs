pub fn scalar_compound_data_types() {
  integers();
  floating_integers();
  booleans();
  chars();
}

fn integers() {
  let decimal = 98_002;
  let hex = 0xff;
  let octal = 0o77;
  let binary = 0b1111_0000;
  let byte = b'A'; // Byte(u8 only)

  println!("{}", decimal);
  println!("{}", hex);
  println!("{}", octal);
  println!("{}", binary);
  println!("{}", byte);
}

fn floating_integers() {
  let x: f64 = 2.0; // f64
  let y: f32 = 3.0; // f32
  println!("{}", x);
  println!("{}", y);
}

fn booleans() {
  let t = true;
  let f: bool = false; // with explicity type annotation
  println!("{}", t);
  println!("{}", f);
}

fn chars() {
  let c = 'z';
  let z: char = 'Z'; // with explicit type annotation
  let heart_eyed_cat = '😻';
  println!("{}", c);
  println!("{}", z);
  println!("{}", heart_eyed_cat);
}
