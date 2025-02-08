import * as fs from "fs";
import { next as Automerge } from "@automerge/automerge"

const load = Automerge.load;
const splice = Automerge.splice;
const init = Automerge.init;
const change = Automerge.change;

function bench(label: string, f: Function) {
  const start = performance.now();
  const result = f()
  const end = performance.now();
  const elapsed = end - start;
  console.log(`        ${label}: ${Math.floor(elapsed)}ms`);
  return result
}

function rand(max) {
  return Math.floor(Math.random() * max);
}

describe("Automerge Battery", () => {
  describe("bestiary", () => {
    it("load webstraits", async () => {
      let file = fs.readFileSync("./benches/webstraits.amrg");
      let doc = bench("load", () => Automerge.load(file))
    })
    it("load stephan", async () => {
      let file = fs.readFileSync("./benches/stephen.amrg");
      let doc = bench("load", () => Automerge.load(file))
    })
  })
  describe("text ops", () => {
    it("splice big doc", async () => {
        const size = 1024 * 10;
        let doc = init<any>();
        let bigText = ".".repeat(size);
        doc = bench("set", () => Automerge.change(doc, d => { d.text = bigText }))
        doc = bench("splice", () => change(doc, d => splice(d, ["text"], rand(size), 4, "abcd")))
        doc = bench("splice", () => change(doc, d => splice(d, ["text"], rand(size), 4, "abcd")))
        doc = bench("splice", () => change(doc, d => splice(d, ["text"], rand(size), 4, "abcd")))
        //console.log(doc)
    })
  })
})
