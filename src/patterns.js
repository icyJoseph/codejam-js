// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fd74/00000000002b3034
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

// keeps track of the test case being solved
const caseTracker = {
  current: [],
  cases: [],
  results: [],
  numberOfCases: null,
  nextCase: null,
  currentLine: null,
  nextLine() {
    this.currentLine = this.currentLine === null ? 0 : this.currentLine + 1;
  },
  setNumberOfCases(val) {
    this.numberOfCases = val;
  },
  setNextCase(val) {
    this.nextCase = val;
  },
  addToCurrent(activity) {
    this.current = [...this.current, activity];
  },
  addCase() {
    if (this.current.length) {
      this.cases = [...this.cases, this.current];
      this.current = [];
    }
  },
  addResult(result) {
    this.results = [...this.results, result];
  }
};

const MAX_LETTERS = 10 * 10 * 10 * 10;
const PATTERN = "*";
const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const rightMostName = (pattern) => {
  const [, ...rightMost] = pattern.split("*");
  return rightMost.join("");
};

const leftMostName = (pattern) => {
  const [leftMost = ""] = pattern.split("*");

  return leftMost;
};

const isImpliedByOthers = (pattern, allPaterns) => {
  const others = allPaterns.filter((other) => other !== pattern);
  // *S is implied by *COCONUTS
  const implied = others.some((other) => {
    const rightMostOther = rightMostName(other);
    const rightMostPattern = rightMostName(pattern);
    const start = rightMostPattern.length;
    const otherSection = rightMostOther.substr(rightMostOther.length - start);
    return otherSection === rightMostPattern;
  });
  return implied;
};

const isLowOrderPattern = (pattern, allPaterns) => {
  const others = allPaterns.filter((other) => other !== pattern);
  // *S is implied by *COCONUTS
  const implied = others.some((other) => {
    const rightMostOther = rightMostName(other);
    const rightMostPattern = rightMostName(pattern);
    const start = rightMostPattern.length;
    const otherSection = rightMostOther.substr(rightMostOther.length - start);
    return otherSection === rightMostPattern && pattern.length < other.length;
  });
  return implied;
};

const impliesOthers = (pattern, allPaterns) => {
  const others = allPaterns.filter((other) => other !== pattern);
  // C* is implied by CODE*
  const implied = others.some((other) => {
    const leftMostOther = leftMostName(other);
    const leftMostPattern = leftMostName(pattern);

    return leftMostPattern.includes(leftMostOther);
  });
  return implied;
};

function patternMatch({ total, patterns }) {
  const unique = [...new Set(patterns)];
  const higherOrder = unique.filter(
    (pattern) => !isLowOrderPattern(pattern, patterns)
  );

  const notImplied = higherOrder.filter(
    (pattern) => !isImpliedByOthers(pattern, higherOrder)
  );

  if (notImplied.length > 1) {
    return PATTERN;
  }

  const [answerPattern = ""] = notImplied;
  const answer = answerPattern.replace(PATTERN, "");
  const wildCards = unique.filter(
    (pattern) => rightMostName(pattern).length === 0
  );

  if (wildCards.length) {
    const leastGeneric = wildCards.filter((wildCard) =>
      impliesOthers(wildCard, wildCards)
    );

    if (leastGeneric.length === 1) {
      const [generic] = leastGeneric;
      const use = generic.replace(PATTERN, "");
      if (use !== answer) {
        return `${use}${answer}`;
      }
      return answer;
    } else {
      return PATTERN;
    }
  }

  return answer;
}

rl.on("line", function (line) {
  //code goes here
  // if it is the first line, this will turn currentLine to zero
  caseTracker.nextLine();
  const lineNumber = caseTracker.currentLine;

  // for the first line, which specifies the number of cases
  if (!lineNumber) {
    caseTracker.setNextCase(1);
    return caseTracker.setNumberOfCases(parseInt(line));
  }
  // matrix sizes
  if (lineNumber && caseTracker.nextCase === lineNumber) {
    caseTracker.setNextCase(caseTracker.nextCase + parseInt(line) + 1);
  }

  if (lineNumber && lineNumber < caseTracker.nextCase) {
    caseTracker.addToCurrent(line);
    if (lineNumber === caseTracker.nextCase - 1) {
      caseTracker.addCase();
    }
  }
  // if this is the last line
  if (caseTracker.numberOfCases === caseTracker.cases.length) {
    return rl.close();
  }
}).on("close", function () {
  try {
    caseTracker.cases.forEach((line, index) => {
      const [total, ...patterns] = line;
      const result = patternMatch({ total: parseInt(total), patterns });
      caseTracker.addResult(`Case #${index + 1}: ${result}`);
    });
  } catch (e) {
    console.log(e);
  }

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
