// Simple struct to store data, for seeing if we have data after
// various executions and such

// This should be a mediator, but we are only using this for unit testing,
// for a way to just store data and retrieve it in an OOP fashion, so
// it is ok to not use the design patterns that we are testing at the same
// time :D

// Derive the implementation of the "Default" trait
#[derive(Default)]
pub struct DataStorage<DataType> {
  pub data : DataType
}
