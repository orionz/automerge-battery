use automerge::Automerge;
use divan::Bencher;
use std::time::Duration;

//use divan::{AllocProfiler};
//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

const FILES : [&str;3] = [
  "./benches/embark.automerge",
  "./benches/moby-dick.automerge",
  "./benches/monday-meeting-notes.automerge",
  //"./benches/webstraits.amrg",
  //"./benches/stephen.amrg",
];
const MAX : Duration = Duration::from_secs(3);

#[divan::bench(args=FILES, max_time=MAX)]
fn load(bencher: Bencher, filename: &str) {
    let data = std::fs::read(filename).unwrap();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

#[divan::bench(args=FILES, max_time=MAX)]
fn save(bencher: Bencher, filename: &str) {
    let data = std::fs::read(filename).unwrap();
    let doc = Automerge::load(data.as_slice()).unwrap();
    bencher.bench_local(|| {
        doc.save();
    });
}
