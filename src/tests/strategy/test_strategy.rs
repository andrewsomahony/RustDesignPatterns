// This strategy will take a u64 as input and return a boolean
// signifying if it's equal to its value

use crate::strategy::IStrategy;

pub(super) struct TestStrategy<'strategy_lifetime> {
  // A reference to our test value
  // !!! We really don't need to have a reference to a primitive,
  // !!! but this test is also serving as a way to be more comfortable
  // !!! with Rust memory management, so we are using a reference for that
  // !!! reason.

  _test_value :&'strategy_lifetime u64
}

impl<'strategy_lifetime> TestStrategy<'strategy_lifetime> {
  pub(super) fn new(
    test_value :&'strategy_lifetime u64
  ) -> Self{
    Self {
      _test_value: test_value
    }
  }
}

impl IStrategy<'_, u64, bool> for TestStrategy<'_> {
  fn execute(
    &mut self,
    input_value: u64
  ) -> bool {
    return input_value == *self._test_value;
  }
}