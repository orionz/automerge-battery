use automerge::ObjType;
use automerge::{transaction::Transactable, AutoCommit, ObjId, ROOT};
use divan::Bencher;
use std::time::Duration;

//use divan::{AllocProfiler};
//#[global_allocator]
//static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn load_commands() -> (Vec<(usize, isize, String)>, AutoCommit, ObjId) {
    let contents = include_str!("./edits.json");
    let edits = jzon::parse(contents).expect("cant parse edits");
    let mut commands = vec![];
    for i in 0..edits.len() {
        let edit = &edits[i];
        let pos: usize = edit.as_array().unwrap()[0].as_u64().unwrap() as usize;
        let del: isize = edit.as_array().unwrap()[1].as_i64().unwrap() as isize;
        let mut vals = String::new();
        for j in 2..edit.as_array().unwrap().len() {
            let v = edit.as_array().unwrap()[j].as_str().unwrap();
            vals.push_str(v);
        }
        commands.push((pos, del, vals));
    }
    let mut doc = AutoCommit::new();
    doc.update_diff_cursor();
    let text = doc.put_object(ROOT, "text", ObjType::Text).unwrap();
    (commands, doc, text)
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn edit_trace_single_tx(bencher: Bencher) {
    let (commands, mut doc, text) = load_commands();

    bencher.bench_local(|| {
        for (pos, del, vals) in commands.iter() {
            doc.splice_text(&text, *pos, *del, &vals).unwrap();
        }
        doc.commit();
    })
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn edit_trace_many_tx(bencher: Bencher) {
    let (commands, mut doc, text) = load_commands();

    bencher.bench_local(|| {
        for (pos, del, vals) in commands.iter() {
            doc.splice_text(&text, *pos, *del, &vals).unwrap();
            doc.commit();
        }
    })
}
