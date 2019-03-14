const fs = require("fs");
const { performance } = require("perf_hooks");
const { exec, spawn } = require("child_process");

// TODO: Perhaps we could trigger a transpile process from here

const [name] = process.argv.slice(-1);
const testPath = `tests/${name}.in`;
const target = `build/${name}`;
const expectedScriptName = "index.js";

// given the file name
if (name) {
  fs.readFile(testPath, "utf-8", (err, tests) => {
    if (err) return console.log("Test file not found", err);

    // test if build/name is a thing
    fs.readdir(target, (err, files) => {
      if (err) return console.log("Did you transpile?", err);
      const [script] = files;
      if (script === expectedScriptName) {
        // setup spawn
        const startTime = performance.now();
        const child = spawn("node", [target]);
        // prepare the encoding and pipe
        child.stdin.setEncoding("utf-8");
        child.stdout.pipe(process.stdout);
        // write the actual data
        child.stdin.write(tests);
        // measure memory
        memUsage(child);
        // end process
        child.stdin.end();

        child.on("exit", code => {
          const time = (performance.now() - startTime) / 1000;
          console.log("Run Time:", time, "s");
          console.log("Exit Code:", code);
        });
      } else {
        // Exit, there was a problem with the script name
        console.log("Make sure the file script is named index.js");
      }
    });
  });
} else {
  // Exit, no name was given
  console.log("A name is required.");
}

function memUsage(child) {
  exec("ps -p" + child.pid + " -o vsize=", function(err, stdout, stderr) {
    const fail = err || stderr;
    if (fail) {
      return console.log("Failure while measuring memory");
    }
    const memory = parseInt(stdout, 10);
    return console.log("Memory:", memory, "kilobytes");
  });
}
