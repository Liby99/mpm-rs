use pbr::ProgressBar;

use super::mpm::*;

pub struct Driver {
  pub world: World,
  pub dt: f32,
}

impl Driver {
  pub fn new(world: World, dt: f32) -> Self {
    Self { world, dt }
  }

  pub fn run(&mut self, outdir: String, num_steps: usize) -> std::io::Result<()> {
    // First dump the default frame
    let filename = format!("{}/{}.poly", outdir, 0);
    self.world.dump(filename)?;

    // Then
    let mut pb = ProgressBar::new(num_steps as u64);
    for frame in 0..num_steps {
      pb.inc();

      // First step the world forward
      self.world.step(self.dt);

      // Then get the filename and dump the particles
      let filename = format!("{}/{}.poly", outdir, frame + 1);
      self.world.dump(filename)?;
    }
    pb.finish_print("Done");
    Ok(())
  }
}
