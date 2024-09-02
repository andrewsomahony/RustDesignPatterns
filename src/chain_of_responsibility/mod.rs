use std::error::Error;

pub trait IChainOfResponsibilityHandler<'struct_lifetime, ObjectType> {
  fn handle(
    &self,
    object :&mut ObjectType
  ) -> Result<(), Box<dyn Error>>;
}

pub mod list;