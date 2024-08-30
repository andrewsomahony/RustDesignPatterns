pub trait IBuilder<ObjectType> {
  fn build(
    &self
  ) -> ObjectType;
}