use automerge::{transaction::Transactable, Automerge, ReadDoc, ROOT};
//use divan::AllocProfiler;
use divan::Bencher;

//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, i.to_string(), i.to_string()).unwrap();
    }
    tx.commit();
    doc
}

#[divan::bench(args=[100_000])]
fn range(bencher: Bencher, n: u64) {
    let doc = doc(n);
    bencher.bench_local(|| {
      let range = doc.values(ROOT);
      range.for_each(drop);
    })
}

#[divan::bench(args=[100_000])]
fn range_at(bencher: Bencher, n: u64) {
    let doc = doc(n);
    bencher.bench_local(|| {
      let range = doc.values_at(ROOT, &doc.get_heads());
      range.for_each(drop);
    });
}

