import {next as Automerge} from "@automerge/automerge";
import crypto from "crypto";

function generateRandomString(length) {
  return crypto.randomBytes(length).toString('base64');
}

function percentile(array, p) {
  return array[Math.floor((p / 100) * array.length)];
}

function stress(iterations) {
  const smallString = generateRandomString(16);
  const mediumString = generateRandomString(1024);
  const largeString = generateRandomString(16 * 1024);

  const times = [];
  for (let i = 0; i < iterations; i += 1) {
    const start = Date.now();
    try {
      Automerge.change(Automerge.init(), doc => {
        doc.smallString = smallString;
        doc.mediumString = mediumString;
        doc.largeString = largeString;
      });
    } finally {
      times.push(Date.now() - start);
    }
  }

  const sorted = times.sort();
  return {
    p50: percentile(sorted, 50),
    p75: percentile(sorted, 75),
    p99: percentile(sorted, 99),
  };
}

console.log(stress(10));
