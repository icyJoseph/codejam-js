// Problem: https://codingcompetitions.withgoogle.com/codejam/round/000000000019fd27/000000000020bdf9
const readline = require("readline");
const rl = readline.createInterface(process.stdin, process.stdout);

const toInt = (x) => parseInt(x);

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
  },
};

const CAM = "C";
const JAMIE = "J";
const IMPOSSIBLE = "IMPOSSIBLE";

const initialState = {
  schedule: [],
  available: [CAM, JAMIE],
  taken: [],
};

const head = ([head]) => head;
const asc = (a, b) => head(a) - head(b);

const activityHappensAtTime = (activity, time) => {
  const [start, end] = activity;
  return time >= start && time < end;
};

const buildSchedule = (sortedActivityList) => {
  const { schedule } = sortedActivityList.reduce(
    (prev, currentActivity) => {
      const { schedule, available, taken, impossible } = prev;
      if (impossible) {
        return prev;
      }
      const [time, end, order] = currentActivity;
      // if the time has passed,
      // free the parent available
      const updatedTaken = taken.filter(({ assignee, until }) => {
        if (time >= until) {
          available.push(assignee);

          return false;
        }
        return true;
      });

      const [nextAssignee, ...nextAvailable] = available;
      const nextTaken = [
        ...updatedTaken,
        { assignee: nextAssignee, until: end },
      ];

      return {
        schedule: [...schedule, { assignee: nextAssignee, order }],
        available: nextAvailable,
        taken: nextTaken,
        impossible: false,
      };
    },
    { ...initialState }
  );

  return schedule
    .sort((a, b) => a.order - b.order)
    .map(({ assignee }) => assignee)
    .join("");
};

const gant = (activities) => {
  const sorted = activities
    .slice(0)
    .map((activity, index) => [...activity, index])
    .sort(asc);

  const allRanges = sorted.flat();
  const [low, high] = [Math.min(...allRanges), Math.max(...allRanges)];

  const overlaps = Array.from(
    { length: high - low },
    (_, i) => i + low
  ).map((time) =>
    activities.filter((activity) => activityHappensAtTime(activity, time))
  );

  const possible = overlaps.every((overlap) => overlap.length <= 2);

  if (possible) {
    return buildSchedule(sorted);
  }
  return IMPOSSIBLE;
};

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
      const [header, ...rows] = line;
      const numberOfActivities = parseInt(header);
      const activities = rows.map((row) => row.split(" ").map(toInt));
      //   console.log("---------", index);
      const result = gant(activities, numberOfActivities);

      caseTracker.addResult(`Case #${index + 1}: ${result}`);
    });
  } catch (e) {
    console.log(e);
  }

  caseTracker.results.forEach((result) => console.log(result));

  process.exit(0);
});
