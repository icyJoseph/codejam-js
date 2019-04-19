// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/00000000000881da
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

const S = "S";
const E = "E";
const countInStr = (str, c) => str.split("").filter(e => e === c).length;
const opposite = a => (a === S ? E : S);
const mirrorPath = path => {
  // all paths have an even number of steps
  return path.split().reduce(
    (prev, curr) => {
      const nextSquare = `${prev.maybeSqr}${curr}`;
      if (countInStr(S) === countInStr(E)) {
        // we have a square path
        const nextSteps = nextSquare
          .split("")
          .map(opposite)
          .join("");

        return { maybeSqr: "", steps: `${prev.steps}${nextSteps}` };
      }
      return { maybeSqr: nextSquare, steps: prev.steps };
    },

    { maybeSqr: "", steps: "" }
  );
};

rl.on("line", function(line) {
  //code goes here
  const caseNumber = caseTracker.get();
  // for the first line, which specifies the number of cases
  if (!caseNumber) {
    return caseTracker.set(parseInt(line));
  }

  if (caseNumber && caseNumber % 2 === 0) {
    caseTracker.addProblem(line);
  }

  if (caseNumber / 2 === caseTracker.numberOfTests) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((line, index) => {
    const { steps } = mirrorPath(line);
    caseTracker.addResult(`Case #${index + 1}: ${steps}`);
  });

  caseTracker.results.forEach(result => console.log(result));

  process.exit(0);
});
