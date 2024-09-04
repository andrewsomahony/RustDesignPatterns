// The "Strategy" design pattern is a way to handle data in a generic fashion.
// We aren't necessarily building new objects, but rather we are performing
// some sort of process on the input value, and returning an output value.

// Strategies can be generalized and functionality can be provided abstractly,
// which makes them quite powerful

pub trait IStrategy<'strategy_lifetime, InputValueType, OutputValueType> {
  fn execute(
    // The strategy can modify itself if it so pleases, such as
    // to execute a mediator or another design pattern
    &mut self,
    input_value :InputValueType
  ) -> OutputValueType;
}