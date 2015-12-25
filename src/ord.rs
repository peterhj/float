use std::cmp::{Ordering};

/// Infimum NaN wrapper around `f32` (NaN is less than all other values).
#[derive(Clone, Copy, Default, Debug)]
pub struct F32InfNan(pub f32);

impl PartialEq for F32InfNan {
  fn eq(&self, other: &F32InfNan) -> bool {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        self.0 == other.0
      }
      (true, true)    => {
        true
      }
      _ => {
        false
      }
    }
  }
}

impl Eq for F32InfNan {
}

impl PartialOrd for F32InfNan {
  fn partial_cmp(&self, other: &F32InfNan) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for F32InfNan {
  fn cmp(&self, other: &F32InfNan) -> Ordering {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        if self.0 < other.0 {
          Ordering::Less
        } else if self.0 > other.0 {
          Ordering::Greater
        } else {
          Ordering::Equal
        }
      }
      (false, true)   => {
        Ordering::Greater
      }
      (true, false)   => {
        Ordering::Less
      }
      (true, true)    => {
        Ordering::Equal
      }
    }
  }
}

/// Supremum NaN wrapper around `f32` (NaN is greater than all other values).
#[derive(Clone, Copy, Default, Debug)]
pub struct F32SupNan(pub f32);

impl PartialEq for F32SupNan {
  fn eq(&self, other: &F32SupNan) -> bool {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        self.0 == other.0
      }
      (true, true)    => {
        true
      }
      _ => {
        false
      }
    }
  }
}

impl Eq for F32SupNan {
}

impl PartialOrd for F32SupNan {
  fn partial_cmp(&self, other: &F32SupNan) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for F32SupNan {
  fn cmp(&self, other: &F32SupNan) -> Ordering {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        if self.0 < other.0 {
          Ordering::Less
        } else if self.0 > other.0 {
          Ordering::Greater
        } else {
          Ordering::Equal
        }
      }
      (false, true)   => {
        Ordering::Less
      }
      (true, false)   => {
        Ordering::Greater
      }
      (true, true)    => {
        Ordering::Equal
      }
    }
  }
}

/// Infimum NaN wrapper around `f64` (NaN is less than all other values).
#[derive(Clone, Copy, Default, Debug)]
pub struct F64InfNan(pub f64);

impl PartialEq for F64InfNan {
  fn eq(&self, other: &F64InfNan) -> bool {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        self.0 == other.0
      }
      (true, true)    => {
        true
      }
      _ => {
        false
      }
    }
  }
}

impl Eq for F64InfNan {
}

impl PartialOrd for F64InfNan {
  fn partial_cmp(&self, other: &F64InfNan) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for F64InfNan {
  fn cmp(&self, other: &F64InfNan) -> Ordering {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        if self.0 < other.0 {
          Ordering::Less
        } else if self.0 > other.0 {
          Ordering::Greater
        } else {
          Ordering::Equal
        }
      }
      (false, true)   => {
        Ordering::Greater
      }
      (true, false)   => {
        Ordering::Less
      }
      (true, true)    => {
        Ordering::Equal
      }
    }
  }
}

/// Supremum NaN wrapper around `f64` (NaN is greater than all other values).
#[derive(Clone, Copy, Default, Debug)]
pub struct F64SupNan(pub f64);

impl PartialEq for F64SupNan {
  fn eq(&self, other: &F64SupNan) -> bool {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        self.0 == other.0
      }
      (true, true)    => {
        true
      }
      _ => {
        false
      }
    }
  }
}

impl Eq for F64SupNan {
}

impl PartialOrd for F64SupNan {
  fn partial_cmp(&self, other: &F64SupNan) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for F64SupNan {
  fn cmp(&self, other: &F64SupNan) -> Ordering {
    match (self.0.is_nan(), other.0.is_nan()) {
      (false, false)  => {
        if self.0 < other.0 {
          Ordering::Less
        } else if self.0 > other.0 {
          Ordering::Greater
        } else {
          Ordering::Equal
        }
      }
      (false, true)   => {
        Ordering::Less
      }
      (true, false)   => {
        Ordering::Greater
      }
      (true, true)    => {
        Ordering::Equal
      }
    }
  }
}
