# The Vega programming language.

**Usage**: `vega [command] [input] [flags] [args]`

```markdown
  vega version           Display your current Vega version
  vega help              Display this message
  vega new {name}        Create a new project
  vega init              Initialize a new Vega project in the current folder
  vega tokenize {file}   Output the token list
  vega parse {file}      Output the VIR
  vega test              Run all the tests
  vega build             Turn code into the target output's format
  vega run               Run the main file
  vega eval {code}       Evaluate the code
  vega print {code}      Evaluate the code and print the result
  vega repl              Start a REPL session
  vega add {pkg}         Add a package to the project
  vega remove {pkg}      Remove a package from the project
  vega update            Update dependencies listed in Vega.toml
```

## Global flags:
```
  -n, --no-flood    Print a counter instead of flooding the console
  -p, --plain       Disable ANSI escape codes, including colors
  -d, --debug       Enable debug mode
  -q, --quiet       Disable extra output
  -w, --workspace   Set the workspace path
```
