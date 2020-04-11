// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fd27/0000000000209aa0
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

const toInt = (x) => parseInt(x);

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
const IMPOSSIBLE = "IMPOSSIBLE";
const POSSIBLE = "POSSIBLE";

function valid(rows, expected) {
  let k = 0;

  rows.forEach((row, index) => {
    k = k + row[index];
  });

  types[rows.size] = types[rows.size] || {};
  types[rows.size][k] = rows;
  
  return expected === k;
}

const types = {
  2: {
    2: [
      [1, 2],
      [2, 1],
    ],
    4: [
      [2, 1],
      [1, 2],
    ],
  },
  3: {
    3: [
      [1, 2, 3],
      [3, 1, 2],
      [2, 3, 1],
    ],
    6: [
      [2, 3, 1],
      [1, 2, 3],
      [3, 1, 2],
    ],
    9: [
      [3, 2, 1],
      [1, 3, 2],
      [2, 1, 3],
    ],
  },
  5: {
    5: [
      [1, 2, 3, 4, 5],
      [2, 1, 4, 5, 3],
      [3, 5, 1, 2, 4],
      [4, 3, 5, 1, 2],
      [5, 4, 2, 3, 1],
    ],
    7: [
      [1, 2, 3, 4, 5],
      [3, 1, 4, 5, 2],
      [4, 5, 2, 1, 3],
      [5, 3, 1, 2, 4],
      [2, 4, 5, 3, 1],
    ],
    8: [
      [1, 2, 3, 4, 5],
      [2, 1, 4, 5, 3],
      [3, 5, 1, 2, 4],
      [5, 4, 2, 3, 1],
      [4, 3, 5, 1, 2],
    ],
    9: [
      [1, 2, 3, 4, 5],
      [2, 1, 4, 5, 3],
      [4, 5, 1, 3, 2],
      [3, 4, 5, 2, 1],
      [5, 3, 2, 1, 4],
    ],
    10: [
      [1, 2, 3, 4, 5],
      [2, 1, 4, 5, 3],
      [3, 4, 5, 1, 2],
      [5, 3, 1, 2, 4],
      [4, 5, 2, 3, 1],
    ],
    11: [
      [1, 2, 3, 4, 5],
      [2, 3, 1, 5, 4],
      [3, 5, 4, 1, 2],
      [4, 1, 5, 2, 3],
      [5, 4, 2, 3, 1],
    ],
    12: [
      [1, 2, 3, 4, 5],
      [2, 3, 1, 5, 4],
      [3, 4, 5, 1, 2],
      [5, 1, 4, 2, 3],
      [4, 5, 2, 3, 1],
    ],
    13: [
      [1, 2, 3, 4, 5],
      [2, 3, 1, 5, 4],
      [5, 1, 4, 2, 3],
      [4, 5, 2, 3, 1],
      [3, 4, 5, 1, 2],
    ],
    14: [
      [1, 2, 3, 4, 5],
      [2, 1, 4, 5, 3],
      [3, 4, 5, 1, 2],
      [4, 5, 2, 3, 1],
      [5, 3, 1, 2, 4],
    ],
    15: [
      [1, 2, 3, 4, 5],
      [2, 3, 1, 5, 4],
      [3, 4, 5, 1, 2],
      [4, 5, 2, 3, 1],
      [5, 1, 4, 2, 3],
    ],
    16: [
      [1, 2, 3, 4, 5],
      [2, 3, 4, 5, 1],
      [4, 1, 5, 2, 3],
      [5, 4, 1, 3, 2],
      [3, 5, 2, 1, 4],
    ],
    17: [
      [1, 2, 3, 4, 5],
      [2, 4, 1, 5, 3],
      [4, 3, 5, 2, 1],
      [5, 1, 4, 3, 2],
      [3, 5, 2, 1, 4],
    ],
    18: [
      [1, 2, 3, 4, 5],
      [2, 4, 5, 1, 3],
      [3, 5, 4, 2, 1],
      [4, 3, 1, 5, 2],
      [5, 1, 2, 3, 4],
    ],
    19: [
      [1, 2, 3, 4, 5],
      [2, 5, 1, 3, 4],
      [3, 4, 5, 1, 2],
      [4, 3, 2, 5, 1],
      [5, 1, 4, 2, 3],
    ],
    20: [
      [1, 2, 3, 4, 5],
      [2, 5, 4, 1, 3],
      [3, 4, 5, 2, 1],
      [4, 3, 1, 5, 2],
      [5, 1, 2, 3, 4],
    ],
    21: [
      [2, 1, 3, 4, 5],
      [1, 5, 4, 2, 3],
      [3, 4, 5, 1, 2],
      [4, 3, 2, 5, 1],
      [5, 2, 1, 3, 4],
    ],
    22: [
      [3, 1, 2, 4, 5],
      [1, 5, 4, 2, 3],
      [2, 4, 5, 3, 1],
      [4, 3, 1, 5, 2],
      [5, 2, 3, 1, 4],
    ],
    23: [
      [4, 1, 2, 3, 5],
      [1, 5, 3, 4, 2],
      [2, 4, 5, 1, 3],
      [3, 2, 4, 5, 1],
      [5, 3, 1, 2, 4],
    ],
    25: [
      [5, 1, 2, 3, 4],
      [1, 5, 3, 4, 2],
      [2, 4, 5, 1, 3],
      [3, 2, 4, 5, 1],
      [4, 3, 1, 2, 5],
    ],
  },
};

function promising(rows, size) {
  let c = 0;
  let r = 0;

  const matrix = toMatrix(rows, size);

  matrix.forEach((row, index) => {
    if (row.length !== [...new Set(row)].length) {
      r = r + 1;
    }
    const col = matrix.map((row) => row[index]).filter((e) => e);

    if (col.length !== [...new Set(col)].length) {
      c = c + 1;
    }
  });

  if (c !== 0) {
    return false;
  }
  if (r !== 0) {
    return false;
  }
  return true;
}

function toMatrix(rows, size) {
  return Array.from({ length: size }, (_, i) => i).map((index) =>
    rows.slice(index * size, size * (index + 1))
  );
}

function latinSquareCombinator(i, rows, size, stop) {
  if (promising(rows, size)) {
    if (rows.length === size * size) {
      const matrix = toMatrix(rows, size);
      if (valid(matrix, stop)) {
        throw matrix;
      }
    } else {
      Array.from({ length: size }, (_, j) => j + 1).forEach((val) => {
        rows[i + 1] = val;
        latinSquareCombinator(i + 1, [...rows], size, stop);
      });
    }
  }
}

function findLatinSquare({ N, K }) {
  if (types[N] && types[N][K]) {
    return [POSSIBLE, types[N][K]];
  }

  let square;

  if (N === 5 && [6, 24].includes(K)) {
    return [IMPOSSIBLE];
  }
  try {
    latinSquareCombinator(-1, [], N, K);
  } catch (e) {
    square = e;
  } finally {
    if (square) {
      return [POSSIBLE, square];
    }
    return [IMPOSSIBLE];
  }
}

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
      const [N, K] = line.split(" ").map(toInt);

      const [result, rows] = findLatinSquare({ K, N });

      caseTracker.addResult(`Case #${index + 1}: ${result}`);
      if (result === POSSIBLE) {
        rows.forEach((row) => {
          caseTracker.addResult(row.join(" "));
        });
      }
    });

    caseTracker.results.forEach((result) => console.log(result));
  } catch (e) {
    console.log(e);
  }

  process.exit(0);
});
