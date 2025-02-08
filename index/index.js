import * as fs from "fs";
import { next as Automerge } from "@automerge/automerge"

function bench(action, f) {
  const start = performance.now();
  let result = f()
  const end = performance.now();
  const elapsed = end - start;
  console.log(`        ${action}: ${Math.floor(elapsed)}ms`);
  return result
}

let file = fs.readFileSync("./benches/stephen.amrg");
let doc = bench("load", () => Automerge.load(file));
let saved = bench("save", () => Automerge.save(doc));
let changes = bench("getAllChanges", () => Automerge.getAllChanges(doc));
let newDoc = Automerge.init();
bench("applyChanges", () => Automerge.applyChanges(newDoc, changes));
