use automerge::{Automerge};
use automerge_battery::{big_paste_doc, poorly_simulated_typing_doc, text_splice_100};
use divan::{black_box, Bencher};

use trace_alloc::measure_memusage;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 1)]
fn load_typing(bencher: Bencher) {
    let data = poorly_simulated_typing_doc(N).save();
    bencher.bench_local(|| {
        let (peak, steady, result) = measure_memusage(|| {
            Automerge::load(&data).unwrap()
        });
        black_box(result);
        print_mem(peak,steady);
    });
}

#[divan::bench(sample_count = 1)]
fn load_big_paste(bencher: Bencher) {
    let data = big_paste_doc(N).save();
    bencher.bench_local(|| {
        let (peak, steady, result) = measure_memusage(|| {
            Automerge::load(&data).unwrap()
        });
        black_box(result);
        print_mem(peak,steady);
    });
}

#[divan::bench(sample_count = 1)]
fn load_splice_100(bencher: Bencher) {
    let data = text_splice_100(N).save();
    bencher.bench_local(|| {
        let (peak, steady, result) = measure_memusage(|| {
            Automerge::load(&data).unwrap()
        });
        black_box(result);
        print_mem(peak,steady);
    });
}

fn print_value(label: &str, size: usize) {
  if size > 1024 * 1024 * 10 {
     let out = format!("{},{:0>3}", size / 1024 / 1000, size / 1024 % 1000);
     println!(":: {}: {: >8} KB", label, out);
  } else {
     println!(":: {}: {: >8} KB", label, size / 1024);
  }
}

fn print_mem(peak: usize, steady: usize) {
        println!("");
        print_value("peak", peak);
        print_value("stdy", steady);
}

