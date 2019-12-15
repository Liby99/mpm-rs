use std::fs::File;
use std::io::Write;
use specs::prelude::*;

use crate::resources::*;
use crate::components::*;

pub struct DumpSystem {
  dump_count: usize,
}

impl Default for DumpSystem {
  fn default() -> Self {
    Self { dump_count: 0 }
  }
}

impl<'a> System<'a> for DumpSystem {
  type SystemData = (
    Read<'a, OutputDirectory>,
    Read<'a, StepCount>,
    Read<'a, DumpSkip>,
    ReadStorage<'a, ParticlePosition>,
  );

  fn run(&mut self, (out_dir, step_count, dump_skip, positions): Self::SystemData) {
    if dump_skip.need_dump(step_count.get()) {
      self.dump_count += 1;
      let filename = format!("{}/{}.poly", out_dir.get(), self.dump_count);
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
}