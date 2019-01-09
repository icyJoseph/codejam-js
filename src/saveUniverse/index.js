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

// helper
function pipe(...fns) {
  return function folder(arg) {
    return fns.reduce((prev, fn) => fn(prev), arg);
  };
}

// functions to solve the problem
// applies damage to shield and returns new shield and current damage
function applyDamage([shield, laser]) {
  return [shield - laser, laser];
}
// amplifies damange and returns the shield and new damage
function amplifyDamage([shield, laser]) {
  return [shield, laser * 2];
}
// map string commands to functions
const instructionsMap = {
  C: amplifyDamage,
  S: applyDamage
};

const getFunctions = command => instructionsMap[command];

// swaps CS patters starting from the right
// we start from the right since those have higher power of two
function swap(commands) {
  const pivot = commands.lastIndexOf("CS");
  if (pivot === -1) {
    return commands;
  }
  return commands.slice(0, pivot).concat("SC", commands.slice(pivot + 2));
}

const initialDamage = 1;

// recusively hack the commands
// if swap returns the same and final shield is less than 0, we are DOOMED
// otherwise returns how many steps it took to hack the commands
function hackCommands(commands, shield, step = 0) {
  const robotFunctions = commands.split("").map(getFunctions);

  const [finalShield] = pipe(...robotFunctions)([shield, initialDamage]);

  if (finalShield < 0) {
    const newCommands = swap(commands);
    if (newCommands === commands) {
      return "IMPOSSIBLE";
    }
    return hackCommands(newCommands, shield, step + 1);
  }

  return step;
}

rl.on("line", function(line) {
  //code goes here
  const caseNumber = caseTracker.get();
  // for the first line, which specifies the number of cases
  if (!caseNumber) {
    return caseTracker.set(parseInt(line));
  }

  const [stringShield, commands] = line.split(" ");

  const shield = parseInt(stringShield);

  const result = hackCommands(commands, shield);

  return console.log(`case #${caseNumber}: ${result}`);
}).on("close", function() {
  process.exit(0);
});
