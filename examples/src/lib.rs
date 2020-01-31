use clap::*;
use mpm_ply_dump::*;
use mpm_rs::*;
use mpm_viewer::*;
use pbr::ProgressBar;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Config<'a> {
  pub world_size: Vector3f,
  pub world_dx: f32,
  pub world_dt: f32,
  pub output_directory: &'a str,
  pub num_cycles: u64,
  pub dump_skip: usize,
}

impl<'a> Default for Config<'a> {
  fn default() -> Self {
    Self {
      world_size: Vector3f::new(1.0, 1.0, 1.0),
      world_dx: 0.02,
      world_dt: 0.01,
      output_directory: "result",
      num_cycles: 500,
      dump_skip: 10,
    }
  }
}

pub fn run_example<'a, 'b, F>(config: Config, init: F)
where
  F: Fn(&mut World<'a, 'b>),
{
  // Get the arguments
  let view_arg = Arg::with_name("view").short("v").long("view").help("Open viewer");
  let debug_arg = Arg::with_name("debug")
    .short("d")
    .long("debug")
    .help("Enable debug prints");
  let time_arg = Arg::with_name("time").short("t").long("time").help("Time computation");
  let matches = App::new("MPM Example")
    .arg(view_arg)
    .arg(debug_arg)
    .arg(time_arg)
    .get_matches();

  // Get basic world builder
  let world_builder = WorldBuilder::new()
    .with_size(config.world_size)
    .with_dx(config.world_dx)
    .with_dt(config.world_dt);

  // Build the world
  let mut world = (if matches.is_present("view") {
    if matches.is_present("debug") {
      println!("[DEBUG] Enabling viewer");
    }
    world_builder.with_system(WindowSystem::new())
  } else {
    if matches.is_present("debug") {
      println!(
        "[DEBUG] Enabling dumping result. Output to '{}'",
        config.output_directory
      );
      println!("[DEBUG] Setting dump skip {}", config.dump_skip);
    }
    let dump_sys = PlyDumpSystem::new(config.output_directory, config.dump_skip);
    world_builder.with_system(dump_sys)
  })
  .build();

  if matches.is_present("debug") {
    println!("[DEBUG] Initializing world...");
  }

  // Invoke custom initialize function
  init(&mut world);

  // Print the number of particles if debug enabled
  if matches.is_present("debug") {
    println!("[DEBUG] Done initializing world");
    println!("[DEBUG] Number of particles: {}", world.num_particles());
  }

  // Run the world
  if matches.is_present("view") {
    if matches.is_present("debug") {
      println!("[DEBUG] Starting viewer...");
    }
    while world.not_ending() {
      world.step();
    }
  } else {
    let start = SystemTime::now();
    if matches.is_present("debug") {
      println!("[DEBUG] Creating output directory '{}'", config.output_directory);
    }
    std::fs::create_dir_all(config.output_directory).unwrap();
    let mut pb = ProgressBar::new(config.num_cycles);
    for _ in 0..config.num_cycles {
      pb.inc();
      world.step();
    }
    let finish = if matches.is_present("time") {
      let secs_elapsed = start.elapsed().unwrap().as_secs();
      format!("Finished {} cycles in {} secs", config.num_cycles, secs_elapsed)
    } else {
      format!("Finished {} cycles", config.num_cycles)
    };
    pb.finish_print(finish.as_str());
  }
}
