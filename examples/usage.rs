//! Usage examples aren't really necessary.
//! This just tests that the exports are as expected.

extern crate point;

use point::SimplePoint;
use point::PipelinePoint;

fn main() {
  let pt = SimplePoint::xy_rgb(0, -100, 100, 200, 255);
  println!("Simple point: {}", pt);
  println!("Simple point: {:?}", pt);

  println!("Min color: {}, Max color: {}",
    SimplePoint::MIN_COLOR,
    SimplePoint::MAX_COLOR);

  let pt = PipelinePoint::xy_rgb(0.5, 0.5, 10.0, 20.5, 255.0);
  println!("Pipeline point: {}", pt);
  println!("Pipeline point: {:?}", pt);

  println!("Min color: {}, Max color: {}",
      PipelinePoint::MIN_COLOR,
      PipelinePoint::MAX_COLOR);
}

