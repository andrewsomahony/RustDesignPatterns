// The Mediator pattern mediates a piece of data
// by processing it with an object unknown to the caller.
// It is essentially an indirect connection between objects
// and interfaces, where the caller doesn't have to know how
// the data is actually processed, only that it has been processed

pub trait IMediator<DataType> {
  fn execute(
    &mut self,
    data :DataType
  );
}