const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  numberOfTests: null,
  count: null,
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
  }
};

function oddEvenSplit(arr) {
  return arr.reduce(
    ({ even, odd }, curr, index) => {
      return index % 2 === 0
        ? { odd, even: [...even, curr] }
        : { even, odd: [...odd, curr] };
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

function shouldEnd(caseNumber, rl) {
  caseTracker.numberOfTests === caseNumber ? rl.close() : null;
}

rl.on("line", function(line) {
  const lineNumber = caseTracker.get();

  if (!lineNumber) {
    return caseTracker.set(parseInt(line));
  }

  if (lineNumber % 2 === 0) {
    const numbers = line.split(" ").map(num => parseInt(num));

    const { even, odd } = oddEvenSplit(numbers);

    const sortedEven = [...even].sort((a, b) => a - b);
    const sortedOdd = [...odd].sort((a, b) => a - b);

    const result = interlaceOddEven(sortedEven, sortedOdd);

    const caseNumber = parseInt(lineNumber / 2);

    return (
      console.log(`Case #${caseNumber}: ${result}`) || shouldEnd(caseNumber, rl)
    );
  }
}).on("close", function() {
  process.exit(0);
});
