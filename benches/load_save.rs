use automerge::Automerge;
use automerge_battery::{
    big_paste_doc, big_random_chunky_doc, big_random_doc, maps_in_maps_doc,
    poorly_simulated_typing_doc,
};
use divan::Bencher;
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_big_paste(bencher: Bencher) {
    let doc = big_paste_doc(N);
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_maps_in_maps(bencher: Bencher) {
    let doc = maps_in_maps_doc(N);
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}

/*
#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_deep_history(bencher: Bencher) {
    let doc = deep_history_doc(N);
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}
*/

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_typing(bencher: Bencher) {
    let doc = poorly_simulated_typing_doc(N);
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_big_paste(bencher: Bencher) {
    let data = big_paste_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_chunky(bencher: Bencher) {
    let data = big_random_chunky_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_chunky(bencher: Bencher) {
    let doc = big_random_chunky_doc(N);
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_big_random_test(bencher: Bencher) {
    let data = big_random_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_maps_in_maps(bencher: Bencher) {
    let data = maps_in_maps_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}

/*
#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_deep_history(bencher: Bencher) {
    let data = deep_history_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}
*/

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_typing(bencher: Bencher) {
    let data = poorly_simulated_typing_doc(N).save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}
