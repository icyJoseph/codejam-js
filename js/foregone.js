// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/0000000000088231
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  problems: [],
  results: [],
  numberOfLines: null,
  currentLine: null,
  nextLine() {
    this.currentLine = this.currentLine === null ? 0 : this.currentLine + 1;
  },
  setNumberOfLines(val) {
    this.numberOfTests = val;
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
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;
  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    // expect as many lines as the first line states
    return caseTracker.setNumberOfLines(parseInt(line));
  }

  caseTracker.addProblem(line);

  if (lineNumber === caseTracker.numberOfLines) {
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
