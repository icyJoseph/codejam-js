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

function rules(a, b, strict = false) {
  if (!strict) {
    if (a === b) {
      return true;
    }
  }
  // our robot vs their robot
  const combined = `${a}${b}`;
  switch (combined) {
    case "PR":
    case "RS":
    case "SP":
      return true;
    default:
      return false;
  }
}

const transpose = matrix =>
  matrix.reduceRight(
    (acc, row) =>
      row.map((curr, i) => {
        const prev = acc[i] || [];
        return prev ? [curr, ...prev] : [curr];
      }),
    []
  );

const unique = arr =>
  arr.reduce(
    (prev, curr) => (prev.includes(curr) ? prev : [...prev, curr]),
    []
  );

const beatsAll = (ourMove, theirMoves = []) => {
  if (theirMoves.length === 0) {
    return true;
  }
  if (theirMoves.length === 1) {
    return theirMoves.every(adMove => rules(ourMove, adMove, true));
  }
  return theirMoves.every(adMove => rules(ourMove, adMove));
};

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
    const { result } = Array.from({
      length: Math.max(...programs.map(p => p.split("").length))
    }).reduce(
      (prev, _, index) => {
        if (prev.programs.length === 0) {
          return prev;
        }
        const current = transpose(prev.programs.map(p => p.split("")))[index];
        const uniqueMoves = unique(current || []);
        const choice = ["R", "P", "S"].filter(ourMove =>
          beatsAll(ourMove, uniqueMoves)
        );

        // keeps those which won't be out of scope
        const leftPrograms = prev.programs.filter(
          p => !rules(choice, p.split("")[index], true)
        );

        return { programs: leftPrograms, result: [...prev.result, choice] };
      },
      { result: [], programs }
    );

    const [program = "IMPOSSIBLE"] = transpose(result).map(a => a.join(""));

    const notUnique = programs.includes(program);
    if (notUnique) {
      const first = program.slice(0, 1);
      const [append] = ["R", "P", "S"].filter(ourMove =>
        beatsAll(ourMove, [first])
      );
      const finalProgram = `${program}${append}`;
      return caseTracker.addResult(`Case #${index + 1}: ${finalProgram}`);
    }

    return caseTracker.addResult(`Case #${index + 1}: ${program}`);
  });

  caseTracker.results.forEach(result => console.log(result));

  process.exit(0);
});
