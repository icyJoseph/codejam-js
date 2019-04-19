// Problem: https://codingcompetitions.withgoogle.com/codejam/round/0000000000051705/00000000000881de
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  currentCase: [],
  input: [],
  saved: [],
  output: [],
  numberOfTests: null,
  testNumber: null,
  solving: false,
  inc() {
    this.testNumber = this.testNumber === null ? 0 : this.testNumber + 1;
  },
  set(val) {
    this.numberOfTests = val;
  },
  get() {
    this.inc();
    return this.testNumber;
  },
  setSolving(state) {
    this.solving = state;
  },
  isSolving() {
    return this.solving;
  },
  setCase(N, B, F) {
    this.currentCase = [N, B, F];
  },
  getCase() {
    return this.currentCase;
  },
  setInput(input) {
    this.input = input;
    this.saved = input;
  },
  getInput() {
    return this.input;
  },
  appendOutput(output) {
    this.output = [...this.output, output.split("")];
  },
  printNext() {
    const [next, ...rest] = this.input;
    this.input = rest;
    return console.log(next.join(""));
  },
  printFailing() {
    const submittedNumbers = transpose(this.saved).map(
      (e, i) => 16 * Math.floor(i / 16) + parseInt(e.join(""), 2)
    );
    const { returnedNumbers } = transpose(this.output).reduce(
      (prev, curr, index, src) => {
        const value = parseInt(curr.join(""), 2);
        const previous = src[index - 1] || [];
        const currentBase =
          parseInt(previous.join(""), 2) >= value ? prev.base + 1 : prev.base;
        const numeric = currentBase * 16 + value;

        return {
          returnedNumbers: [...prev.returnedNumbers, numeric],
          base: currentBase
        };
      },
      { returnedNumbers: [], base: 0 }
    );

    const missing = submittedNumbers.reduce(
      (acc, num, index) =>
        returnedNumbers.includes(num) ? acc : [...acc, index],
      []
    );

    return console.log(missing.join(" "));
  },
  clearAll() {
    this.currentCase = [];
    this.input = [];
    this.saved = [];
    this.output = [];
  }
};

const SUCCESS = "1";
const FAILURE = "-1";
const ZERO = "0";
const bits = 4;

const transpose = matrix =>
  matrix.reduceRight(
    (acc, row) =>
      row.map((curr, i) => {
        const prev = acc[i];
        return prev ? [curr, ...prev] : [curr];
      }),
    []
  );

const generateInput = N => {
  // get binary representation of numbers from 0 to 15
  // repeat as many times as needed to match N - 1 rows
  // do matrix transpose
  const length = Math.pow(2, bits);

  const base = Array.from({ length }, (_, i) => i).map(num => {
    const body = num.toString(2);
    const head = ZERO.repeat(bits - body.length);
    return `${head}${body}`.split("");
  });

  const repeated = Array.from({ length: N }, (_, i) => base[i % length]);

  return transpose(repeated);
};

rl.on("line", function(line) {
  if (!caseTracker.isSolving()) {
    if (line === FAILURE) {
      // if result is wrong exit
      return rl.close();
    }
    // getting a single 1 while not solving
    if (line === SUCCESS && caseTracker.numberOfTests) {
      // clear and get ready for a new problem
      if (caseTracker.numberOfTests === caseTracker.testNumber) {
        return rl.close();
      }
      return caseTracker.clearAll();
    }
    // read the number of test cases
    const current = caseTracker.get();
    if (!current) {
      // if current is falsy, then we are at the beginning
      return caseTracker.set(parseInt(line));
    }
    // if not at the beginning
    // and if we are not solving
    // read the test case
    const [N, B, F] = line.split(" ").map(e => parseInt(e));
    // add the test case
    caseTracker.setCase(N, B, F);
    // set to solving
    caseTracker.setSolving(true);
    // generate a test string
    const testInput = generateInput(N);
    // add it to our
    caseTracker.setInput(testInput);
    // print the starting string
    return caseTracker.printNext();
  } else if (caseTracker.isSolving()) {
    // read result
    caseTracker.appendOutput(line);

    if (caseTracker.getInput().length === 0) {
      // no more input to print
      caseTracker.setSolving(false);
      // print the failing workers
      return caseTracker.printFailing();
    }
    // print next line
    return caseTracker.printNext();
  }
}).on("close", () => process.exit(0));
