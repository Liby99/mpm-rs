use std::fs::File;
use std::io::Write;
use specs::prelude::*;

use crate::resources::*;
use crate::components::*;

pub struct DumpSystem;

impl<'a> System<'a> for DumpSystem {
  type SystemData = (
    Read<'a, OutputDirectory>,
    Read<'a, StepCount>,
    ReadStorage<'a, ParticlePosition>,
  );

  fn run(&mut self, (out_dir, step_count, positions): Self::SystemData) {
    let filename = format!("{}/{}.poly", out_dir.get(), step_count.get());
    let mut file = File::create(filename).unwrap();
    file.write(b"POINTS\n").unwrap();
    for (i, pos) in (&positions).join().enumerate() {
      let p = pos.get();
      let line = format!("{}: {} {} {}\n", i + 1, p.x, p.y, p.z);
      file.write(line.as_bytes()).unwrap();
    }
    file.write(b"POLYS\nEND\n").unwrap();
  }
}