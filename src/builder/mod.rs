pub trait IBuilder<'builder_lifetime, ObjectType> {
  fn build(
    &self
  ) -> ObjectType;
}