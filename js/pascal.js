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
  }
};

function pascalTriangle(row, index) {
  return index > row
    ? false
    : index === 1 || index === row
    ? 1
    : pascalTriangle(row - 1, index - 1) + pascalTriangle(row - 1, index);
}

const move = (position, direction) => {
  const [r, k] = position;
  switch (direction) {
    case "UL":
      return [r - 1 < 1 ? 1 : r - 1, k - 1 < 1 ? 1 : k - 1];
    case "UR":
      return [r - 1 < 1 ? 1 : r - 1, k + 1];
    case "L":
      return [r, k - 1 < 1 ? 1 : k - 1];
    case "R":
      return [r, k + 1 > r ? r : k + 1];
    case "DL":
      return [r + 1, k - 1 < 1 ? 1 : k - 1];
    case "DR":
      return [r + 1, k + 1];
    default:
      throw new Error("Invalid move");
  }
};

const addToPath = (path, pos) => {
  const [r, k] = pos;
  path.push(`${r} ${k}`);
  return path;
};

const findPath = (target) => {
  if (target < 5) {
    let s = 1;
    let path = ["1 1"];

    let position = move([1, 1], "DL");
    let nodeVal = pascalTriangle(...position);
    path = addToPath(path, position);
    s = s + nodeVal;

    position = move(position, "R");
    nodeVal = pascalTriangle(...position);
    path = addToPath(path, position);
    s = s + nodeVal;

    while (s !== target) {
      position = move(position, "DR");
      path = addToPath(path, position);
      s = s + pascalTriangle(...position);
    }
    return path;
  } else {
    let s = 1;
    let path = ["1 1"];

    let position = move([1, 1], "DL");
    let nodeVal = pascalTriangle(...position);
    path = addToPath(path, position);
    s = s + nodeVal;

    position = move(position, "DR");
    nodeVal = pascalTriangle(...position);
    path = addToPath(path, position);
    s = s + nodeVal;

    position = move(position, "R");
    nodeVal = pascalTriangle(...position);
    path = addToPath(path, position);
    s = s + nodeVal;

    while (s !== target) {
      position = move(position, "DR");
      path = addToPath(path, position);
      s = s + pascalTriangle(...position);
    }
    return path;
  }
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
    const max = findPath(501);
    caseTracker.problems.forEach((line, index) => {
      const result = max.slice(0, parseInt(line));
    //   const target = result.reduce(
    //     (prev, curr) =>
    //       prev + pascalTriangle(...curr.split(" ").map((x) => parseInt(x))),
    //     0
    //   );

      caseTracker.addResult(`Case #${index + 1}:\n${result.join("\n")}`);
    });

    caseTracker.results.forEach((result) => console.log(result));
  } catch (e) {
    console.log(e);
  }

  process.exit(0);
});
