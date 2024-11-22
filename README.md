
## A battery of benchmarks to run against different versions of Automerge

  This repo should be checked out side by side to an automerge checkout (eg automerge is at `../automerge`)

  Then run...

  `cargo bench`

## Getting a flamegraph

### Setup

  `cargo install flamegraph`

### On MacOS

  `cargo flamegraph --root --bench edit_trace`

