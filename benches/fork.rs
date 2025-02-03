use automerge::{transaction::Transactable, AutoCommit, ChangeHash, ObjType, ROOT};
use divan::Bencher;
use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng, RngCore};
use std::time::Duration;

/*
use divan::AllocProfiler;
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
*/

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
fn gen_doc(n: u64, chunk: u64) -> (AutoCommit, Vec<ChangeHash>) {
    let mut hashes = vec![];
    let mut doc = AutoCommit::new();
    let text = doc.put_object(ROOT, "content", ObjType::Text).unwrap();
    doc.splice_text(&text, 0, 0, &random_string(chunk)).unwrap();
    let mut len = chunk;
    for _ in 0..(n / chunk) {
        let pos = (thread_rng().next_u32() as u64 % len) as usize;
        doc.splice_text(&text, pos, 0, &random_string(chunk))
            .unwrap();
        hashes.push(doc.get_heads().first().unwrap().clone());
        len += chunk;
    }
    (doc, hashes)
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn fork_1x(bencher: Bencher) {
    let (mut doc, hashes) = gen_doc(N, 1);
    bencher.bench_local(|| {
        let hash = hashes.choose(&mut thread_rng()).cloned().unwrap();
        let _ = doc.fork_at(&[hash]);
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn fork_10x(bencher: Bencher) {
    let (mut doc, hashes) = gen_doc(N, 10);
    bencher.bench_local(|| {
        let hash = hashes.choose(&mut thread_rng()).cloned().unwrap();
        let _ = doc.fork_at(&[hash]);
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn fork_100x(bencher: Bencher) {
    let (mut doc, hashes) = gen_doc(N, 100);
    bencher.bench_local(|| {
        let hash = hashes.choose(&mut thread_rng()).cloned().unwrap();
        let _ = doc.fork_at(&[hash]);
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn fork_1000x(bencher: Bencher) {
    let (mut doc, hashes) = gen_doc(N, 1000);
    bencher.bench_local(|| {
        let hash = hashes.choose(&mut thread_rng()).cloned().unwrap();
        let _ = doc.fork_at(&[hash]);
    })
}
