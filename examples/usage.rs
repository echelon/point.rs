//! Usage examples aren't really necessary.
//! This just tests that the exports are as expected.

extern crate point;

use point::SimplePoint;

fn main() {
  let pt = SimplePoint::xy_rgb(0, 0, 100, 200, 300);
  println!("Point: {}", pt);
  println!("Point: {:?}", pt);
}

