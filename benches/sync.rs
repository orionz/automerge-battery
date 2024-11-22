use automerge::{
    sync::{self, SyncDoc},
    transaction::Transactable,
    Automerge, ROOT,
};
//use divan::AllocProfiler;
use divan::Bencher;

//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

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

fn one_tx_increasing_put(n: u64) -> DocWithSync {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for i in 0..n {
        tx.put(ROOT, i.to_string(), i).unwrap();
    }
    tx.commit();
    doc.into()
}

fn many_tx_increasing_put(n: u64) -> DocWithSync {
    let mut doc = Automerge::default();

    for i in 0..n {
        let mut tx = doc.transaction();
        tx.put(ROOT, i.to_string(), i).unwrap();
        tx.commit();
    }

    doc.into()
}

#[derive(Debug)]
enum Test {
  ManyTx(u64),
  OneTx(u64),
}

impl Test {
  fn init(&self) -> DocWithSync {
    match self {
      Self::ManyTx(n) => many_tx_increasing_put(*n),
      Self::OneTx(n) => one_tx_increasing_put(*n),
    }
  }
}

#[divan::bench(args=[
  Test::ManyTx(10_000),
  Test::OneTx(10_000),
])]
fn sync(bencher: Bencher, test: &Test) {
    let doc = test.init();
    let mut doc1 = doc.clone();
    let mut doc2 = DocWithSync::default();
    bencher.bench_local(|| {
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
    })
}

