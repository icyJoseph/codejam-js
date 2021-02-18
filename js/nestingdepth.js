// Problem: https://codingcompetitions.withgoogle.com/codejam/round/00000000000000cb/0000000000007966
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
    this.numberOfLines = val;
  },
  addProblem(problem) {
    this.problems = [...this.problems, problem];
  },
  addResult(result) {
    this.results = [...this.results, result];
  },
};

const left = "(";
const right = ")";

const solveFor = (input) => {
  const padStart = "0";
  const padEnd = "0";
  const padded = [padStart, ...input, padEnd];

  const paddedResult = padded.reduce((acc, curr, index, src) => {
    const next = src[index + 1] || padEnd;

    const nextDelta = parseInt(curr) - parseInt(next);

    const repeats = Math.abs(nextDelta);
    if (nextDelta < 0) {
      return `${acc}${curr}${left.repeat(repeats)}`;
    } else if (nextDelta > 0) {
      return `${acc}${curr}${right.repeat(repeats)}`;
    } else {
      return `${acc}${curr}`;
    }
  }, "");
  const result = paddedResult.slice(1, paddedResult.length - 1);
  return result;
};

rl.on("line", function (line) {
  //code goes here
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;

  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    return caseTracker.setNumberOfLines(parseInt(line));
  }

  caseTracker.addProblem(line);

  if (lineNumber === caseTracker.numberOfLines) {
    return rl.close();
  }
}).on("close", function () {
  caseTracker.problems.forEach((line, index) => {
    const input = line.split("");

    const output = solveFor(input);

    caseTracker.addResult(`Case #${index + 1}: ${output}`);
  });

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
