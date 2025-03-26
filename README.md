# The Vega programming language.

**Usage**: `vega {command} {input} {flags} {args}`

```
  vega version           Display your current Vega version
  vega help              Display this message

  vega new {name}        Create a new project
  vega init              Initialize a new Vega project in the current folder
  vega add {pkg}         Add a package to the project
  vega remove {pkg}      Remove a package from the project
  vega update            Update dependencies listed in Vega.toml

  vega tokenize {file}   Output the token list
  vega parse {file}      Output the VIR
  vega eval {code}       Evaluate the code
  vega print {code}      Evaluate the code and print the result
  vega repl              Start a REPL session
  vega build             Turn code into the target output's format
  vega test              Run all the tests
  vega run               Run the main file
```

## Global flags:
```
  -n, --no-flood    Print a counter instead of flooding the console when testing
  -p, --plain       Disable coloring
  -d, --debug       Enable debug mode
  -q, --quiet       Disable extra output
  -w, --workspace   Set the workspace path
```
