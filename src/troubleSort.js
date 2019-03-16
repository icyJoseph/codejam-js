const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  numberOfTests: null,
  count: null,
  problems: [],
  results: [],
  inc() {
    this.count = this.count === null ? 1 : this.count + 1;
  },
  set(val) {
    this.numberOfTests = val;
  },
  get() {
    const current = this.count;
    this.inc();
    return current;
  },
  addProblem(problem) {
    this.problems = [...this.problems, problem];
  },
  addResult(result) {
    this.results = [...this.results, result];
  }
};

function oddEvenSplit(arr) {
  return arr.reduce(
    ({ even, odd }, curr, index) => {
      return index % 2 === 0
        ? { odd, even: [...even, parseInt(curr)] }
        : { even, odd: [...odd, parseInt(curr)] };
    },
    { even: [], odd: [] }
  );
}

function getIndex(arr, index, fallback) {
  return arr[index] !== undefined ? arr[index] : fallback;
}

function interlaceOddEven(even, odd, index = 0) {
  if (index >= even.length) {
    return "OK";
  }

  const first = getIndex(even, index);
  const middle = getIndex(odd, index, first);
  const last = getIndex(even, index + 1, middle || first);

  if (first > middle) {
    return 2 * index;
  }
  if (middle > last) {
    return 2 * index + 1;
  }

  return interlaceOddEven(even, odd, index + 1);
}

function sort(arr) {
  return [...arr].sort((a, b) => a - b);
}

rl.on("line", function(line) {
  const lineNumber = caseTracker.get();

  if (!lineNumber) {
    caseTracker.set(parseInt(line));
  }

  if (lineNumber && lineNumber % 2 === 0) {
    caseTracker.addProblem(line);
  }

  if (lineNumber / 2 === caseTracker.numberOfTests) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((line, index) => {
    const numbers = line.split(" ");

    const { even, odd } = oddEvenSplit(numbers);

    const sortedEven = sort(even);
    const sortedOdd = sort(odd);

    const result = interlaceOddEven(sortedEven, sortedOdd);

    caseTracker.addResult(`Case #${index + 1}: ${result}`);
  });

  caseTracker.results.forEach(res => console.log(res));
  process.exit(0);
});
