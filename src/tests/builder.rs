use crate::builder::IBuilder;

// Create a struct that will perform number addition

#[derive(Default)]
struct NumberAdder {
  _value1 :u64,
  _value2 :u64
}

// Implement our number addition struct

impl NumberAdder {
  pub fn set_value1(
    &mut self,
    value1 :u64
  ) -> &mut NumberAdder {
    self._value1 =
      value1;
    return self;
  }

  pub fn set_value2(
    &mut self,
    value2 :u64
  ) -> &mut NumberAdder {
    self._value2 =
      value2;
    return self;
  }
}

// Implement the IBuilder trait for our NumberAdder

impl IBuilder<u64> for NumberAdder {
  fn build(&self) -> u64 {
    // Simply add our 2 numbers together
    return self._value1 + self._value2;
  }
}

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