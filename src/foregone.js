// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/0000000000088231
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

const testAnswer = (target, a, b) => {
  if (a.indexOf("4") === -1 || b.indexOf("4") === -1) {
    return parseInt(target) === parseInt(a) + parseInt(b);
  }
  return false;
};

const addZeroIfNotEmpty = b => (b ? `${b}0` : b);
const breakDown = target => {
  const firstFourIndex = target.indexOf("4");
  if (firstFourIndex === -1) {
    return { a: 0, b: target }; // should never happen
  }
  return target.split("").reduce(
    (prev, curr) => {
      if (curr === "4") {
        return { b: `${prev.b}1`, a: `${prev.a}3` };
      } else {
        return { b: addZeroIfNotEmpty(prev.b), a: `${prev.a}${curr}` };
      }
    },
    { a: "", b: "" }
  );
};

rl.on("line", function(line) {
  //code goes here
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
    const { a, b } = breakDown(line);
    caseTracker.addResult(`Case #${index + 1}: ${a} ${b}`);
  });

  caseTracker.results.forEach(result => console.log(result));

  process.exit(0);
});
