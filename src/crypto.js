// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/000000000008830b
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  headers: [],
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
  addProblemHeader(header) {
    this.headers = [...this.headers, header];
  },
  addProblem(problem) {
    this.problems = [...this.problems, problem];
  },
  addResult(result) {
    this.results = [...this.results, result];
  }
};

const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

// finds the greatest commont denominator between two numbers
function greatestCommonDivisor(a, b) {
  if (b === 0n) {
    return a;
  }
  return greatestCommonDivisor(b, a % b);
}

// finds the index of the next element different than curr, in src, starting at start
const findNextDifferentIndex = (curr, start, src) =>
  src.findIndex((el, index) => el !== curr && index > start);

// reduce over the cyphers
// if array accumulator, prev, is empty, find the
// common pattern with the next element in src, which is different
// than curr, and return factor, common, where factor is curr / common
// if the array acculator, prev, is NOT empty, take the last element on it,
// called last, and divide curr by last, add that to the accumulator and proceed
const decodeCypher = (N, cyphers) => {
  return cyphers.reduce((prev, curr, index, src) => {
    // look behind in prev
    const [last] = prev.slice(-1);
    // if there  is nothing to look behind
    if (!last) {
      // look ahead until a different number happens
      const nextDifferentIndex = findNextDifferentIndex(curr, index, src);
      const nextDifferent = src[nextDifferentIndex];
      const steps = nextDifferentIndex - index;
      const common = greatestCommonDivisor(curr, nextDifferent);
      const factor = curr / common;
      if (steps % 2 === 0) {
        return [...prev, common, factor];
      }
      return [...prev, factor, common];
    }
    // else if there is something to look behind
    // then the next number is the result of curr / last
    // where last would the common factor found at some point
    const factor = curr / last;
    return [...prev, curr / last];
  }, []);
};

const sort = arr =>
  arr
    .reduce(
      (prev, curr) => (prev.indexOf(curr) === -1 ? prev.concat(curr) : prev),
      []
    )
    .sort((a, b) => {
      if (a - b >= 1n) {
        return 1;
      }
      if (b - a >= 1n) {
        return -1;
      }
      return 0;
    });

rl.on("line", function(line) {
  //code goes here
  // if it is the first line, this will turn currentLine to zero
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;
  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    // for this problem the number of expected lines is twice the amount of
    // tests - after the first line
    return caseTracker.setNumberOfLines(parseInt(line) * 2);
  }
  // odd lines
  if (lineNumber && lineNumber % 2 !== 0) {
    caseTracker.addProblemHeader(line);
  }
  // even lines
  if (lineNumber && lineNumber % 2 === 0) {
    caseTracker.addProblem(line);
  }
  // if this is the last line
  if (lineNumber === caseTracker.numberOfLines) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((line, index) => {
    const [N, L] = caseTracker.headers[index].split(" ").map(e => parseInt(e));
    const cyphers = line.split(" ").map(e => BigInt(e));
    const factors = decodeCypher(N, cyphers);
    const lookUpTable = sort(factors);

    const message = factors
      .map(e => {
        const tableIndex = lookUpTable.indexOf(e);
        const letter = alphabet[tableIndex];
        return letter;
      })
      .join("");

    caseTracker.addResult(`Case #${index + 1}: ${message}`);
  });

  caseTracker.results.forEach(result => console.log(result));

  process.exit(0);
});
