// Create a struct that will perform number addition

use crate::builder::IBuilder;

#[derive(Default)]
pub(super) struct NumberAdder {
  _value1 :u64,
  _value2 :u64
}

// Implement our number addition struct

impl NumberAdder {
  pub(super) fn set_value1(
    &mut self,
    value1 :u64
  ) -> &mut NumberAdder {
    self._value1 =
      value1;
    return self;
  }

  pub(super) fn set_value2(
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