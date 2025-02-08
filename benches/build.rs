use automerge_battery::{
    big_paste_doc, big_random_chunky_doc, maps_in_maps_doc,
    deep_history_doc, poorly_simulated_typing_doc,
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
fn build_big_test(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(big_paste_doc(N));
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn build_big_random_chunky_doc(bencher: Bencher) {
    bencher.bench_local(|| {
        black_box(big_random_chunky_doc(N));
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
