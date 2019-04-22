const fs = require("fs");
const { performance } = require("perf_hooks");
const { exec, spawn } = require("child_process");

// TODO: Perhaps we could trigger a transpile process from here
const [name] = process.argv.slice(2);
const testPath = `tests/${name}.in`;
const target = `${name}.rs`;
const buildCommand = `rustc -C opt-level=3 -o rust_builds/saveUniverse src/${target}`;
const runCommand = `rust_builds/${name}`;

// given the file name
if (name) {
  fs.readFile(testPath, "utf-8", (err, tests) => {
    if (err) return console.log("Test file not found", err);

    // check if build/name is a thing
    fs.readdir("src/", (err, files) => {
      if (err) return console.log("Did you save?", err);
      // check if the build has the expected script name
      if (files.includes(target)) {
        // setup spawn
        const startBuild = performance.now();
        const spawnBuild = exec(buildCommand, err => {
          if (err) return console.log("Error", err);
          const startTime = performance.now();
          const child = spawn(runCommand);
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
            const end = performance.now();
            const runTime = (end - startTime) / 1000;
            const totalTime = (end - startBuild) / 1000;
            console.log("Run Time:", runTime, "s");
            console.log("Total time, including build:", totalTime, "s");
            console.log("Exit Code:", code);
          });
        });
      } else {
        // Exit, there was a problem with the script name
        console.log(`Make sure the file script exists as ${name}.rss`);
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
