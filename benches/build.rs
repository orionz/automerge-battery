use automerge_battery::{
    big_paste_doc, deep_history_doc, maps_in_maps_doc, poorly_simulated_typing_doc, text_splice_100,
};
use divan::{black_box, Bencher};
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_typing(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(poorly_simulated_typing_doc(N));
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_big_paste(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(big_paste_doc(N));
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_text_splice_100(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(text_splice_100(N));
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_maps_in_maps_doc(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(maps_in_maps_doc(N));
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_deep_history_doc(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(deep_history_doc(N));
    })
}
