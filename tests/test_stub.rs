extern crate float;

use float::stub::*;
use std::mem::{align_of, size_of};

#[test]
fn test_align() {
  assert_eq!(align_of::<f16_stub>(), 2);
}

#[test]
fn test_size() {
  assert_eq!(size_of::<f16_stub>(), 2);
}

#[test]
fn test_align_x2() {
  assert_eq!(align_of::<f16x2_stub>(), 4);
}

#[test]
fn test_size_x2() {
  assert_eq!(size_of::<f16x2_stub>(), 4);
}
