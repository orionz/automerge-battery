use automerge::marks::{ExpandMark, Mark};
use automerge::transaction::Transactable;
use automerge::{Automerge, ReadDoc, ROOT};
use automerge_battery::text_splice_100;
use divan::Bencher;
use getrandom;
use std::cmp::{max, min};
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

// add mark
// splice_with_mark
// apply_with_mark

#[divan::bench(max_time = Duration::from_secs(3))]
fn add_mark(bencher: Bencher) {
    let mut doc = text_splice_100(N);
    bencher.bench_local(|| {
        add_random_marks(&mut doc, 1);
    });
}

fn rand() -> usize {
    let mut buf = [0u8; size_of::<usize>()];
    getrandom::fill(&mut buf).unwrap();
    usize::from_ne_bytes(buf)
}

fn add_random_marks(d: &mut Automerge, num: usize) {
    let (_, text) = d.get(ROOT, "content").unwrap().unwrap();
    let mut tx = d.transaction();
    let len = tx.length(&text);
    for i in 0..num {
        let a = rand() % len;
        let b = rand() % len;
        let mark = Mark::new(format!("mark{}", i), true, min(a, b), max(a, b));
        tx.mark(&text, mark, ExpandMark::Both).unwrap();
    }
    tx.commit();
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_with_marks(bencher: Bencher) {
    let mut doc = text_splice_100(N);
    add_random_marks(&mut doc, 1000);
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| {
        let mut d = doc.fork();
        for _ in 0..1000 {
            let mut tx = d.transaction();
            let pos = rand() % tx.length(&text);
            tx.splice_text(&text, pos, 0, "XXX").unwrap();
            tx.commit();
        }
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn splice_without_marks(bencher: Bencher) {
    let doc = text_splice_100(N);
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| {
        let mut d = doc.fork();
        for _ in 0..1000 {
            let mut tx = d.transaction();
            let pos = rand() % tx.length(&text);
            tx.splice_text(&text, pos, 1, "XXX").unwrap();
            tx.commit();
        }
    });
}
