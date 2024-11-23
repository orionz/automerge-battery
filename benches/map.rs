use automerge::{transaction::Transactable, Automerge, ScalarValue, ROOT};
use std::time::Duration;
//use divan::AllocProfiler;

//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(args=[100,1000,10_000], max_time = Duration::from_secs(3))]
fn repeated_increment(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    tx.put(ROOT, "counter", ScalarValue::counter(0)).unwrap();
    for _ in 0..n {
        tx.increment(ROOT, "counter", 1).unwrap();
    }
    tx.commit();
    doc
}

#[divan::bench(args=[100,1000,10_000], max_time = Duration::from_secs(3))]
fn repeated_put(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, "0", i).unwrap();
    }
    tx.commit();
    doc
}

#[divan::bench(args=[100,1000,10_000], max_time = Duration::from_secs(3))]
fn increasing_put(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, i.to_string(), i).unwrap();
    }
    tx.commit();
    doc
}

#[divan::bench(args=[100,1000,10_000], max_time = Duration::from_secs(3))]
fn decreasing_put(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in (0..n).rev() {
        tx.put(ROOT, i.to_string(), i).unwrap();
    }
    tx.commit();
    doc
}
