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

const isCoordinateOdd = ({ x, y }) => {
  return [x, y].reduce((acc, curr) => Math.abs(curr) + acc, 0) % 2 !== 0;
};

const canBeFinishedInOppositeDirection = ({ x, y, direction }) => {
  if (direction === NORTH) {
    return x === 0 && y + 1 === 0;
  } else if (direction === EAST) {
    return y === 0 && x + 1 === 0;
  }
};

const hasFinished = ({ x, y }) => x === 0 && y === 0;

const scale = (num) => num / 2;

function moveTowardsOdd({ x, y }) {
  if (Math.abs(x) % 2 !== 0) {
    // try to move EAST
    let pivotX = x - 1;
    if (
      (isCoordinateOdd({ x: scale(pivotX), y: scale(y) }) &&
        !canBeFinishedInOppositeDirection({
          direction: EAST,
          x: scale(pivotX),
          y: scale(y)
        })) ||
      hasFinished({ x: scale(pivotX), y: scale(y) })
    ) {
      return { step: EAST, nextX: scale(pivotX), nextY: scale(y) };
    } else {
      // failed to move EAST, move WEST
      pivotX = x + 1;
      return { step: WEST, nextX: scale(pivotX), nextY: scale(y) };
    }
  } else if (Math.abs(y) % 2 !== 0) {
    // try to move NORTH
    let pivotY = y - 1;
    if (
      (isCoordinateOdd({ x: scale(x), y: scale(pivotY) }) &&
        !canBeFinishedInOppositeDirection({
          direction: NORTH,
          x: scale(x),
          y: scale(pivotY)
        })) ||
      hasFinished({ x: scale(x), y: scale(pivotY) })
    ) {
      return { step: NORTH, nextX: scale(x), nextY: scale(pivotY) };
    } else {
      // failed to move NORTH, move SOUTH
      pivotY = y + 1;
      return { step: SOUTH, nextX: scale(x), nextY: scale(pivotY) };
    }
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
  steps = `${steps}${step}`;
  return moveExpogo({ x: nextX, y: nextY, steps });
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
