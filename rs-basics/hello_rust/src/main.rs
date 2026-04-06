mod basics;

fn main() {
  println!("\n=========== Hello World ===========\n");
  basics::comments::hello_world();
  basics::comments::hello_world_2();
  // basics::functions::sample_function();

  println!("\n=========== Vars ===========\n");
  basics::var::immutability_by_default();
  basics::var::mutable_var();
  basics::var::constants_sample_one();

  println!("\n=========== Scalars & Compound Data Types ===========\n");
  basics::scalar_compound_data_types::scalar_compound_data_types();

  println!("\n=========== Tuples ===========\n");
  basics::tuples::tuple_sample_one();

  println!("\n=========== Arrays ===========\n");
  basics::arrays::array_sample_one();
}
