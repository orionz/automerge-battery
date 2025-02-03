use automerge::Automerge;
use divan::Bencher;
use std::time::Duration;

//use divan::{AllocProfiler};
//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(args=["./benches/embark.automerge","./benches/moby-dick.automerge"], max_time = Duration::from_secs(3))]
fn load(bencher: Bencher, filename: &str) {
    let data = std::fs::read(filename).unwrap();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

#[divan::bench(args=["./benches/embark.automerge","./benches/moby-dick.automerge"], max_time = Duration::from_secs(3))]
fn save(bencher: Bencher, filename: &str) {
    let data = std::fs::read(filename).unwrap();
    let doc = Automerge::load(data.as_slice()).unwrap();
    bencher.bench_local(|| {
        doc.save();
    });
}
