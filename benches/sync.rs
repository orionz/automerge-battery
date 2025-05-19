use automerge::{
    sync::{self, SyncDoc},
    transaction::Transactable,
    ReadDoc, Automerge, ROOT,
};
use automerge_battery::{rand, list_splice_100, text_splice_100};
use std::time::Duration;
use divan::Bencher;

const N : u64 = 100_000;

fn main() {
    divan::main();
}

#[derive(Clone, Default)]
struct DocWithSync {
    doc: Automerge,
    peer_state: sync::State,
}

impl DocWithSync {
  fn sync(&mut self, other: &mut DocWithSync) {
    while let Some(message1) = self.doc.generate_sync_message(&mut self.peer_state) {
       other.doc.receive_sync_message(&mut other.peer_state, message1).unwrap();
       if let Some(message2) = other.doc.generate_sync_message(&mut other.peer_state) {
         self.doc.receive_sync_message(&mut self.peer_state, message2).unwrap()
       }
    }
  }
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
fn full_sync(bencher: Bencher, test: &Test) {
    let doc = test.init();
    let mut doc1 = doc.clone();
    let mut doc2 = DocWithSync::default();
    bencher.bench_local(|| {
        doc1.sync(&mut doc2);
    })
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn tiny_text_sync(bencher: Bencher) {
  let mut doc1 : DocWithSync = text_splice_100(N).into();
  let mut doc2 : DocWithSync = Automerge::new().into();
  let len = N as usize;
  doc1.sync(&mut doc2);
  let (_, text) = doc1.doc.get(ROOT, "content").unwrap().unwrap();
  bencher.bench_local(|| {
    let mut tx = doc1.doc.transaction();
    let pos = rand() % len;
    tx.splice_text(&text, pos, 1, "_").unwrap();
    tx.commit();
    doc1.sync(&mut doc2);
  })
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn tiny_list_sync(bencher: Bencher) {
  let mut doc1 : DocWithSync = list_splice_100(N).into();
  let mut doc2 : DocWithSync = Automerge::new().into();
  let len = N as usize;
  doc1.sync(&mut doc2);
  let (_, list) = doc1.doc.get(ROOT, "content").unwrap().unwrap();
  bencher.bench_local(|| {
    let mut tx = doc1.doc.transaction();
    let pos = rand() % len;
    tx.splice(&list, pos, 0, vec!["_".into()]).unwrap();
    tx.commit();
    doc1.sync(&mut doc2);
  })
}
