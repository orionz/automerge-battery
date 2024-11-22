use automerge::{
    sync::{self, SyncDoc},
    transaction::Transactable,
    Automerge, ROOT,
};
use automerge_battery::TestItem;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
  divan::main();
}

#[derive(Clone, Default )]
struct DocWithSync {
    doc: Automerge,
    peer_state: sync::State,
}

impl From<Automerge> for DocWithSync {
    fn from(doc: Automerge) -> Self {
        Self {
            doc,
            peer_state: sync::State::default(),
        }
    }
}

fn one_tx_increasing_put(n: u64) -> TestItem<DocWithSync> {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, i.to_string(), i).unwrap();
    }
    tx.commit();
    TestItem::new("single_tx_increasing_put", doc.into())
}

fn many_tx_increasing_put(n: u64) -> TestItem<DocWithSync> {
    let mut doc = Automerge::default();

    for i in 0..n {
        let mut tx = doc.transaction();
        tx.put(ROOT, i.to_string(), i).unwrap();
        tx.commit();
    }

    TestItem::new("many_tx_increasing_put", doc.into())
}

#[divan::bench(args=[many_tx_increasing_put(10_000),one_tx_increasing_put(10_000)])]
fn sync(doc1: &TestItem<DocWithSync>) {
    let mut doc1 = doc1.item.clone();
    let mut doc2 = DocWithSync::default();
    while let Some(message1) = doc1.doc.generate_sync_message(&mut doc1.peer_state) {
        doc2.doc
            .receive_sync_message(&mut doc2.peer_state, message1)
            .unwrap();

        if let Some(message2) = doc2.doc.generate_sync_message(&mut doc2.peer_state) {
            doc1.doc
                .receive_sync_message(&mut doc1.peer_state, message2)
                .unwrap()
        }
    }
}

