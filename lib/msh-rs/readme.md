# Rust .msh File Loader

Reference program: https://github.com/Yixin-Hu/TetWild/blob/master/extern/pymesh/MshLoader.cpp

## Usage

``` rust
match TetrahedronMesh::load("res/bunny.msh") {
  Ok(mesh) => {
    // Do something with the mesh
  },
  Err(err) => {
    // Handle error
  }
}
```