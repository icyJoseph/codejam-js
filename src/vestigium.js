// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fd27/000000000020993c
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

const toInt = (x) => parseInt(x);

// keeps track of the test case being solved
const caseTracker = {
  headers: [],
  matrix: [],
  matrices: [],
  results: [],
  numberOfCases: null,
  nextMatrix: null,
  currentLine: null,
  nextLine() {
    this.currentLine = this.currentLine === null ? 0 : this.currentLine + 1;
  },
  setNumberOfCases(val) {
    this.numberOfCases = val;
  },
  setNextMatrix(val) {
    this.nextMatrix = val;
  },
  addToMatrix(row) {
    this.matrix = [...this.matrix, row];
  },
  addMatrix() {
    if (this.matrix.length) {
      this.matrices = [...this.matrices, this.matrix];
      this.matrix = [];
    }
  },
  addResult(result) {
    this.results = [...this.results, result];
  },
};

const walkDiagonal = (rows) => {
  let k = 0;
  let r = 0;
  let c = 0;

  rows.forEach((row, index) => {
    k = k + row[index];
    if (row.length !== [...new Set(row)].length) {
      r = r + 1;
    }
    const col = rows.map((row) => row[index]);
    if (col.length !== [...new Set(col)].length) {
      c = c + 1;
    }
  });

  return [k, r, c];
};

rl.on("line", function (line) {
  //code goes here
  // if it is the first line, this will turn currentLine to zero
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;

  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    caseTracker.setNextMatrix(1);
    return caseTracker.setNumberOfCases(parseInt(line));
  }
  // matrix sizes
  if (lineNumber && caseTracker.nextMatrix === lineNumber) {
    caseTracker.setNextMatrix(caseTracker.nextMatrix + parseInt(line) + 1);
  }

  if (lineNumber && lineNumber < caseTracker.nextMatrix) {
    caseTracker.addToMatrix(line);
    if (lineNumber === caseTracker.nextMatrix - 1) {
      caseTracker.addMatrix();
    }
  }
  // if this is the last line
  if (caseTracker.numberOfCases === caseTracker.matrices.length) {
    return rl.close();
  }
}).on("close", function () {
  caseTracker.matrices.forEach((line, index) => {
    const [_, ...stringRows] = line;
    const rows = stringRows.map((row) => row.split(" ").map(toInt));
    // walk the diagonal
    const [k, r, c] = walkDiagonal(rows);

    caseTracker.addResult(`Case #${index + 1}: ${k} ${r} ${c}`);
  });

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
