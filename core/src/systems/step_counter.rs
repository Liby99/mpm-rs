use specs::prelude::*;

use crate::resources::*;

pub struct StepCounterSystem;

impl<'a> System<'a> for StepCounterSystem {
  type SystemData = Write<'a, StepCount>;

  fn run(&mut self, mut step_count: Self::SystemData) {
    step_count.step();
  }
}
