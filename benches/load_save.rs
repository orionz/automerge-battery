use automerge::{transaction::Transactable, Automerge, ObjType, ROOT};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use automerge_battery::TestItem;
use divan::AllocProfiler;

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

fn big_paste_doc(n: u64) -> TestItem<Automerge> {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    tx.put(ROOT, "content", random_string(n)).unwrap();
    tx.commit();

    TestItem::new("big_paste_doc",doc)
}

fn poorly_simulated_typing_doc(n: u64) -> TestItem<Automerge> {
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

    TestItem::new("poorly_simulated_typing_doc",doc)
}

fn maps_in_maps_doc(n: u64) -> TestItem<Automerge> {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();

    let mut map = ROOT;

    for i in 0..n {
        // we make a map
        map = tx.put_object(map, i.to_string(), ObjType::Map).unwrap();
    }

    tx.commit();
    TestItem::new("maps_in_maps_doc",doc)
}

fn deep_history_doc(n: u64) -> TestItem<Automerge> {
    let mut doc = Automerge::new();
    for i in 0..n {
        let mut tx = doc.transaction();
        tx.put(ROOT, "x", i.to_string()).unwrap();
        tx.put(ROOT, "y", i.to_string()).unwrap();
        tx.commit();
    }

    TestItem::new("deep_history_doc",doc)
}

#[divan::bench(args=[
  big_paste_doc(N),
  poorly_simulated_typing_doc(N),
  maps_in_maps_doc(N),
  deep_history_doc(N)
])]
fn save(doc: &TestItem<Automerge>) -> TestItem<Vec<u8>> {
    doc.map(|i| i.save())
}

#[divan::bench(args=[
  save(&big_paste_doc(N)),
  save(&poorly_simulated_typing_doc(N)),
  save(&maps_in_maps_doc(N)),
  save(&deep_history_doc(N))
])]
fn load(save_data: &TestItem<Vec<u8>>) {
    Automerge::load(save_data.item.as_slice()).unwrap();
}

