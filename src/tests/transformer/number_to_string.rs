use crate::builder::IBuilder;
use crate::transformer::ITransformer;

#[derive(Default)]
pub(super) struct NumberToStringTransformer {
  // Store our input object
  _input_object :u64
}

impl IBuilder<'_, String>
  for NumberToStringTransformer {
  fn build(
    &self
  ) -> String {
    // Return a new string using "to_string"
    return self._input_object.to_string();
  }
}

impl<'transformer_lifetime> ITransformer<'transformer_lifetime, u64, String>
  for NumberToStringTransformer {
  fn set_input_object(
    &'transformer_lifetime mut self,
    input_object: u64
  ) -> &mut impl ITransformer<'transformer_lifetime, u64, String> {
    // Set our input object
    self._input_object =
      input_object;

    return self;
  }
}