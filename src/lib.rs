// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Beam.rs is a library that defines the core Point types for various laser
//! projection libraries. This standardization makes it easier for library
//! interop and prevents expensive and unwieldy type conversion.

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

use std::fmt;

/// The highest value that can be specified for a single color channel.
/// Note: this may only be relevant for EtherDream.
pub const COLOR_MAX : u16 = 65535;

/// Core point type.
/// Supports position (x, y), color (r, g, b), and an is_blank flag.
#[derive(Clone, Copy, Debug)]
pub struct SimplePoint {
  /// X-coordinate.
  pub x: i16,
  /// Y-coordinate.
  pub y: i16,
  /// Red color value.
  pub r: u16,
  /// Green color value.
  pub g: u16,
  /// Blue color value.
  pub b: u16,
  /// Whether the point is semantically considered a "blanking" point.
  /// A blanking point may still encode color information, but we generally do
  /// not render these points unless we're debugging.
  pub is_blank: bool,
}

impl SimplePoint {
  /// SimplePoint CTOR.
  /// Lets you specify colors for each channel separately.
  pub fn xy_rgb(x: i16, y: i16, r: u16, g: u16, b: u16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: r,
      g: g,
      b: b,
      is_blank: false,
    }
  }

  /// SimplePoint CTOR.
  /// Crates a blanking point.
  /// The blanking boolean is set to true, meaning this is semantically
  /// considered to be used for blanking purposes.
  pub fn xy_blank(x: i16, y: i16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: 0,
      g: 0,
      b: 0,
      is_blank: true,
    }
  }

  /// SimplePoint CTOR.
  /// Uses the same intensity value for all color channels.
  pub fn xy_luma(x: i16, y: i16, luminance: u16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: luminance,
      g: luminance,
      b: luminance,
      is_blank: false,
    }
  }

  /// SimplePoint CTOR.
  /// Sets only the red color channel.
  pub fn xy_red(x: i16, y: i16, red: u16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: red,
      g: 0,
      b: 0,
      is_blank: false,
    }
  }

  /// SimplePoint CTOR.
  /// Sets only the green color channel.
  pub fn xy_green(x: i16, y: i16, green: u16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: 0,
      g: green,
      b: 0,
      is_blank: false,
    }
  }

  /// SimplePoint CTOR.
  /// Sets only the blue color channel.
  pub fn xy_blue(x: i16, y: i16, blue: u16) -> SimplePoint {
    SimplePoint {
      x: x,
      y: y,
      r: 0,
      g: 0,
      b: blue,
      is_blank: false,
    }
  }

  /// SimplePoint CTOR.
  /// If set to on, the lasers are at full power. Otherwise, they're off.
  /// An "off" point is *not* considered a blanking point.
  pub fn xy_binary(x: i16, y: i16, on: bool) -> SimplePoint {
    let c = if on { COLOR_MAX } else { 0 };
    SimplePoint::xy_rgb(x, y, c, c, c)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_xy_rgb() {
    let pt = SimplePoint::xy_rgb(100, -100, 1, 200, 300);
    assert_eq!(100, pt.x);
    assert_eq!(-100, pt.y);
    assert_eq!(1, pt.r);
    assert_eq!(200, pt.g);
    assert_eq!(300, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_xy_blank() {
    let pt = SimplePoint::xy_blank(-1000, -2000);
    assert_eq!(-1000, pt.x);
    assert_eq!(-2000, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(true, pt.is_blank);
  }

  #[test]
  fn test_xy_luma() {
    let pt = SimplePoint::xy_luma(10, 20, 255);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(255, pt.r);
    assert_eq!(255, pt.g);
    assert_eq!(255, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_xy_red() {
    let pt = SimplePoint::xy_red(10, 20, 1000);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(1000, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_xy_green() {
    let pt = SimplePoint::xy_green(10, 20, 255);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(255, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_xy_blue() {
    let pt = SimplePoint::xy_blue(10, 20, 2000);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(2000, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_xy_binary() {
    let pt = SimplePoint::xy_binary(-9000, 9001, false);
    assert_eq!(-9000, pt.x);
    assert_eq!(9001, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(false, pt.is_blank);

    let pt = SimplePoint::xy_binary(8000, -9001, true);
    assert_eq!(8000, pt.x);
    assert_eq!(-9001, pt.y);
    assert_eq!(65535, pt.r);
    assert_eq!(65535, pt.g);
    assert_eq!(65535, pt.b);
    assert_eq!(false, pt.is_blank);
  }
}

impl fmt::Display for SimplePoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
