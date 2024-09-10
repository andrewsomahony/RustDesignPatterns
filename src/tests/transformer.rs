use crate::tests::transformer::number_to_string::NumberToStringTransformer;
use crate::builder::IBuilder;
use crate::transformer::ITransformer;

// Import our number to string transformer

mod number_to_string;

#[test]
fn test_transformer() {
  // Create our number to string transformer

  let mut number_to_string_transformer :NumberToStringTransformer =
    NumberToStringTransformer::default();

  assert_eq!(
    number_to_string_transformer.set_input_object(
      10
    )
    .build(),
    "10"
  )
}