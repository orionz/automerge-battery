use automerge::Automerge;
use automerge_battery::{big_paste_doc, poorly_simulated_typing_doc, text_splice_100};
use divan::{black_box, Bencher};
use std::time::Duration;

use divan::AllocProfiler;
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const N: u64 = 100_000;

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
fn drop_text_splice_100(bencher: Bencher) {
    let data = text_splice_100(N);
    bencher.bench_local(|| {
        let d = data.clone();
        std::mem::drop(black_box(d))
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_text_splice_100(bencher: Bencher) {
    let data = text_splice_100(N).save();
    bencher.bench_local(|| Automerge::load(&data).unwrap())
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_typing(bencher: Bencher) {
    let data = poorly_simulated_typing_doc(N).save();
    bencher.bench_local(|| Automerge::load(&data).unwrap())
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_big_test(bencher: Bencher) {
    let data = big_paste_doc(N).save();
    bencher.bench_local(|| Automerge::load(&data).unwrap())
}
