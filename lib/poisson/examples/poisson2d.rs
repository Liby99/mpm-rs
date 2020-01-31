extern crate clap;
extern crate nalgebra as na;

use clap::*;
use na::*;
use poisson::*;
use std::fs::File;
use std::io::prelude::*;

type Vector2f = Vector2<f32>;

fn main() {
  let cmd_args = App::new("poisson2d")
    .version("0.1")
    .author("Ziyang L. <liby99@icloud.com>")
    .about("Output 2D poisson samples in JSON format")
    .arg(
      Arg::with_name("width")
        .long("width")
        .short("w")
        .value_name("WIDTH")
        .default_value("1.0")
        .help("Set the width of the space to sample"),
    )
    .arg(
      Arg::with_name("height")
        .long("height")
        .short("h")
        .value_name("HEIGHT")
        .default_value("1.0")
        .help("Set the height of the space to sample"),
    )
    .arg(
      Arg::with_name("radius")
        .long("radius")
        .short("r")
        .value_name("RADIUS")
        .default_value("0.05")
        .help("Set the radius of the poisson disk"),
    )
    .arg(
      Arg::with_name("output")
        .long("output")
        .short("o")
        .value_name("FILE")
        .help("Output file name"),
    )
    .get_matches();

  // Get the command line arguments
  let width: f32 = cmd_args
    .value_of("width")
    .map(|s| s.parse().expect("Invalid Width"))
    .unwrap_or(1.0);
  let height: f32 = cmd_args
    .value_of("height")
    .map(|s| s.parse().expect("Invalid Height"))
    .unwrap_or(1.0);
  let radius: f32 = cmd_args
    .value_of("radius")
    .map(|s| s.parse().expect("Invalid Radius"))
    .unwrap_or(0.05);
  let sampler = Sampler2f::new()
    .with_size(Vector2f::new(width, height))
    .with_radius(radius);

  // Start output
  if let Some(filename) = cmd_args.value_of("output") {
    // Output file
    let mut is_first_row = true;
    let mut file = File::create(filename).expect(format!("Cannot open file {}", filename).as_str());
    file.write_all(b"[\n").expect("Cannot write to file");
    for sample in sampler.generate() {
      if !is_first_row {
        file.write_all(b",\n").expect("Cannot write to file");
      } else {
        is_first_row = false;
      }
      file
        .write_fmt(format_args!("  {{ \"x\": {}, \"y\": {} }}", sample.x, sample.y))
        .expect("Cannot write to file");
    }
    file.write_all(b"\n]\n").expect("Cannot write to file");
  } else {
    // Output
    let mut is_first_row = true;
    println!("[");
    for sample in sampler.generate() {
      if !is_first_row {
        println!(",");
      } else {
        is_first_row = false;
      }
      print!("  {{ \"x\": {}, \"y\": {} }}", sample.x, sample.y);
    }
    println!("\n]");
  }
}
