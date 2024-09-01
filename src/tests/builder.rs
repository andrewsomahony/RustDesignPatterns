mod number_adder;

use crate::builder::IBuilder;
use crate::tests::builder::number_adder::NumberAdder;
// Our unit test method

#[test]
fn builder_test() {
  // Create our number adder

  let mut addition_builder :NumberAdder =
    NumberAdder::default();

  // Set our number adder input values and call
  // "build" to execute the addition

  let added_value :u64 =
    addition_builder
      .set_value1(
        12
      )
      .set_value2(
        14
      )
      .build();

  // Make sure our addition was correct

  assert_eq!(
    added_value,
    26
  );
}