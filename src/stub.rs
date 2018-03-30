#[derive(Clone, Copy, Default, Debug)]
#[repr(C, align(2))]
pub struct f16_stub(pub u16);

#[derive(Clone, Copy, Default, Debug)]
#[repr(C, align(4))]
pub struct f16x2_stub(pub u16, pub u16);

pub trait FloatStub: Sized {
  type Inner;

  fn from_parts(negative: bool, exponent: Self::Inner, fraction: Self::Inner) -> Self;
}

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

impl f16_stub {
  #[inline]
  fn _zero() -> f16_stub {
    f16_stub(0)
  }

  #[inline]
  fn _one() -> f16_stub {
    f16_stub::from_parts(false, 0, 0)
  }
}
