use std::num::{Zero, One};

pub trait FloatStub: Sized {
  type Inner;

  fn from_parts(negative: bool, exponent: Self::Inner, fraction: Self::Inner) -> Self;
}

#[derive(Clone, Copy, Default, Debug)]
pub struct f16_stub(u16);

impl FloatStub for f16_stub {
  type Inner = u16;

  #[inline]
  fn from_parts(negative: bool, exponent: u16, fraction: u16) -> f16_stub {
    let sign = if negative { 0x8000 } else { 0 };
    let biased_exp = ((exponent + 15) & 0x1f) << 10;
    let fraction = fraction & 0x3ff;
    f16_stub(sign | biased_exp | fraction)
  }
}

impl Zero for f16_stub {
  #[inline]
  fn zero() -> f16_stub {
    f16_stub(0)
  }
}

impl One for f16_stub {
  #[inline]
  fn one() -> f16_stub {
    f16_stub::from_parts(false, 0, 0)
  }
}
