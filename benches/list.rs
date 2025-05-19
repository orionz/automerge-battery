use automerge::transaction::Transactable;
use automerge::{PatchLog, Automerge, ReadDoc, ROOT};
use automerge_battery::{list_splice_100, text_splice_100};
use divan::Bencher;
use getrandom;
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_cursor_now(bencher: Bencher) {
    let doc = list_splice_100(N);
    let len = N as usize;
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| {
      let pos = rand() % len;
      let cursor = doc.get_cursor(&list, pos, None).unwrap();
      assert_eq!(doc.get_cursor_position(&list, &cursor, None).unwrap(), pos)
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_cursor_at(bencher: Bencher) {
    let mut doc = list_splice_100(N);
    let len = N as usize;
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    let heads = doc.get_heads();
    let mut tx = doc.transaction();
    tx.splice(&list, len/2, 1, vec!["x".into(),"y".into(),"z".into()]).unwrap();
    tx.commit();
    bencher.bench_local(|| {
      let pos = rand() % len;
      let cursor = doc.get_cursor(&list, pos, Some(&heads)).unwrap();
      assert_eq!(doc.get_cursor_position(&list, &cursor, Some(&heads)).unwrap(), pos)
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_splice_index_now(bencher: Bencher) {
    let mut doc = list_splice_100(N);
    let len = N as usize;
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| {
      let pos = rand() % len;
      let mut tx = doc.transaction();
      tx.insert(&list, pos, "x").unwrap();
      tx.commit();
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn list_splice_index_at(bencher: Bencher) {
    let mut doc = list_splice_100(N);
    let len = N as usize;
    let (_, list) = doc.get(ROOT, "content").unwrap().unwrap();
    let heads = doc.get_heads();
    let mut tx = doc.transaction();
    tx.splice(&list, len/2, 1, vec!["x".into(),"y".into(),"z".into()]).unwrap();
    tx.commit();
    bencher.bench_local(|| {
      let pos = rand() % len;
      let mut tx = doc.transaction_at(PatchLog::null(), &heads);
      tx.insert(&list, pos, "x").unwrap();
      tx.commit();
    });
}

fn rand() -> usize {
    let mut buf = [0u8; size_of::<usize>()];
    getrandom::fill(&mut buf).unwrap();
    usize::from_ne_bytes(buf)
}
