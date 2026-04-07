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

  println!("\n=========== Shadowing ===========\n");
  basics::shadowing::shadowing_sample_one();
  basics::shadowing::shadowing_sample_two();
  basics::shadowing::shadowing_vs_mutability();
  // basics::shadowing::real_world_shadowing()

  println!("\n=========== Scalars & Compound Data Types ===========\n");
  basics::scalar_compound_data_types::scalar_compound_data_types();

  println!("\n=========== Tuples ===========\n");
  basics::tuples::tuple_sample_one();

  println!("\n=========== Arrays ===========\n");
  basics::arrays::array_sample_one();

  println!("\n=========== Macros ===========\n");
  basics::macros::marco_sample_one();
  basics::macros::macro_sample_two();

  println!("\n=========== Control flows ===========\n");
  basics::control_flow::if_sample_one();
  basics::control_flow::if_logical_sample();
  basics::control_flow::using_if_in_let_stmt();
  // basics::control_flow::infinite_loop();
  basics::control_flow::break_loop();
  basics::control_flow::returning_values_from_loop();
  basics::control_flow::label_sample_one();
  basics::control_flow::continue_sample_one();
  basics::control_flow::iterating_over_a_range();
  basics::control_flow::for_loop_sample_one();
  basics::control_flow::while_loop();
  basics::control_flow::combining_conditions_in_while_loop();
  basics::control_flow::break_continue_for_loop();
  basics::control_flow::break_continue_while_loop();
}
