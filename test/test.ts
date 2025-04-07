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
  describe.only("from string performance", () => {
      console.log("AUTOMERGE ", Automerge.stats(Automerge.init()));
      let lengths = [1e1, 1e2, 1e3, 1e4, 1e5, 1e6];
      let strings : string[] = lengths.map(n => getRandomString(n));
      for (let s of strings) {
        bench(`from ${s.length}`, () => Automerge.from({ s }));
      }
  })
})


function getRandomString(length: number) : string {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  return Array.from({ length }).reduce((acc: string, next) => acc + characters.charAt(Math.floor(Math.random() * characters.length)), '');
}

