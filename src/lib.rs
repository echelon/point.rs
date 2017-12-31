// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Point.rs is a library that defines the core Point types for various laser
//! projection libraries. This standardization makes it easier for library
//! interop and prevents expensive and unwieldy type conversion.

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

use std::fmt;

/// Core point type.
/// Supports position (x, y), color (r, g, b), and an is_blank flag.
#[derive(Clone, Copy, Debug, Default)]
pub struct SimplePoint {
  /// X-coordinate.
  pub x: i16,
  /// Y-coordinate.
  pub y: i16,
  /// Red color value.
  pub r: u8,
  /// Green color value.
  pub g: u8,
  /// Blue color value.
  pub b: u8,
  /// Whether the point is semantically considered a "blanking" point.
  /// A blanking point may still encode color information, but we generally do
  /// not render these points unless we're debugging.
  pub is_blank: bool,
}

/// Working point type. Do math calculations on this point type.
/// Supports position (x, y), color (r, g, b), and an is_blank flag.
#[derive(Clone, Copy, Debug, Default)]
pub struct PipelinePoint {
  /// X-coordinate.
  pub x: f32,
  /// Y-coordinate.
  pub y: f32,
  /// Red color value.
  pub r: f32,
  /// Green color value.
  pub g: f32,
  /// Blue color value.
  pub b: f32,
  /// Whether the point is semantically considered a "blanking" point.
  /// A blanking point may still encode color information, but we generally do
  /// not render these points unless we're debugging.
  pub is_blank: bool,
}

impl SimplePoint {
  /// Minimum value of the color channels.
  pub const MIN_COLOR : u8 = 0;

  /// Maximum value of the color channels.
  pub const MAX_COLOR : u8 = 255;

  /// SimplePoint CTOR.
  /// Lets you specify colors for each channel separately.
  pub fn xy_rgb(x: i16, y: i16, r: u8, g: u8, b: u8) -> SimplePoint {
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

  /// Transform a SimplePoint into a PipelinePoint for math operations.
  pub fn into_pipeline_pt(&self) -> PipelinePoint {
    PipelinePoint {
      x: self.x as f32,
      y: self.y as f32,
      r: self.r as f32,
      g: self.g as f32,
      b: self.b as f32,
      is_blank: self.is_blank,
    }
  }

  /// SimplePoint CTOR.
  /// Uses the same intensity value for all color channels.
  pub fn xy_luma(x: i16, y: i16, luminance: u8) -> SimplePoint {
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
  pub fn xy_red(x: i16, y: i16, red: u8) -> SimplePoint {
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
  pub fn xy_green(x: i16, y: i16, green: u8) -> SimplePoint {
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
  pub fn xy_blue(x: i16, y: i16, blue: u8) -> SimplePoint {
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
    let c = if on { Self::MAX_COLOR } else { 0 };
    SimplePoint::xy_rgb(x, y, c, c, c)
  }
}

impl PipelinePoint {
  /// Minimum value of the color channels.
  pub const MIN_COLOR : f32 = 0.0;

  /// Maximum value of the color channels.
  pub const MAX_COLOR : f32 = 255.0;

  /// PipelinePoint CTOR.
  /// Lets you specify colors for each channel separately.
  pub fn xy_rgb(x: f32, y: f32, r: f32, g: f32, b: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: r,
      g: g,
      b: b,
      is_blank: false,
    }
  }

  /// PipelinePoint CTOR.
  /// Crates a blanking point.
  /// The blanking boolean is set to true, meaning this is semantically
  /// considered to be used for blanking purposes.
  pub fn xy_blank(x: f32, y: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: 0.0,
      g: 0.0,
      b: 0.0,
      is_blank: true,
    }
  }

  /// Transform a PipelinePoint into a SimplePoint for sending to the DAC.
  pub fn into_simple_pt(&self) -> SimplePoint {
    SimplePoint {
      x: self.x as i16,
      y: self.y as i16,
      r: self.r as u8,
      g: self.g as u8,
      b: self.b as u8,
      is_blank: self.is_blank,
    }
  }

  /// PipelinePoint CTOR.
  /// Uses the same intensity value for all color channels.
  pub fn xy_luma(x: f32, y: f32, luminance: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: luminance,
      g: luminance,
      b: luminance,
      is_blank: false,
    }
  }

  /// PipelinePoint CTOR.
  /// Sets only the red color channel.
  pub fn xy_red(x: f32, y: f32, red: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: red,
      g: 0.0,
      b: 0.0,
      is_blank: false,
    }
  }

