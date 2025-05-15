use automerge::sync;
use automerge::sync::SyncDoc;
use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjType, ReadDoc, ROOT};

use divan::Bencher;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng, RngCore};
use std::time::Duration;

const N: u64 = 100_000;

fn main() {
    divan::main();
}

fn random_string(n: u64) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n as usize)
        .map(char::from)
        .collect();

    rand_string
}

#[inline(never)]
fn gen_text_doc(n: u64, chunk: u64) -> AutoCommit {
    let mut doc = AutoCommit::new();
    let text = doc.put_object(ROOT, "content", ObjType::Text).unwrap();
    doc.splice_text(&text, 0, 0, &random_string(chunk)).unwrap();
    let mut len = chunk;
    for _ in 0..(n / chunk) {
        let pos = (thread_rng().next_u32() as u64 % len) as usize;
        doc.splice_text(&text, pos, 0, &random_string(chunk))
            .unwrap();
        len += chunk;
    }
    assert_eq!(doc.stats().num_ops, n + 1 + chunk);
    doc
}

/*
#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn single_ch_splice(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();
    bencher.bench_local(|| {
      let pos = (thread_rng().next_u32() as u64 % N) as usize;
      doc.splice_text(&text, pos, 0, ".").unwrap();
      doc.commit();
    })
}
*/

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn single_ch_sync(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    let mut remote = doc.fork();
    let mut doc_s = sync::State::new();
    let mut remote_s = sync::State::new();
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();

    sync(&mut doc, &mut doc_s, &mut remote, &mut remote_s);

    bencher.bench_local(|| {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc.splice_text(&text, pos, 0, ".").unwrap();
        doc.commit();
        sync(&mut doc, &mut doc_s, &mut remote, &mut remote_s);
        assert_eq!(doc.get_heads(), remote.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn single_ch_apply(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    let mut remote = doc.fork();
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();

    bencher.bench_local(|| {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc.splice_text(&text, pos, 0, ".").unwrap();
        let change = doc.get_last_local_change().unwrap();
        remote.apply_changes(vec![change.clone()]).unwrap();
        assert_eq!(doc.get_heads(), remote.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn single_ch_merge(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    let mut remote = doc.fork();
    let (_, text) = doc.get(ROOT, "content").unwrap().unwrap();

    bencher.bench_local(|| {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc.splice_text(&text, pos, 0, ".").unwrap();
        remote.merge(&mut doc).unwrap();
        assert_eq!(doc.get_heads(), remote.get_heads());
    })
}

fn sync(d1: &mut AutoCommit, s1: &mut sync::State, d2: &mut AutoCommit, s2: &mut sync::State) {
    while d1.get_heads() != d2.get_heads() {
        if let Some(msg) = d1.sync().generate_sync_message(s1) {
            d2.sync().receive_sync_message(s2, msg).unwrap();
        }
        if let Some(msg) = d2.sync().generate_sync_message(s2) {
            d1.sync().receive_sync_message(s1, msg).unwrap();
        }
    }
}
