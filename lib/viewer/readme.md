# MPM Viewer

## Usage

``` rust
use mpm_viewer::*;

fn main() {
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(WindowSystem::new())
    .build();

  while world.not_ending() {
    world.step();
  }
}
```

## API

When including `mpm_viewer::*`, you can have access to two new functions in
`world`:

- `not_ending() -> bool`: Check if the world is *not* ending
- `is_ending() -> bool`: Check if the world *is* ending

The world will become "ending" when the user close the window or quit the program.