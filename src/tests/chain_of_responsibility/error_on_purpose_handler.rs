use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::tests::error::test_error::TestError;

// Create a handler that deliberately returns an error instead of
// handling itself correctly

pub(super) struct ErrorOnPurposeHandler {
  _message :String
}

// Implementation of our ErrorOnPurposeHandler

impl ErrorOnPurposeHandler {
  pub(super) fn new(
    message :String
  ) -> Self{
    Self {
      _message: message
    }
  }
}

// Implementation of our ChainOfResponsibilityHandler for
// our error on purpose handler

impl<ObjectType> IChainOfResponsibilityHandler<ObjectType>
  for ErrorOnPurposeHandler {
  fn handle(
    &self,
    // Prefixed with an underscore to avoid an "unused variable" warning
    _object: &mut ObjectType
  ) -> Result<(), Box<dyn Error>> {
    // Return an error on purpose!

    Err(
      Box::new(
        TestError::new(
          self._message.clone()
        )
      )
    )
  }
}