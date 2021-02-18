// Problem: https://codingcompetitions.withgoogle.com/codejam/round/00000000000000cb/0000000000007966
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  problems: [],
  problemHeaders: [],
  results: [],
  readingProblem: false,
  numberOfLines: null,
  currentLine: null,
  nextLine() {
    this.currentLine = this.currentLine === null ? 0 : this.currentLine + 1;
  },
  setNumberOfLines(val) {
    this.numberOfLines = val;
  },
  addProblemHeader(head) {
    this.problemHeaders = [...this.problemHeaders, head];
    this.problems = [...this.problems, []];
  },
  addProblem(problem) {
    const [currentHeader] = this.problemHeaders.slice(-1);
    const [lastProblem] = this.problems.slice(-1);
    const updated = [...lastProblem, problem];
    const len = this.problems.length;
    this.problems = [...this.problems.slice(0, len - 1), updated];
    if (updated.length === currentHeader) {
      this.setReadingProblem(false);
    }
  },
  addResult(result) {
    this.results = [...this.results, result];
  },
  setReadingProblem(val) {
    this.readingProblem = !!val;
  }
};

function bestBetweenTwo(a, b) {
  const combined = `${a}${b}`;
  switch (combined) {
    case "PS":
    case "SP":
      return "S";
    case "PR":
    case "RP":
      return "P";
    case "SR":
    case "RS":
      return "R";
    default:
      return false;
  }
}

function bestResponse(move) {
  switch (move) {
    case "S":
      return "R";
    case "P":
      return "S";
    case "R":
      return "P";
    default:
      return move;
  }
}

const unique = arr =>
  arr.reduce(
    (prev, curr) => (prev.includes(curr) ? prev : [...prev, curr]),
    []
  );

rl.on("line", function(line) {
  //code goes here
  if (!caseTracker.readingProblem) {
    caseTracker.nextLine();
    const lineNumber = caseTracker.currentLine;

    // for the first line, which specifies the number of cases
    if (!lineNumber) {
      return caseTracker.setNumberOfLines(parseInt(line));
    }
    caseTracker.setReadingProblem(true);
    // sets A, adversaries robots
    return caseTracker.addProblemHeader(parseInt(line));
  }
  // sets a robot problem
  return caseTracker.addProblem(line);

  if (lineNumber === caseTracker.numberOfLines) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((programs, index) => {
    const robots = caseTracker.problemHeaders[index];
    const longest = Math.max(...programs.map(p => p.split("").length));
    // ensure that all programs will last the same
    const withPadding = programs.map(program =>
      program.repeat(Math.ceil(longest / program.length)).slice(0, longest)
    );
    // walk the length of the longest program
    const { result = [] } = Array.from({
      length: longest
    }).reduce(
      (prev, _, index) => {
        if (!prev.result || prev.done) {
          return prev;
        }
        const allMoves = prev.programs
          .map(p => p.split("")[index])
          .filter(e => e);

        const uniqueMoves = unique(allMoves);
        const uniqueMovesQty = uniqueMoves.length;
        switch (uniqueMovesQty) {
          case 3:
            // worst case, we cannot ever win
            return { ...prev, result: undefined };
          case 2:
            // we can choose the best move and tie with some robots
            // while beating others
            const nextMove = bestBetweenTwo(...uniqueMoves);
            return {
              ...prev,
              programs: prev.programs.filter(program => {
                const theirMove = program.split("")[index];
                return theirMove === nextMove;
              }),
              result: [...prev.result, nextMove]
            };
          case 1:
            // best case, because we can beat everyone at this point
            return {
              ...prev,
              done: true,
              result: [...prev.result, bestResponse(...uniqueMoves)]
            };
          default:
            return prev;
        }
      },
      { result: [], programs: withPadding, done: false }
    );

    const program = result.join("") || "IMPOSSIBLE";

    if (programs.includes(program)) {
      const append = bestResponse(...program.split("").slice(0));
      return caseTracker.addResult(`Case #${index + 1}: ${program}${append}`);
    }
    return caseTracker.addResult(`Case #${index + 1}: ${program}`);
  });

  caseTracker.results.forEach(result => console.log(result));

  process.exit(0);
});
