use automerge::{transaction::Transactable, Automerge, ObjType, ROOT};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng, RngCore};
use std::fmt::{Display, Error, Formatter};

pub struct TestItem<T> {
    pub label: String,
    pub item: T,
}

impl<T> TestItem<T> {
    pub fn new<S: Into<String>>(label: S, item: T) -> TestItem<T> {
        Self {
            label: label.into(),
            item,
        }
    }

    pub fn map<B, F: Fn(&T) -> B>(&self, f: F) -> TestItem<B> {
        TestItem {
            label: self.label.clone(),
            item: f(&self.item),
        }
    }
}

impl<T> Display for TestItem<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        self.label.fmt(fmt)
    }
}

#[inline(never)]
pub fn maps_in_maps_doc(n: u64) -> Automerge {
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

fn random_string(n: u64) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n as usize)
        .map(char::from)
        .collect();

    rand_string
}

#[inline(never)]
pub fn big_paste_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    let text = tx.put_object(ROOT, "content", ObjType::Text).unwrap();
    tx.splice_text(text, 0, 0, &random_string(n)).unwrap();
    tx.commit();
    doc
}

#[inline(never)]
pub fn big_random_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    let text = tx.put_object(ROOT, "content", ObjType::Text).unwrap();
    tx.splice_text(&text, 0, 0, &random_string(1)).unwrap();
    let mut len = 1;
    for _ in 0..n {
        let pos = thread_rng().next_u32() as usize % len;
        tx.splice_text(&text, pos, 0, &random_string(1)).unwrap();
        len += 1;
    }
    tx.commit();
    doc
}

#[inline(never)]
pub fn text_splice_100(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    let text = tx.put_object(ROOT, "content", ObjType::Text).unwrap();
    tx.splice_text(&text, 0, 0, &random_string(1)).unwrap();
    let mut len = 1;
    for _ in 0..(n / 100) {
        let pos = thread_rng().next_u32() as usize % len;
        tx.splice_text(&text, pos, 0, &random_string(100)).unwrap();
        len += 1;
    }
    tx.commit();
    doc
}

#[inline(never)]
pub fn poorly_simulated_typing_doc(n: u64) -> Automerge {
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

#[inline(never)]
pub fn deep_history_doc(n: u64) -> Automerge {
    let mut doc = Automerge::new();
    for i in 0..n {
        let mut tx = doc.transaction();
        tx.put(ROOT, "x", i.to_string()).unwrap();
        tx.put(ROOT, "y", i.to_string()).unwrap();
        tx.commit();
    }

    doc
}
