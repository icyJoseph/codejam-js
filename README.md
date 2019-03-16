# CodeJam Solutions using ES6+

This repository features solutions to Google's code jam problems using ES6+.

## Motivation

The Code Jam competition only supports nodejs `4.8.2` which creates a barrier of entry for someone who's writing modern javascript on a daily basis. We are just very much used to the syntax sugar and new functionality available to us.

## How does it work?

We setup babel as transpiler to help us convert our code to our target environment, that is, `4.8.2`.

```json
{
  "presets": [
    [
      "@babel/preset-env",
      {
        "targets": {
          "node": "4.8"
        }
      }
    ]
  ]
}
```

We install these dependencies:

```json
{
  "devDependencies": {
    "@babel/cli": "^7.2.3",
    "@babel/core": "^7.2.2",
    "@babel/node": "^7.2.2",
    "@babel/preset-env": "^7.2.3"
  }
}
```

And setup our scripts:

```json
{
  "scripts": {
    "start": "babel-node",
    "transpile": "babel src -d build",
    "test": "echo \"Error: no test specified\" && exit 1"
  }
}
```

Just in case you want to use some new features to help you build test cases, you can use `babel-node` to the start script. Just run `yarn start <filename>`. Alternatively, `npx babel-node <filename>`.

We now have a project that supports babel 7+ and all of the JavaScript candy that comes with it. Furthermore, our project will rewrite our code to make sure that node `4.8.2` can run it!

## Approach

First solve the problem!

Second, write the code, in `src/<problemName>/index.js`.

Third, transpile it, `yarn transpile`. This command will actually transpile all of the files located in `src/`.

Now we have `4.8.2` friendly code, located in a newly created, `build/` folder.

You can switch to node `4.8.2` using `nvm` and give it a go.

This repo features a code runner, which is constantly being improved. The code runner, will take problem name as argument, look for a test file inside `tests` and a solution file inside `build/`.

Test files should be named `problemName.in`. I have left two, but please generate your own.

With the solution placed in build and your test, file you can run your solution:

```bash
node runner <problemName/>
```

And we can see if our solution is working as expected.

## Submission

To submit your solution, just go to CodeJam's website, navigate to the problem and select `Load File`. Then submit `build/<problemName>`, or just copy paste its contents into the CodeJam editor.
