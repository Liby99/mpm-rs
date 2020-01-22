# MPM PLY Dump System

## Usage

``` rust
let outdir = "result/bunny";
let dump_skip = 10;

std::fs::create_dir_all(outdir).unwrap();

let mut world = WorldBuilder::new(world_size, grid_h)
  .with_system(PlyDumpSystem::new(outdir, dump_skip))
  .build();

for _ in 0..100 {
  world.step();
}
```

## Parameters

- `outdir`: The output directory. The system doesn't create it automatically. If you want to export to an unexisting
  directory, please create that directory automatically.
- `dump_skip`: Output a file every `dump_skip`. In the above example, we have dump_skip `10`. So we will output a
  file at frame `0`, `9`, `19`, ..., `99` (10 in total). Each file will still be numbered incrementally starting
  from `1`.