  /// PipelinePoint CTOR.
  /// Sets only the green color channel.
  pub fn xy_green(x: f32, y: f32, green: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: 0.0,
      g: green,
      b: 0.0,
      is_blank: false,
    }
  }

  /// PipelinePoint CTOR.
  /// Sets only the blue color channel.
  pub fn xy_blue(x: f32, y: f32, blue: f32) -> PipelinePoint {
    PipelinePoint {
      x: x,
      y: y,
      r: 0.0,
      g: 0.0,
      b: blue,
      is_blank: false,
    }
  }

  /// PipelinePoint CTOR.
  /// If set to on, the lasers are at full power. Otherwise, they're off.
  /// An "off" point is *not* considered a blanking point.
  pub fn xy_binary(x: f32, y: f32, on: bool) -> PipelinePoint {
    let c = if on { Self::MAX_COLOR } else { 0.0 };
    PipelinePoint::xy_rgb(x, y, c, c, c)
  }
}

impl fmt::Display for SimplePoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl fmt::Display for PipelinePoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simplepoint_xy_rgb() {
    let pt = SimplePoint::xy_rgb(100, -100, 1, 200, 123);
    assert_eq!(100, pt.x);
    assert_eq!(-100, pt.y);
    assert_eq!(1, pt.r);
    assert_eq!(200, pt.g);
    assert_eq!(123, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_blank() {
    let pt = SimplePoint::xy_blank(-1000, -2000);
    assert_eq!(-1000, pt.x);
    assert_eq!(-2000, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(true, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_into_pipeline_pt() {
    let sp = SimplePoint::xy_rgb(100, -100, 1, 200, 234);
    let pp = sp.into_pipeline_pt();
    assert_eq!(100.0, pp.x);
    assert_eq!(-100.0, pp.y);
    assert_eq!(1.0, pp.r);
    assert_eq!(200.0, pp.g);
    assert_eq!(234.0, pp.b);
    assert_eq!(false, pp.is_blank);

    let sp = SimplePoint::xy_blank(-9_001, 10_000);
    let pp = sp.into_pipeline_pt();
    assert_eq!(-9001.0, pp.x);
    assert_eq!(10_000.0, pp.y);
    assert_eq!(0.0, pp.r);
    assert_eq!(0.0, pp.g);
    assert_eq!(0.0, pp.b);
    assert_eq!(true, pp.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_luma() {
    let pt = SimplePoint::xy_luma(10, 20, 255);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(255, pt.r);
    assert_eq!(255, pt.g);
    assert_eq!(255, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_red() {
    let pt = SimplePoint::xy_red(10, 20, 100);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(100, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_green() {
    let pt = SimplePoint::xy_green(10, 20, 255);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(255, pt.g);
    assert_eq!(0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_blue() {
    let pt = SimplePoint::xy_blue(10, 20, 200);
    assert_eq!(10, pt.x);
    assert_eq!(20, pt.y);
    assert_eq!(0, pt.r);
    assert_eq!(0, pt.g);
    assert_eq!(200, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_simplepoint_xy_binary() {
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
    assert_eq!(255, pt.r);
    assert_eq!(255, pt.g);
    assert_eq!(255, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_rgb() {
    let pt = PipelinePoint::xy_rgb(100.0, -100.0, 1.0, 200.0, 220.0);
    assert_eq!(100.0, pt.x);
    assert_eq!(-100.0, pt.y);
    assert_eq!(1.0, pt.r);
    assert_eq!(200.0, pt.g);
    assert_eq!(220.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_blank() {
    let pt = PipelinePoint::xy_blank(-1000.0, -2000.0);
    assert_eq!(-1000.0, pt.x);
    assert_eq!(-2000.0, pt.y);
    assert_eq!(0.0, pt.r);
    assert_eq!(0.0, pt.g);
    assert_eq!(0.0, pt.b);
    assert_eq!(true, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_into_simple_pt() {
    let pp = PipelinePoint::xy_rgb(100.0, -100.0, 1.0, 200.0, 240.0);
    let sp= pp.into_simple_pt();
    assert_eq!(100, sp.x);
    assert_eq!(-100, sp.y);
    assert_eq!(1, sp.r);
    assert_eq!(200, sp.g);
    assert_eq!(240, sp.b);
    assert_eq!(false, sp.is_blank);

    let pp = PipelinePoint::xy_blank(-9_001.0, 10_000.0);
    let sp= pp.into_simple_pt();
    assert_eq!(-9001, sp.x);
    assert_eq!(10_000, sp.y);
    assert_eq!(0, sp.r);
    assert_eq!(0, sp.g);
    assert_eq!(0, sp.b);
    assert_eq!(true, sp.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_luma() {
    let pt = PipelinePoint::xy_luma(10.0, 20.0, 255.0);
    assert_eq!(10.0, pt.x);
    assert_eq!(20.0, pt.y);
    assert_eq!(255.0, pt.r);
    assert_eq!(255.0, pt.g);
    assert_eq!(255.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_red() {
    let pt = PipelinePoint::xy_red(10.0, 20.0, 100.0);
    assert_eq!(10.0, pt.x);
    assert_eq!(20.0, pt.y);
    assert_eq!(100.0, pt.r);
    assert_eq!(0.0, pt.g);
    assert_eq!(0.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_green() {
    let pt = PipelinePoint::xy_green(10.0, 20.0, 255.0);
    assert_eq!(10.0, pt.x);
    assert_eq!(20.0, pt.y);
    assert_eq!(0.0, pt.r);
    assert_eq!(255.0, pt.g);
    assert_eq!(0.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_blue() {
    let pt = PipelinePoint::xy_blue(10.0, 20.0, 200.0);
    assert_eq!(10.0, pt.x);
    assert_eq!(20.0, pt.y);
    assert_eq!(0.0, pt.r);
    assert_eq!(0.0, pt.g);
    assert_eq!(200.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  #[test]
  fn test_pipelinepoint_xy_binary() {
    let pt = PipelinePoint::xy_binary(-9000.0, 9001.0, false);
    assert_eq!(-9000.0, pt.x);
    assert_eq!(9001.0, pt.y);
    assert_eq!(0.0, pt.r);
    assert_eq!(0.0, pt.g);
    assert_eq!(0.0, pt.b);
    assert_eq!(false, pt.is_blank);

    let pt = PipelinePoint::xy_binary(8000.0, -9001.0, true);
    assert_eq!(8000.0, pt.x);
    assert_eq!(-9001.0, pt.y);
    assert_eq!(255.0, pt.r);
    assert_eq!(255.0, pt.g);
    assert_eq!(255.0, pt.b);
    assert_eq!(false, pt.is_blank);
  }

  // This simply tests that f32 is enough for our needs.
  // It's not really necessary to retain these tests, but it's a good
  // demonstration.
  #[test]
  fn single_precision_is_sufficient_for_i16() {
    // I assume the compiler doesn't optimize this away.
    fn convert(n: i16) -> i16 { (n as f32) as i16 }

    assert_eq!(i16::max_value(), convert(i16::max_value()));

    for n in i16::min_value() .. i16::max_value() {
      assert_eq!(n, convert(n));
    }
  }
}
