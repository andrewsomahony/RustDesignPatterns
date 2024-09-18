// Command is one of the simpler design patterns, but strong in its indirect
// complexity.

pub trait ICommand {
  fn execute(
    // Commands can modify themselves if need be
    // !!! Is this true, not really sure per the pattern
    &mut self
  );
}