/*
use divan::Bencher;
use packer::*;
use std::time::Duration;

use rand::{thread_rng, RngCore};

const N: u64 = 100_000;

fn main() {
    divan::main();
}

const IRANGE: i64 = 1000;
const URANGE: u64 = 1000;

fn rand_u64() -> u64 {
    thread_rng().next_u64() % URANGE
}
fn rand_i64() -> i64 {
    rand_u64() as i64 - IRANGE / 2
}
fn rand_bool() -> bool {
    rand_u64() % 2 == 0
}
fn rand_usize() -> usize {
    thread_rng().next_u64() as usize
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn insert_raw(bencher: Bencher) {
    let mut col: ColumnData<RawCursor> = (0..N).map(|_| vec![0, 1, 2, 3, 4]).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % (col.len() / 5) * 5;
        let value = vec![0, 1, 2, 3, 4];
        col.splice(pos, 0, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn insert_uint(bencher: Bencher) {
    let mut col: ColumnData<UIntCursor> = (0..N).map(|_| rand_u64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_u64();
        col.splice(pos, 0, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn insert_int(bencher: Bencher) {
    let mut col: ColumnData<IntCursor> = (0..N).map(|_| rand_i64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_i64();
        col.splice(pos, 0, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn insert_delta(bencher: Bencher) {
    let mut col: ColumnData<DeltaCursor> = (0..N).map(|_| rand_i64().abs()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_i64().abs();
        col.splice(pos, 0, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn insert_bool(bencher: Bencher) {
    let mut col: ColumnData<BooleanCursor> = (0..N).map(|_| rand_bool()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_bool();
        col.splice(pos, 0, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn seek_bool(bencher: Bencher) {
    let col: ColumnData<BooleanCursor> = (0..N).map(|_| rand_bool()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        col.get(pos);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn seek_int(bencher: Bencher) {
    let col: ColumnData<IntCursor> = (0..N).map(|_| rand_i64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        col.get(pos);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn seek_unt(bencher: Bencher) {
    let col: ColumnData<UIntCursor> = (0..N).map(|_| rand_u64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        col.get(pos);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn seek_delta(bencher: Bencher) {
    let col: ColumnData<DeltaCursor> = (0..N).map(|_| rand_i64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        col.get(pos);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn seek_raw(bencher: Bencher) {
    let col: ColumnData<RawCursor> = (0..N).map(|_| vec![0, 1, 2, 3, 4]).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % (col.len() / 5) * 5;
        col.get(pos);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_uint(bencher: Bencher) {
    let mut col: ColumnData<UIntCursor> = (0..N).map(|_| rand_u64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_u64();
        col.splice(pos, 1, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_int(bencher: Bencher) {
    let mut col: ColumnData<IntCursor> = (0..N).map(|_| rand_i64()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_i64();
        col.splice(pos, 1, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_delta(bencher: Bencher) {
    let mut col: ColumnData<DeltaCursor> = (0..N).map(|_| rand_i64().abs()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_i64().abs();
        col.splice(pos, 1, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_bool(bencher: Bencher) {
    let mut col: ColumnData<BooleanCursor> = (0..N).map(|_| rand_bool()).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % col.len();
        let value = rand_bool();
        col.splice(pos, 1, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_raw(bencher: Bencher) {
    let mut col: ColumnData<RawCursor> = (0..N).map(|_| vec![0, 1, 2, 3, 4]).collect();
    bencher.bench_local(|| {
        let pos = rand_usize() % (col.len() / 5) * 5;
        let value = vec![0, 1, 2, 3, 4];
        col.splice(pos, 5, [value]);
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_uint(bencher: Bencher) {
    let col: ColumnData<UIntCursor> = (0..N).map(|_| rand_u64()).collect();
    let data = col.save();
    bencher.bench_local(|| {
        UIntCursor::load(&data).unwrap();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_int(bencher: Bencher) {
    let col: ColumnData<IntCursor> = (0..N).map(|_| rand_i64()).collect();
    let data = col.save();
    bencher.bench_local(|| {
        IntCursor::load(&data).unwrap();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_delta(bencher: Bencher) {
    let col: ColumnData<DeltaCursor> = (0..N).map(|_| rand_i64().abs()).collect();
    let data = col.save();
    bencher.bench_local(|| {
        DeltaCursor::load(&data).unwrap();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_bool(bencher: Bencher) {
    let col: ColumnData<BooleanCursor> = (0..N).map(|_| rand_bool()).collect();
    let data = col.save();
    bencher.bench_local(|| {
        BooleanCursor::load(&data).unwrap();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn load_raw(bencher: Bencher) {
    let col: ColumnData<RawCursor> = (0..N).map(|_| vec![0, 1, 2, 3, 4]).collect();
    let data = col.save();
    bencher.bench_local(|| {
        RawCursor::load(&data).unwrap();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_uint(bencher: Bencher) {
    let col: ColumnData<UIntCursor> = (0..N).map(|_| rand_u64()).collect();
    bencher.bench_local(|| {
        col.save();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_int(bencher: Bencher) {
    let col: ColumnData<IntCursor> = (0..N).map(|_| rand_i64()).collect();
    bencher.bench_local(|| {
        col.save();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_delta(bencher: Bencher) {
    let col: ColumnData<DeltaCursor> = (0..N).map(|_| rand_i64().abs()).collect();
    bencher.bench_local(|| {
        col.save();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_bool(bencher: Bencher) {
    let col: ColumnData<BooleanCursor> = (0..N).map(|_| rand_bool()).collect();
    bencher.bench_local(|| {
        col.save();
    });
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn save_raw(bencher: Bencher) {
    let col: ColumnData<RawCursor> = (0..N).map(|_| vec![0, 1, 2, 3, 4]).collect();
    bencher.bench_local(|| {
        col.save();
    });
}
*/
