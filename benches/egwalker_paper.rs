use automerge::{AutoCommit, ReadDoc, ROOT};
use divan::Bencher;
use std::time::Duration;

//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

//const FILES: [&str; 2] = [ "C1.am", "C2.am" ];
const FILES: [&str; 7] = [
    "S1.am", "S3.am", "S3.am", "A1.am", "A2.am", "C1.am", "C2.am",
];

#[divan::bench(args=FILES, max_time = Duration::from_secs(3))]
fn load(bencher: Bencher, filename: &str) {
    let bytes = std::fs::read(format!("./egwalker-paper/{}", filename)).unwrap();
    bencher.bench_local(|| {
        let _doc = AutoCommit::load(&bytes).unwrap();
    });
}

#[divan::bench(args=FILES, max_time = Duration::from_secs(3))]
fn get_text(bencher: Bencher, filename: &str) {
    let bytes = std::fs::read(format!("./egwalker-paper/{}", filename)).unwrap();
    let doc = AutoCommit::load(&bytes).unwrap();
    //println!("{:?}",doc.stats());
    bencher.bench_local(|| {
        let (_, text_id) = doc.get(ROOT, "text").unwrap().unwrap();
        let _result = doc.text(text_id).unwrap();
    });
}
