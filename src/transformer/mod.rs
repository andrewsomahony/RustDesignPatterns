// The transformer is an extension of the builder interface.
// The design patterns are self-contained for the most part, but this is
// one that we find that we are using quite often, so we define it here
// as its own interface

use crate::builder::IBuilder;

// Extend our transformer from IBuilder, as that's what it is
pub trait ITransformer<InputObjectType, OutputObjectType>
  : IBuilder<OutputObjectType> {
  // A transformer allows the caller to set its input object.
  fn set_input_object(
    // We need our self input parameter to have the same lifetime as what we are
    // returning
    &mut self,
    input_object :InputObjectType
    // We can return an impl instead of a box, as the compiler can figure out when
    // we return "self" what size to return, and can infer everything from there, as
    // self is obviously implementing ITransformer
  ) -> &mut impl ITransformer<InputObjectType, OutputObjectType>;
}