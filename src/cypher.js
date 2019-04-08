const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  headers: [],
  problems: [],
  results: [],
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

const alphabet = [
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z"
];

const isPrime = num => {
  for (let i = BigInt(2); i * i <= num; i++)
    if (num % i === BigInt(0)) return false;
  return num > 1;
};

const sieve = bigMax => {
  let f0, f1;
  for (let i = BigInt(2); i * i <= bigMax; i++) {
    if (bigMax % i === BigInt(0)) {
      if (isPrime(i)) {
        f0 = i;
        f1 = bigMax / i;
        if (isPrime(f1)) {
          break;
        }
      }
    }
  }
  return [f0, f1];
};

const head = ([head]) => head;
const last = arr => head(arr.slice(-1));
// check if they have the same elements in the same position
const sameOrder = ([a, b], [c, d]) => a === c && b === d;

const compare = (curr, ahead, behind, direction) => {
  const [a, b] = curr;
  // console.log("input", curr, behind, ahead, direction);
  if (!!behind) {
    const [first, second] = behind;
    if (direction === "right") {
      // we would want [first, second], [a,b], where a === second;
      const ret = first === a ? [b, a] : [a, b];
      return ret;
    }
    if (direction === "left") {
      const ret = second === a ? [a, b] : [b, a];
      return ret;
    }
  } else {
    const [first, second] = ahead;
    if (direction === "right") {
      // we would want [first, second], [a,b], where a === second;
      const ret = a === second ? curr : [b, a];
      return ret;
    }
    if (direction === "left") {
      // we would want [b,a], [first, second], where a === first
      const ret = b === first ? curr : [a, b];
      return ret;
    }
  }
};

const walker = direction => (prev, curr, index, src) => {
  // start from the right to left => right
  // start from left to right => left
  const inc = direction === "right" ? -1 : +1;
  const ahead = src[index + inc];
  const behind = direction === "right" ? head(prev) : last(prev);
  const result = compare(curr, ahead, behind, direction);
  return direction === "right" ? [result, ...prev] : [...prev, result];
};

const decodeCypher = (N, cyphers) => {
  return cyphers
    .reduce((prev, curr) => {
      const factors = sieve(BigInt(curr));
      return prev.concat([factors]);
    }, [])
    .reduceRight(walker("right"), [])
    .reduce(walker("left"), [])
    .reduce((factors, curr, index, src) => {
      if (src.length - 1 === index) {
        return [...factors, ...curr];
      }
      const [next] = curr;
      return [...factors, next];
    }, []);
};

const order = arr =>
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
  const caseNumber = caseTracker.get();
  // for the first line, which specifies the number of cases
  if (!caseNumber) {
    return caseTracker.set(parseInt(line) * 2);
  }
  if (caseNumber && caseNumber % 2 !== 0) {
    caseTracker.addProblemHeader(line);
  }
  if (caseNumber && caseNumber % 2 === 0) {
    caseTracker.addProblem(line);
  }
  if (caseNumber === caseTracker.numberOfTests) {
    return rl.close();
  }
}).on("close", function() {
  caseTracker.problems.forEach((line, index) => {
    const [N, L] = caseTracker.headers[index].split(" ").map(e => parseInt(e));
    const cyphers = line.split(" ").map(e => parseInt(e));

    const broken = decodeCypher(N, cyphers);

    const lookUpTable = order(broken);

    const message = broken
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
