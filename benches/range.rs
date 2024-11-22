use automerge::{transaction::Transactable, Automerge, ReadDoc, ROOT};
use automerge_battery::TestItem;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn doc(n: u64) -> TestItem<Automerge> {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, i.to_string(), i.to_string()).unwrap();
    }
    tx.commit();
    TestItem::new(format!("doc_{}",n),doc)
}

#[divan::bench(args=[doc(100_000)])]
fn range(doc: &TestItem<Automerge>) {
    let range = doc.item.values(ROOT);
    range.for_each(drop);
}

#[divan::bench(args=[doc(100_000)])]
fn range_at(doc: &TestItem<Automerge>) {
    let range = doc.item.values_at(ROOT, &doc.item.get_heads());
    range.for_each(drop);
}

