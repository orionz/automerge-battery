//use automerge::{transaction::Transactable, AutoCommit, Automerge, ObjType, ReadDoc, ROOT};
//use divan::Bencher;
//use rand::distributions::Alphanumeric;
//use rand::{thread_rng, Rng, RngCore};
//use std::time::Duration;

/*
use divan::AllocProfiler;
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();
*/

//const N: u64 = 100_000;

fn main() {
    divan::main();
}
/*

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

#[inline(never)]
fn gen_map_doc(n: u64, chunk: u64) -> AutoCommit {
    let mut doc = AutoCommit::new();
    let map = doc.put_object(ROOT, "content", ObjType::Map).unwrap();
    for _ in 0..(n / chunk) {
        for _ in 0..chunk {
            doc.put(
                &map,
                format!("key{}", thread_rng().next_u32() % 10000),
                &random_string(10),
            )
            .unwrap();
        }
    }
    doc
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_100k_old(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_100k_new(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 1);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_batch(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_10k_old(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 10);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_10k_new(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 10);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_batch(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_1k_old(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 100);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_1k_new(bencher: Bencher) {
    let mut doc = gen_text_doc(N, 100);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_batch(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn map_apply_changes_1k(bencher: Bencher) {
    let mut doc = gen_map_doc(N, 100);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn map_apply_changes_1k_v2(bencher: Bencher) {
    let mut doc = gen_map_doc(N, 100);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_batch(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn map_apply_changes_10k(bencher: Bencher) {
    let mut doc = gen_map_doc(N, 10);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn map_apply_changes_100k(bencher: Bencher) {
    let mut doc = gen_map_doc(N, 1);
    bencher.bench_local(|| {
        let mut doc2 = Automerge::new();
        let changes = doc.get_changes(&[]).into_iter().map(|c| c.clone());
        doc2.apply_changes_iter(changes).unwrap();
    })
}

const M: u64 = 3000;

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_100k_old(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 1);
    let mut doc2 = doc1.fork();
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      for _ in 0..M {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc1.splice_text(&text, pos, 0, &random_string(1)).unwrap();
      }
      let change = doc1.get_last_local_change().unwrap();
      doc2.apply_changes_iter(vec![change]).unwrap();
      assert_eq!(doc1.get_heads(),doc2.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_100k_new(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 1);
    let mut doc2 = doc1.fork();
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      for _ in 0..M {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc1.splice_text(&text, pos, 0, &random_string(1)).unwrap();
      }
      let change = doc1.get_last_local_change().unwrap();
      doc2.apply_changes_batch(vec![change]).unwrap();
      assert_eq!(doc1.get_heads(),doc2.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_100k_noop(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 1);
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      for _ in 0..M {
        let pos = (thread_rng().next_u32() as u64 % N) as usize;
        doc1.splice_text(&text, pos, 0, &random_string(1)).unwrap();
      }
      let _change = doc1.get_last_local_change().unwrap();
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_1k_old(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 100);
    let mut doc2 = doc1.fork();
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      let pos = (thread_rng().next_u32() as u64 % N) as usize;
      doc1.splice_text(&text, pos, 0, &random_string(M)).unwrap();
      let change = doc1.get_last_local_change().unwrap();
      doc2.apply_changes_batch(vec![change]).unwrap();
      assert_eq!(doc1.get_heads(),doc2.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_1k_new(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 100);
    let mut doc2 = doc1.fork();
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      let pos = (thread_rng().next_u32() as u64 % N) as usize;
      doc1.splice_text(&text, pos, 0, &random_string(M)).unwrap();
      let change = doc1.get_last_local_change().unwrap();
      doc2.apply_changes_batch(vec![change]).unwrap();
      assert_eq!(doc1.get_heads(),doc2.get_heads());
    })
}

#[inline(never)]
#[divan::bench(max_time = Duration::from_secs(3))]
fn text_apply_changes_single_op_1k_noop(bencher: Bencher) {
    let mut doc1 = gen_text_doc(N, 100);
    let text = doc1.get(ROOT, "content").unwrap().unwrap().1;
    bencher.bench_local(|| {
      let pos = (thread_rng().next_u32() as u64 % N) as usize;
      doc1.splice_text(&text, pos, 0, &random_string(M)).unwrap();
      let _change = doc1.get_last_local_change().unwrap();
    })
}
*/
