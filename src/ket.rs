use float_cmp::approx_eq;
use num_complex::Complex64;

/// A ket is a two-dimensional vector.
/// It has two components, "first" and "second".
/// These components, individually, are complex numbers.
#[derive(Debug, Copy, Clone)]
pub struct Ket {
  first: Complex64,
  second: Complex64,
}

// Let's define a couple of helper constants in the right type.
// These correspond to the real values 0 and 1, but are actually complex
// numbers with a real and imaginary parts
pub const COMPLEX_ZERO: Complex64 = Complex64 { re: 0.0, im: 0.0 };
pub const COMPLEX_ONE: Complex64 = Complex64 { re: 1.0, im: 0.0 };

/// A ket with the value [1, 0]. Analogous to the classical bit 0. Has the symbol |0>.
pub const KET_ZERO: Ket = Ket {
  first: COMPLEX_ONE,
  second: COMPLEX_ZERO,
};

/// A ket with the value [0, 1]. Analogous to the classical bit 1. Has the symbol |1>.
pub const KET_ONE: Ket = Ket {
  first: COMPLEX_ZERO,
  second: COMPLEX_ONE,
};

// Now we need to implement equality checking for our Ket
impl PartialEq for Ket {
  fn eq(&self, other: &Self) -> bool {
    self.first == other.first && self.second == other.second
  }
}
impl Eq for Ket {}

// Let's test that our equality checking works
#[test]
fn ket_zero_equal_to_itself() {
  assert_eq!(KET_ZERO == KET_ZERO, true)
}

#[test]
fn ket_one_equal_to_itself() {
  assert_eq!(KET_ONE == KET_ONE, true)
}

#[test]
fn ket_zero_not_equal_to_ket_one() {
  assert_eq!(KET_ZERO != KET_ONE, true)
}

//  Let's implement adding two Kets together
use std::ops::Add;
impl Add for Ket {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      first: self.first + other.first,
      second: self.second + other.second,
    }
  }
}

#[test]
fn ket_zero_add_ket_zero() {
  let sum = KET_ZERO + KET_ONE;
  assert_eq!(
    sum,
    Ket {
      first: COMPLEX_ONE,
      second: COMPLEX_ONE,
    }
  )
}

// Let's implement "scalar" multiplication for Ket;
// multiplying a Ket with a single imaginary number.
// Note that this also includes numbers with just a real part
use std::ops::Mul;
impl Mul<Complex64> for Ket {
  type Output = Ket;

  fn mul(self, rhs: Complex64) -> Ket {
    Ket {
      first: self.first * rhs,
      second: self.second * rhs,
    }
  }
}

#[test]
fn mul_ket_zero_with_one() {
  assert_eq!(KET_ZERO, KET_ZERO * COMPLEX_ONE);
}

#[test]
fn mul_ket_one_with_one() {
  assert_eq!(KET_ONE, KET_ONE * COMPLEX_ONE);
}

// Let's also do this the other way around
impl Mul<Ket> for Complex64 {
  type Output = Ket;

  fn mul(self, rhs: Ket) -> Ket {
    Ket {
      first: self * rhs.first,
      second: self * rhs.second,
    }
  }
}

#[test]
fn mul_one_with_ket_zero() {
  assert_eq!(KET_ZERO * COMPLEX_ONE, KET_ZERO);
}

#[test]
fn mul_one_with_ket_one() {
  assert_eq!(KET_ONE * COMPLEX_ONE, KET_ONE);
}

// Now we can already do pretty nice arithmetic on Kets!
#[test]
fn ket_arithmetic() {
  let a = Complex64::from(0.6) * KET_ZERO;
  let b = Complex64::from(0.8) * KET_ONE;
  let c = a + b;
  assert_eq!(
    c,
    Ket {
      first: Complex64::from(0.6),
      second: Complex64::from(0.8),
    }
  )
}

// Quantum states actually have an additional validity constraints
pub trait ValidQuantumState {
  fn is_valid(&self) -> bool;
}

// The sums of the squares of the amplitudes must be equal to 1
// Amplitude of a complex number x is |x|, available as .norm()
// in the Complex64 type
impl ValidQuantumState for Ket {
  fn is_valid(&self) -> bool {
    let a = self.first.norm();
    let b = self.second.norm();
    let result = (a * a) + (b * b);
    approx_eq!(f64, result, 1.0, ulps = 2)
  }
}

#[test]
fn ket_zero_valid() {
  assert_eq!(KET_ZERO.is_valid(), true);
}

#[test]
fn ket_one_valid() {
  assert_eq!(KET_ONE.is_valid(), true);
}

#[test]
fn ket_invalid() {
  assert_eq!((KET_ONE + KET_ZERO).is_valid(), false);
}

#[test]
fn ket_arithmetic_valid() {
  let a = Complex64 { re: 0.5, im: 0.5 };
  let b = Complex64 {
    re: 0.0_f64,
    im: 1.0 / 2.0_f64.sqrt(),
  };
  let a = a * KET_ZERO;
  let b = b * KET_ONE;
  let c = a + b;
  assert_eq!(c.is_valid(), true)
}
