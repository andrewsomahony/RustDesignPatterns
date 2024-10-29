use crate::builder::IBuilder;
use crate::transformer::ITransformer;

#[derive(Default)]
pub(super) struct NumberToStringTransformer {
  // Store our input object
  _input_object :u64
}

impl IBuilder<String>
  for NumberToStringTransformer {
  fn build(
    &self
  ) -> String {
    // Return a new string using "to_string"
    return self._input_object.to_string();
  }
}

impl ITransformer<u64, String>
  for NumberToStringTransformer {
  fn set_input_object(
    &mut self,
    input_object: u64
  ) -> &mut dyn ITransformer<u64, String> {
    // Set our input object
    self._input_object =
      input_object;

    return self;
  }
}