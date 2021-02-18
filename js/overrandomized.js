// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fef4/00000000003179a1
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  problem: [],
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
  addToCurrentProblem(line) {
    return this.problem.push(line);
  },
  addProblem() {
    this.problems.push([...this.problem]);
    this.problem = [];
  },
  addResult(result) {
    this.results = [...this.results, result];
  }
};

const digits = Array.from({ length: 10 }, (_, i) => 9 - i);

const flattened = (arr) => [].concat(...arr);

const findZero = (list) => {
  const firstDigits = [
    ...new Set(
      list.map(([, response]) => {
        const [firstDigit] = response.split("");
        return firstDigit;
      })
    )
  ];

  const allDigits = [
    ...new Set(flattened(list.map(([, response]) => response.split(""))))
  ];

  const [zero = null] = allDigits.filter(
    (digit) => !firstDigits.includes(digit)
  );
  return zero;
};

const solve = (line) => {
  const indexDictionary = [];
  const [Upper, ...rest] = line;
  const Max = Math.pow(10, parseInt(Upper)) - 1;

  const uniqueQueries = [...new Set(rest)];

  const querieResponseList = uniqueQueries.map((qr) => {
    const [qS, res] = qr.split(" ");
    return [parseInt(qS), res];
  });

  const zero = findZero(querieResponseList);

  if (zero) {
    indexDictionary[0] = zero;
  }

  const sorted = querieResponseList.slice(0).sort(([qA], [qB]) => qA - qB);

  const inverse = sorted.reduce((prev, [query, response]) => {
    prev[query] = prev[query] || [];

    if (query.toString().length === response.length) {
      prev[query].push(response);
    }
    prev[query].sort((a, b) => a - b);
    return prev;
  }, {});

  while (1) {
    Object.keys(inverse).forEach((key) => {
      const values = inverse[key];
      if (values.length === 1) {
        indexDictionary[key] = values[0];
      } else {
        inverse[key] = values.filter((val) => !indexDictionary.includes(val));
      }
    });

    const done =
      indexDictionary.slice(0, 10).filter((x) => x).length ===
      indexDictionary.slice(0, 10).length;

    if (done) {
      break;
    }
  }

  return indexDictionary.slice(0, 10).join("");
};

rl.on("line", function (line) {
  //code goes here
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;

  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    return caseTracker.setNumberOfLines(parseInt(line) * (1 + Math.pow(10, 4)));
  }

  const length = caseTracker.addToCurrentProblem(line);

  if (length === Math.pow(10, 4) + 1) {
    caseTracker.addProblem();
  }

  if (lineNumber === caseTracker.numberOfLines) {
    return rl.close();
  }
}).on("close", function () {
  caseTracker.problems.forEach((line, index) => {
    const result = solve(line);

    caseTracker.addResult(`Case #${index + 1}: ${result}`);
  });

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
