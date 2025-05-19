use automerge::transaction::Transactable;
use automerge::{Automerge, ReadDoc, ROOT};
use automerge_battery::{rand, list_splice_100, text_splice_100};
use divan::Bencher;
use getrandom;
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn text_len_now(bencher: Bencher) {
    let doc = text_splice_100(N);
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| assert_eq!(doc.length(&text), N as usize));
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_len_now(bencher: Bencher) {
    let doc = list_splice_100(N);
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| assert_eq!(doc.length(&list), N as usize));
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn text_len_at(bencher: Bencher) {
    let mut doc = text_splice_100(N);
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();
    let heads = doc.get_heads();
    let mut tx = doc.transaction();
    let pos = rand() % (N as usize - 10);
    tx.splice_text(&text, pos, 9, "01234567890").unwrap();
    tx.commit();
    bencher.bench_local(|| assert_eq!(doc.length_at(&text, &heads), N as usize));
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_len_at(bencher: Bencher) {
    let mut doc = list_splice_100(N);
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    let heads = doc.get_heads();
    let mut tx = doc.transaction();
    let pos = rand() % (N as usize - 10);
    let letters = "0123456789".chars().map(|c| c.into());
    tx.splice(&list, pos, 9, letters).unwrap();
    tx.commit();
    bencher.bench_local(|| assert_eq!(doc.length_at(&list, &heads), N as usize));
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn map_len_now(bencher: Bencher) {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..N {
        tx.put(&ROOT, format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    tx.commit();

    bencher.bench_local(|| assert_eq!(doc.length(&ROOT), N as usize));
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn map_len_at(bencher: Bencher) {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..N {
        tx.put(&ROOT, format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    tx.commit();

    let heads = doc.get_heads();
    let mut tx = doc.transaction();
    tx.put(&ROOT, "next", "value").unwrap();
    tx.commit();

    bencher.bench_local(|| assert_eq!(doc.length_at(&ROOT, &heads), N as usize));
}

