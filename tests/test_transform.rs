use cgmath::{Matrix4, Vector3};
use mdr_engine::MdrTransform;

#[test]
fn default_transform() {
  let default_transform = MdrTransform::default();

  assert_eq!(default_transform.pos, Vector3::new(0.0, 0.0, 0.0));
  assert_eq!(default_transform.rot, Vector3::new(0.0, 0.0, 0.0));
  assert_eq!(default_transform.scale, Vector3::new(1.0, 1.0, 1.0));
}

#[test]
fn translation() {
  // Test translation of (1, 1, 1)
  const TRANSLATION_INPUT: MdrTransform = MdrTransform {
    pos: Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    },
    rot: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    scale: Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    },
  };

  let (matrix, inverse_matrix) = TRANSLATION_INPUT.get_matrix_and_inverse();

  assert_eq!(
    matrix,
    Matrix4::new(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., 1., 1., 1.)
  );
  assert_eq!(
    inverse_matrix,
    Matrix4::new(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., -1., -1., -1., 1.)
  );
}

#[test]
fn rotation_x() {
  const ROTATION_X_INPUT: MdrTransform = MdrTransform {
    pos: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    rot: Vector3 {
      x: 90.0,
      y: 0.0,
      z: 0.0,
    },
    scale: Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    },
  };

  let (matrix, inverse_matrix) = ROTATION_X_INPUT.get_matrix_and_inverse();
  assert_eq!(
    matrix,
    Matrix4::new(1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 1.)
  );
  assert_eq!(
    inverse_matrix,
    Matrix4::new(1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1.)
  );
}

#[test]
fn rotation_y() {
  const ROTATION_Y_INPUT: MdrTransform = MdrTransform {
    pos: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    rot: Vector3 {
      x: 0.0,
      y: 90.0,
      z: 0.0,
    },
    scale: Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    },
  };

  let (matrix, inverse_matrix) = ROTATION_Y_INPUT.get_matrix_and_inverse();
  assert_eq!(
    matrix,
    Matrix4::new(0., 0., -1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.)
  );
  assert_eq!(
    inverse_matrix,
    Matrix4::new(0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 0., 1.)
  );
}

#[test]
fn rotation_z() {
  const ROTATION_Z_INPUT: MdrTransform = MdrTransform {
    pos: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    rot: Vector3 {
      x: 0.0,
      y: 90.0,
      z: 0.0,
    },
    scale: Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    },
  };

  let (matrix, inverse_matrix) = ROTATION_Z_INPUT.get_matrix_and_inverse();
  assert_eq!(
    matrix,
    Matrix4::new(0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
  );
  assert_eq!(
    inverse_matrix,
    Matrix4::new(0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
  );
}

#[test]
fn scale() {
  // Test scale of (2, 2, 2)
  const SCALE_INPUT: MdrTransform = MdrTransform {
    pos: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    rot: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    scale: Vector3 {
      x: 2.0,
      y: 2.0,
      z: 2.0,
    },
  };

  let (matrix, inverse_matrix) = SCALE_INPUT.get_matrix_and_inverse();

  assert_eq!(
    matrix,
    Matrix4::new(2., 0., 0., 0., 0., 2., 0., 0., 0., 0., 2., 0., 0., 0., 0., 1.)
  );
  assert_eq!(
    inverse_matrix,
    Matrix4::new(0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1.)
  );
}
