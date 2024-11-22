use automerge::{transaction::Transactable, Automerge, ObjType, ROOT};
use divan::{AllocProfiler, Bencher};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::time::Duration;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const N: u64 = 10_000;

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

fn big_paste_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    tx.put(ROOT, "content", random_string(n)).unwrap();
    tx.commit();
    doc
}

fn poorly_simulated_typing_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();

    let mut tx = doc.transaction();
    let obj = tx.put_object(ROOT, "content", ObjType::Text).unwrap();
    tx.commit();

    for i in 0..n {
        let mut tx = doc.transaction();
        let pos: usize = i.try_into().unwrap();
        tx.splice_text(&obj, pos, 0, &random_string(1)).unwrap();
        tx.commit();
    }

    doc
}

fn maps_in_maps_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();

    let mut map = ROOT;

    for i in 0..n {
        // we make a map
        map = tx.put_object(map, i.to_string(), ObjType::Map).unwrap();
    }

    tx.commit();
    doc
}

fn deep_history_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    for i in 0..n {
        let mut tx = doc.transaction();
        tx.put(ROOT, "x", i.to_string()).unwrap();
        tx.put(ROOT, "y", i.to_string()).unwrap();
        tx.commit();
    }

    doc
}

#[derive(Debug)]
enum Test {
    BigPaste,
    MapsInMaps,
    DeepHistory,
    PoorlySimulatedTyping,
}

impl Test {
    fn init(&self) -> Automerge {
        match self {
            Self::BigPaste => big_paste_doc(N),
            Self::MapsInMaps => maps_in_maps_doc(N),
            Self::DeepHistory => deep_history_doc(N),
            Self::PoorlySimulatedTyping => poorly_simulated_typing_doc(N),
        }
    }
}

#[divan::bench(args=[Test::BigPaste,Test::MapsInMaps,Test::DeepHistory,Test::PoorlySimulatedTyping], max_time = Duration::from_secs(3))]
fn save(bencher: Bencher, test: &Test) {
    let doc = test.init();
    bencher.bench_local(|| -> Vec<u8> { doc.save() })
}

#[divan::bench(args=[Test::BigPaste,Test::MapsInMaps,Test::DeepHistory,Test::PoorlySimulatedTyping], max_time = Duration::from_secs(3))]
fn load(bencher: Bencher, test: &Test) {
    let data = test.init().save();
    bencher.bench_local(|| {
        Automerge::load(data.as_slice()).unwrap();
    })
}
