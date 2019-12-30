use specs::prelude::*;

#[derive(Default)]
pub struct Hidden;

impl Component for Hidden {
  type Storage = NullStorage<Self>;
}