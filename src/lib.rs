// Declare all our Design Pattern modules

pub mod builder;
pub mod chain_of_responsibility;
pub mod command;
pub mod event_manager;
pub mod mediator;
pub mod strategy;

// Declare our tests module, only visible when we compile a
// test exe
#[cfg(test)]
mod tests;





