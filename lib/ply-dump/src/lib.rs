extern crate specs;

use std::fs::File;
use std::io::Write;
use specs::prelude::*;

use crate::resources::*;
use crate::components::*;

pub struct DumpSystem {
  out_dir: String,
  dump_count: usize,
  dump_skip: usize,
}

impl DumpSystem {
  pub fn new(out_dir: &str, dump_skip: usize) -> Self {
    Self {
      out_dir: String::from(out_dir),
      dump_count: 0,
      dump_skip,
    }
  }
}

impl<'a> System<'a> for DumpSystem {
  type SystemData = (
    Read<'a, StepCount>,
    ReadStorage<'a, ParticlePosition>,
    ReadStorage<'a, Hidden>,
  );

  fn run(&mut self, (step_count, positions, hiddens): Self::SystemData) {
    if step_count.get() % self.dump_skip == 0 {
      self.dump_count += 1;
      let filename = format!("{}/{}.poly", self.out_dir, self.dump_count);
      let mut file = File::create(filename).unwrap();
      file.write(b"POINTS\n").unwrap();
      for (i, (pos, _)) in (&positions, !&hiddens).join().enumerate() {
        let p = pos.get();
        let line = format!("{}: {} {} {}\n", i + 1, p.x, p.y, p.z);
        file.write(line.as_bytes()).unwrap();
      }
      file.write(b"POLYS\nEND\n").unwrap();
    }
  }
}