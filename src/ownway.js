// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/00000000000881da
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
  setNumerOfLines(val) {
    this.numberOfLines = val;
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
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;
  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    // for this problem we expect twice as many lines
    return caseTracker.setNumerOfLines(parseInt(line) * 2);
  }
  // only read the even lines
  if (lineNumber && lineNumber % 2 === 0) {
    caseTracker.addProblem(line);
  }

  if (lineNumber === caseTracker.numberOfLines) {
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
