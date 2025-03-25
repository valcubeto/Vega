# The Vega programming language.

Usage: `vega [command] [input] [flags] [args]`

```
  vega version           Displays your current Vega version
  vega help              Displays this message
  vega new {name}        Creates a new project
  vega init              Initializes a new Vega project in the current folder
  vega tokenize {file}   Outputs the token list
  vega parse             Outputs the VIR
  vega test              Runs all the tests
  vega build             Turns code into the target output's format
  vega run               Runs the main file
  vega print {code}      Evaluates the code and prints the result
  vega repl              Starts a REPL session
  vega add {pkg}         Adds a package to the project
  vega remove {pkg}      Removes a package from the project
  vega update            Updates the project
```

## Global flags:
```
  |--------------|---------------------------------|
  | --no-flood   | Prints a counter instead of     |
  | -n           | flooding the console with the   |
  |              | same message over and over      |
  |--------------|---------------------------------|
  | --plain      | Disables ANSI escape codes,     |
  | -p           | including colors                |
  |--------------|---------------------------------|
  | --debug      | Enable debug mode               |
  | -d           |                                 |
  |--------------|---------------------------------|
  | --quiet      | Disable extra output            |
  | -q           |                                 |
  |--------------|---------------------------------|
  | --workspace  | Set the workspace path          |
  | -w           |                                 |
  |--------------|---------------------------------|
```
