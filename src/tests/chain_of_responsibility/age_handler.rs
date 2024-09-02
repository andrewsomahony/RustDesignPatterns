use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::tests::chain_of_responsibility::object_to_initialize::ObjectToInitialize;

pub(super) struct AgeHandler {
  _age_to_use :u64
}

impl AgeHandler {
  pub fn new(
    age_to_use :u64
  ) -> Self {
    Self {
      _age_to_use: age_to_use
    }
  }
}

impl IChainOfResponsibilityHandler<'_, ObjectToInitialize>
  for AgeHandler {
  fn handle(
    &self,
    object: &mut ObjectToInitialize
  ) -> Result<(), Box<dyn Error>> {
    // Set our object's age

    object.age =
      self._age_to_use;

    return Ok(());
  }
}