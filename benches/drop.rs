use automerge_battery::{
    big_paste_doc, big_random_chunky_doc, 
    poorly_simulated_typing_doc,
};
use divan::{black_box, Bencher};
use std::time::Duration;

use divan::AllocProfiler;
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const N: u64 = 10_000;

fn main() {
    divan::main();
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn drop_typing(bencher: Bencher) {
    let data = poorly_simulated_typing_doc(N);
    bencher.bench_local(|| {
        let d = data.clone();
        std::mem::drop(black_box(d))
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn drop_big_test(bencher: Bencher) {
    let data = big_paste_doc(N);
    bencher.bench_local(|| {
        let d = data.clone();
        std::mem::drop(black_box(d))
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn drop_big_random_chunky_doc(bencher: Bencher) {
    let data = big_random_chunky_doc(N);
    bencher.bench_local(|| {
        let d = data.clone();
        // println!("{:?}", d.stats());
        std::mem::drop(black_box(d))
    })
}
