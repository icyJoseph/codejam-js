// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fef2/00000000002d5b62
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

const isCoordinateOdd = ({ x, y }) => (x + y) % 2 !== 0;

const scale = (num) => num / 2;

function moveTowardsOdd({ x, y }) {
  if (x === 1 && y === 0) {
    // can be finished in one move by moving EAST
    return { step: EAST, nextX: scale(x - 1), nextY: y };
  }

  if (x === -1 && y === 0) {
    // can be finished in one move by moving WEST
    return { step: WEST, nextX: scale(x + 1), nextY: y };
  }

  if (x === 0 && y === 1) {
    // can be finished in one move by moving NORTH
    return { step: NORTH, nextX: x, nextY: scale(y - 1) };
  }

  if (x === 0 && y === -1) {
    // can be finished in one move by moving SOUTH
    return { step: SOUTH, nextX: x, nextY: scale(y + 1) };
  }

  if (x % 2 !== 0) {
    // move EAST
    const wouldBeOdd = isCoordinateOdd({ x: scale(x - 1), y: scale(y) });
    if (wouldBeOdd) {
      return { step: EAST, nextX: scale(x - 1), nextY: scale(y) };
    }
    // better to move WEST
    return { step: WEST, nextX: scale(x + 1), nextY: scale(y) };
  }

  if (y % 2 !== 0) {
    // move NORTH
    const wouldBeOdd = isCoordinateOdd({ x: scale(x), y: scale(y - 1) });
    if (wouldBeOdd) {
      return { step: NORTH, nextX: scale(x), nextY: scale(y - 1) };
    }
    // better to move SOUTH
    return { step: SOUTH, nextX: scale(x), nextY: scale(y + 1) };
  }
}

// There's only one change to make either x or y odd
// and that's the first move
// after that one of them will be even.
// The way to check is by making sure that
// given x, and y, x + y must be odd
// two odd numbers added, result in even
// two even numbers added, result in even
// only even + odd numbers, result in odd
const moveExpogo = ({ x, y, steps = "" }) => {
  if (x === 0 && y === 0) {
    return steps;
  }
  const isOddCoordinate = isCoordinateOdd({ x, y });

  if (!isOddCoordinate) {
    return IMPOSSIBLE;
  }

  // now make the distance to the odd coordinate
  // even, by taking a one unit step
  const { step, nextX, nextY } = moveTowardsOdd({ x, y });
  return moveExpogo({ x: nextX, y: nextY, steps: `${steps}${step}` });
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
    try {
      const [x, y] = line.split(" ").map((x) => parseInt(x));
      const directions = moveExpogo({ x, y });
      caseTracker.addResult(`Case #${index + 1}: ${directions}`);
    } catch (e) {
      console.log(e);
    }
  });

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
