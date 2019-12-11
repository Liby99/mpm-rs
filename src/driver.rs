use std::fs::File;
use std::io::Write;
use pbr::ProgressBar;

use super::mpm::*;

fn get_filename(outdir: &String, frame: usize, num_digits: usize) -> String {
  format!("{}/{:0width$}.poly", outdir, frame, width = num_digits)
}

pub struct Driver {
  pub world: World,
  pub dt: f32,
}

impl Driver {
  pub fn new(world: World, dt: f32) -> Self {
    Self { world, dt }
  }

  pub fn run(&mut self, outdir: String, num_steps: usize) -> std::io::Result<()> {
    let mut pb = ProgressBar::new(num_steps as u64);
    let num_digits = (num_steps as f32).log(10.0).ceil() as usize;
    for frame in 0..num_steps {
      pb.inc();

      // First step the world forward
      self.world.step(self.dt);

      // Then get the filename and dump the particles
      let filename = get_filename(&outdir, frame, num_digits);
      let mut file = File::create(filename)?;
      file.write(b"POINTS\n")?;
      for (i, par) in self.world.particles.iter().enumerate() {
        let pos = par.position;
        let line = format!("{} : {} {} {}\n", i + 1, pos.x, pos.y, pos.z);
        file.write(line.as_bytes())?;
      }
      file.write(b"END\n")?;
    }
    pb.finish_print("Done");
    Ok(())
  }
}