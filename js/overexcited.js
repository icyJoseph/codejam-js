// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fef4/0000000000317409
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
  }
};

const [NORTH, SOUTH, WEST, EAST, IMPOSSIBLE] = [
  "N",
  "S",
  "W",
  "E",
  "IMPOSSIBLE"
];

const move = ({ x, y, direction }) => {
  switch (direction) {
    case NORTH:
      return { x, y: y + 1 };
    case SOUTH:
      return { x, y: y - 1 };
    case EAST:
      return { x: x + 1, y };
    case WEST:
      return { x: x - 1, y };
    default:
      throw new Error(`Invalid direction, ${direction}`);
  }
};

const distance2Origin = ({ x, y }) => Math.abs(x) + Math.abs(y);

const solve = ({ x, y, directions }) => {
  const snapshots = directions.split("").reduce(
    (prev, direction, index) => {
      const [{ x, y }] = prev.slice(-1);
      const next = move({ x, y, direction });
      return prev.concat({
        ...next,
        time: index + 1,
        distance: distance2Origin(next)
      });
    },
    [{ x, y, time: 0, distance: distance2Origin({ x, y }) }]
  );

  const [smallest = null] = snapshots
    .filter(({ time, distance }) => distance <= time)
    .map(({ time }) => time)
    .sort((a, b) => a - b);

  if (smallest === null) {
    return IMPOSSIBLE;
  }
  return smallest;
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
    const [xS, yS, directions] = line.split(" ");
    const [x, y] = [xS, yS].map((x) => parseInt(x));

    const result = solve({ x, y, directions });

    caseTracker.addResult(`Case #${index + 1}: ${result}`);
  });

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
