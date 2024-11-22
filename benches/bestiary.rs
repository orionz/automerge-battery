use automerge::Automerge;
use automerge_battery::TestItem;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
  divan::main();
}

#[divan::bench(args = [ 
  "./benches/embark.automerge",
  "./benches/moby-dick.automerge"
])]
fn load(filename: &str) -> TestItem<Automerge> {
  let data = std::fs::read(filename).unwrap();
  TestItem::new(filename, Automerge::load(data.as_slice()).unwrap())
}

#[divan::bench(args = [
  load("./benches/embark.automerge"),
  load("./benches/moby-dick.automerge")
])]
fn save(doc: &TestItem<Automerge>) {
  doc.item.save();
}

