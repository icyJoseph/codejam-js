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
  add(result) {
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

function interlaceOddEven(even, odd, step = 0) {
  const [firstEven, secondEven, ...restEven] = even;
  const [firstOdd, ...restOdd] = odd;

  if (firstOdd < firstEven) {
    return step;
  }

  if (firstOdd > secondEven) {
    return step + 1;
  }

  if (restOdd.length === 0 || !secondEven) {
    return "OK";
  }

  return interlaceOddEven([secondEven, ...restEven], restOdd, step + 2);
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

    caseTracker.add(`Case #${index + 1}: ${result}`);
  });

  caseTracker.results.forEach(res => console.log(res));
  process.exit(0);
});
