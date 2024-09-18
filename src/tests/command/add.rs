// Use our number adder from our builder in a command

use crate::command::ICommand;
use crate::tests::data::data_storage::DataStorage;
// Make a struct to hold the values required to run our command

pub(super) struct AddCommand<'struct_lifetime> {
  _number1 :u64,
  _number2 :u64,
  // For storing our add result
  _data_storage :&'struct_lifetime mut DataStorage<u64>
}

// Implement our AddCommand

impl<'struct_lifetime> AddCommand<'struct_lifetime> {
  pub(super) fn new(
    number1 :u64,
    number2 :u64,
    data_storage :&'struct_lifetime mut DataStorage<u64>
  ) -> Self {
    Self {
      _number1: number1,
      _number2: number2,
      _data_storage :data_storage
    }
  }
}

// Implement the ICommand trait for our AddCommand struct

impl ICommand for AddCommand<'_> {
  fn execute(
    &mut self
  ) {
    // Simply set our data result to the sum of the
    // 2 numbers
    self._data_storage.data =
      self._number1 + self._number2;
  }
}