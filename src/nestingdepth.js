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
  const { result, nesting } = input.reduce(
    (prev, curr, index, src) => {
      if (parseInt(curr) > prev.nesting) {
        return {
          result: `${prev.result}${left}${curr}`,
          nesting: prev.nesting + 1,
        };
      } else if (parseInt(curr) < prev.nesting) {
        return {
          result: `${prev.result}${right}${curr}`,
          nesting: prev.nesting - 1,
        };
      } else {
        return {
          ...prev,
          result: `${prev.result}${curr}`,
        };
      }
    },
    { result: "", nesting: 0 }
  );
  return `${result}${right.repeat(nesting)}`;
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
  try {
    caseTracker.problems.forEach((line, index) => {
      const input = line.split("");

      const output = solveFor(input);

      caseTracker.addResult(`Case #${index + 1}: ${output}`);
    });
  } catch (e) {
    console.log(e);
  }

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
