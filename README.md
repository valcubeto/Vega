# The Vega programming language.

**Usage**: `vega {command} {flags} {args}`

```
  vega version           Display your current Vega version
  vega help              Display this message

  vega create {name}     Create a new project
  vega init              Initialize a new Vega project in the current folder
  vega status
  vega add {pkg ...}     Add a package to the project
  vega remove {pkg ...}  Remove a package from the project
  vega update {pkg ...}  Update dependencies listed in Vega.toml

  vega tokenize {file}   Output the token list
  vega parse {file}      Output the VIR
  vega check             Check if the code compiles, without compiling
  vega build             Turn code into the target output's format
  vega test              Run all the tests
  vega run               Run the main file
  vega eval {code}       Run the code passed as argument as an expression
  vega print {code}      Run the code and print the result
  vega repl              Start a REPL session
```

## Global flags:
```
  -n, --no-flood    Print a counter instead of flooding the console
  -p, --plain       Disable coloring
  -d, --debug       Enable debug mode
  -q, --quiet       Disable extra output
  -w, --workspace   Set the workspace path
```
