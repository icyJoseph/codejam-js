// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051635/0000000000104e03
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  problems: [],
  results: [],
  numberOfTests: null,
  count: null,
  inc() {
    this.count = this.count === null ? 1 : this.count + 1;
  },
  set(val) {
    this.numberOfTests = val;
  },
  get() {
    const current = this.count;
    this.inc();
    return current;
  },
  addProblem(problem) {
    this.problems = [...this.problems, problem];
  },
  addResult(result) {
    this.results = [...this.results, result];
  }
};

const baseOneIndex = (_, i) => i + 1;
const makeAxis = length => Array.from({ length }, baseOneIndex);
const flatten = arr => [].concat(...arr);
const isValid = (vecA, vecB) => {
  const [a, b] = vecA.split(" ").map(e => parseInt(e));
  const [x, y] = vecB.split(" ").map(e => parseInt(e));
  if (a === x) {
    return false;
  }
  if (b === y) {
    return false;
  }
  if (a + b === x + y) {
    return false;
  }
  if (a - b === x - y) {
    return false;
  }
  return true;
};

const exploreSequence = (curr, src, acc) => {
  // remove current from the source arr
  const rest = src.filter(point => point !== curr);
  /// if only one left after curr
  if (rest.length === 1) {
    // get the  last in the accumulated
    const [prev] = acc.slice(-1);
    // get the one left from rest
    const [final] = rest;
    // compare curr to final and prev and add both if valid
    return isValid(curr, final) && isValid(prev, curr)
      ? [...acc, curr, final]
      : null;
  } // otherwise return null to bail out

  // find valid points in src
  const next = src.filter(point => isValid(curr, point));
  // if nothing left is valid
  if (next.length === 0) {
    // bail out
    return null;
  }
  // take a random slice of the possibilities
  const index = Math.floor(Math.random() * next.length);
  const randomSlice = next.length > 5 ? next.slice(index, index + 1) : next;

  // reduce over the random slice trying to find a valid sequence
  // once the sequence is not falsy, conclude the reduce by returning the non
  // falsy result
  return randomSlice.reduce((prev, prob, index) => {
    if (prev) {
      return prev;
    }
    return exploreSequence(prob, rest, [...acc, curr]);
  }, null);
};

const createSequence = line => {
  // consume Rows and Cols
  const [R, C] = line.split(" ").map(e => parseInt(e));
  // make Axes
  const rows = makeAxis(R);
  const cols = makeAxis(C);
  // make all Matrix Points
  const points = flatten(rows.map(row => cols.map(col => `${row} ${col}`)));
  // reduce through them finding a valid sequence
  return points.reduce(
    ({ sequence, done }, curr, index, src) => {
      // if we find a solution earlier, carry that through to the end
      if (done) {
        return { sequence, done };
      }
      // explore the sequence space
      const seq = exploreSequence(curr, src, []) || [];
      // assign the value of the sequence
      // and if the sequence has the same length as the src
      // conclude that we are done
      return { sequence: seq, done: seq.length === src.length };
    },
    { sequence: [], done: false }
  );
};

rl.on("line", function(line) {
  const caseNumber = caseTracker.get();
  // for the first line, which specifies the number of cases
  if (!caseNumber) {
    return caseTracker.set(parseInt(line));
  }

  caseTracker.addProblem(line);

  if (caseNumber === caseTracker.numberOfTests) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((line, index) => {
    // find a sequence
    const { sequence } = createSequence(line);
    // calculate result heading
    const result = sequence.length === 0 ? "IMPOSSIBLE" : "POSSIBLE";
    // add the result heading
    // if it was possible, add each cell as a new line
    if (result === "POSSIBLE") {
      const withSteps = `Case #${index + 1}: ${result}\n${sequence.join("\n")}`;
      return caseTracker.addResult(withSteps);
    }
    return caseTracker.addResult(`Case #${index + 1}: ${result}`);
  });
  // after solving forEach problem
  // print everything in results
  caseTracker.results.forEach(result => console.log(result));
  // exit with code 0
  process.exit(0);
});
