use mdr_engine::MdrTransform;
use nalgebra::Vector4;

#[test]
fn identity_transform() {
  let identity_transform = MdrTransform::identity();
  let test_vector = Vector4::new(1.0, 1.0, 1.0, 1.0);

  let matrix = identity_transform.matrix();
  let inverse_matrix = identity_transform.inverse_matrix();

  assert_eq!(test_vector, matrix * test_vector);
  assert_eq!(test_vector, inverse_matrix * test_vector);
}
