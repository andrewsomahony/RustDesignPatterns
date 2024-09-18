use std::error::Error;

pub trait IChainOfResponsibilityHandler<ObjectType> {
  fn handle(
    &self,
    object :&mut ObjectType
  ) -> Result<(), Box<dyn Error>>;
}

pub mod list;