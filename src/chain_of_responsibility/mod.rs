pub trait IChainOfResponsibilityHandler<ObjectType> {
  fn handle(
    &self,
    object :&mut ObjectType
  );
}

pub mod list;